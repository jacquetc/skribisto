#include "skrhighlighter.h"
#include <QDebug>
#include <QTextDocument>
#include <QTextBoundaryFinder>
#include "plmdata.h"

SKRHighlighter::SKRHighlighter(QTextDocument *parentDoc)
    : QSyntaxHighlighter(parentDoc), m_spellCheckerSet(false), m_projectId(-2)
{
    SKRSpellChecker *spellChecker = new SKRSpellChecker(this);

    this->setSpellChecker(spellChecker);

    connect(plmdata->projectDictHub(),
            &SKRProjectDictHub::projectDictFullyChanged,
            this,
            [this](int projectId, const QStringList& newProjectDict) {
        if (projectId == this->getProjectId()) {
            this->getSpellChecker()->setUserDict(newProjectDict);
            this->rehighlight();
        }
    }
            );

    connect(plmdata->projectDictHub(), &SKRProjectDictHub::projectDictWordAdded, this,
            [this](int projectId, const QString& newWord) {
        if (projectId == this->getProjectId()) {
            this->getSpellChecker()->addWordToUserDict(newWord);
            this->rehighlight();
        }
    }
            );

    connect(plmdata->projectDictHub(), &SKRProjectDictHub::projectDictWordRemoved, this,
            [this](int projectId, const QString& wordToBeRemoved) {
        if (projectId == this->getProjectId()) {
            this->getSpellChecker()->removeWordFromUserDict(wordToBeRemoved);
            this->rehighlight();
        }
    }
            );

    connect(spellChecker, &SKRSpellChecker::activated, this, [this](bool activated) {
        if (activated) {
            this->getSpellChecker()->setUserDict(m_userDictList);
            this->rehighlight();
        }
    }
            );
}

// -------------------------------------------------------------------


// -------------------------------------------------------------------

void SKRHighlighter::highlightBlock(const QString& text)
{
    setCurrentBlockState(0);


    if (text.length() > 7000) { // cancel the highlighting to prevent slowing.
        qWarning() << "a paragraph is too long for highlighting !";
        return;
    }


    // find  :

    QList<int> findList;

    QTextCharFormat findFormat;

    findFormat.setBackground(Qt::yellow);

    if (!textToHighLight.isEmpty()) {
        int position = 0;

        while (position >= 0) {
            int start = text.indexOf(textToHighLight, position, sensitivity);

            if (start == -1) break;

            //            setFormat(start , textToHighLight.size(), findFormat);

            // list for later merging :
            for (int i = start; i < start + textToHighLight.size();
                 ++i) findList.append(i);

            position = text.indexOf(textToHighLight,
                                    position + textToHighLight.size(),
                                    sensitivity);
        }

        setCurrentBlockState(1);
    }


    //    spell check :

    QList<int> spellcheckerList;

    QTextCharFormat spellcheckFormat;

    // BUG to be uncommented when bug
    // https://bugreports.qt.io/browse/QTBUG-87260 is fixed
    //    spellcheckFormat.setUnderlineColor(Qt::GlobalColor::red);
    //
    //  spellcheckFormat.setUnderlineStyle(QTextCharFormat::SpellCheckUnderline);
    spellcheckFormat.setForeground(QBrush(Qt::GlobalColor::red));

    if (m_spellCheckerSet)
        if (m_spellChecker->isActive()) {
            QTextBoundaryFinder wordFinder(QTextBoundaryFinder::Word, text);
            int wordStart     = 0;
            int wordLength    = 0;
            QString wordValue = "";

            while (wordFinder.position() < text.length())
            {
                if (wordFinder.position() == -1) break;

                if (wordFinder.position() == 0)
                {
                    wordStart = 0;
                }
                else
                {
                    wordStart = wordFinder.position();
                }


                wordLength = wordFinder.toNextBoundary() - wordStart;

                wordValue = text.mid(wordStart, wordLength);


                if (!m_spellChecker->spell(text.mid(wordStart, wordLength))) {
                    //                setFormat(wordStart, wordLength,
                    // spellcheckFormat);


                    if (text.mid(wordStart + wordLength, 1) == "-") { // cerf-volant,
                        // orateur-né
                        wordFinder.toNextBoundary();
                        int nextWordLength = wordFinder.toNextBoundary() -
                                             (wordStart + wordLength + 1);
                        wordFinder.toPreviousBoundary();
                        wordFinder.toPreviousBoundary();

                        QString hyphenWord = text.mid(wordStart,
                                                      wordLength + nextWordLength + 1);

                        //                    qDebug() <<
                        // "hyphenWord_Highlighter : " + hyphenWord;
                        if (!m_spellChecker->spell(text.mid(wordStart,
                                                            hyphenWord.size()))) {
                            wordLength = hyphenWord.size();
                            wordFinder.toNextBoundary();
                            wordFinder.toNextBoundary();

                            for (int i = wordStart; i < wordStart + wordLength;
                                 ++i) spellcheckerList.append(i);
                            continue;
                        }
                        else {
                            wordFinder.toNextBoundary();
                            wordFinder.toNextBoundary();
                            continue;
                        }
                    }


                    // list for later merging :
                    for (int i = wordStart; i < wordStart + wordLength;
                         ++i) spellcheckerList.append(i);
                }
            }
            setCurrentBlockState(2);
        }


    for (int k = 0; k < text.length(); ++k) {
        QTextCharFormat finalFormat;

        if (findList.contains(k)){
            finalFormat.merge(findFormat);
        }

        if (spellcheckerList.contains(k)) {
            finalFormat.merge(spellcheckFormat);
        }

        setFormat(k, 1, finalFormat);
    }

    emit shakeTextSoHighlightsTakeEffectCalled();
}

// -------------------------------------------------------------------


void SKRHighlighter::setTextToHighlight(QString string)
{
    textToHighLight = string;
    this->rehighlight();
}

// -------------------------------------------------------------------

void SKRHighlighter::setCaseSensitivity(bool isCaseSensitive)
{
    if (isCaseSensitive) {
        sensitivity = Qt::CaseSensitive;
    }
    else {
        sensitivity = Qt::CaseInsensitive;
    }
}

// -------------------------------------------------------------------

SKRSpellChecker * SKRHighlighter::getSpellChecker()
{
    if (!m_spellChecker) {
        qDebug() << "no check speller set";
    }
    return m_spellChecker;
}

// -------------------------------------------------------------------

void SKRHighlighter::setSpellChecker(SKRSpellChecker *spellChecker)
{
    if (spellChecker) {
        m_spellChecker    = spellChecker;
        m_spellCheckerSet = true;
    }
    else {
        //        qWarning() << "TextHighlighter : no spellchecker set";
        m_spellCheckerSet = false;
    }
}

int SKRHighlighter::getProjectId() const
{
    return m_projectId;
}

void SKRHighlighter::setProjectId(int projectId)
{
    m_projectId = projectId;

    // get userdict
    if (projectId != -2) {
        m_userDictList.clear();
        m_userDictList = plmdata->projectDictHub()->getProjectDictList(projectId);
    }

    // set user dict
    if (m_spellChecker && m_spellChecker->isHunspellLaunched() && (projectId != -2)) {
        m_spellChecker->setUserDict(m_userDictList);
    }


    emit projectIdChanged(projectId);
}



//--------------------------------------------------------------

