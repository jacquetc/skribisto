#include "highlighter.h"
#include <QDebug>
#include <QTextBoundaryFinder>

Highlighter::Highlighter(QTextDocument *parentDoc)
    : QSyntaxHighlighter(parentDoc), spellCheckerSet(false)
{
    this->setSpellChecker(new SpellChecker(this));

    // for test :
//    QString dictPath = SpellChecker::dictsList().key("fr_FR");
//    m_spellChecker->setDict(dictPath, QStringList(), QStringList() );

//    m_spellChecker->activate();

}

//-------------------------------------------------------------------


//-------------------------------------------------------------------

void Highlighter::highlightBlock(const QString &text)
{

    setCurrentBlockState(0);


    if(text.length() > 7000){ //cancel the highlighting to prevent slowing.
        qWarning() << "a paragraph is too long for highlighting !";
        return;
    }



    // find  :

    QList<int> findList;

    QTextCharFormat findFormat;
    findFormat.setBackground(Qt::yellow);

    if(!textToHighLight.isEmpty()){




        int position = 0;
        while(position >= 0){

            int start = text.indexOf(textToHighLight, position, sensitivity);

            if(start == -1)
                break;

            //            setFormat(start , textToHighLight.size(), findFormat);

            // list for later merging :
            for(int i = start; i < start + textToHighLight.size(); ++i)
                findList.append(i);

            position = text.indexOf(textToHighLight, position + textToHighLight.size(), sensitivity) ;


        }

        setCurrentBlockState(1);
    }


    //    spell check :

    QList<int> spellcheckerList;

    QTextCharFormat spellcheckFormat;
    spellcheckFormat.setUnderlineColor(Qt::GlobalColor::red);
    spellcheckFormat.setUnderlineStyle(QTextCharFormat::SpellCheckUnderline);
    spellcheckFormat.setFontUnderline(false);

    if(spellCheckerSet)
        if(m_spellChecker->isActive()){





            QTextBoundaryFinder wordFinder(QTextBoundaryFinder::Word,text);
            int wordStart = 0;
            int wordLength = 0;
            QString wordValue = "";
            while (wordFinder.position() < text.length())
            {
                if(wordFinder.position() == -1)
                    break;

                if (wordFinder.position()==0)
                {
                    wordStart=0;
                }
                else
                {
                    wordStart=wordFinder.position();
                }


                wordLength=wordFinder.toNextBoundary()-wordStart;

                wordValue=text.mid(wordStart,wordLength);



                if(!m_spellChecker->spell(text.mid(wordStart,wordLength))){
                    //                setFormat(wordStart, wordLength, spellcheckFormat);


                    if(text.mid(wordStart + wordLength, 1) == "-"){ // cerf-volant, orateur-né
                        wordFinder.toNextBoundary();
                        int nextWordLength = wordFinder.toNextBoundary()- (wordStart + wordLength +1);
                        wordFinder.toPreviousBoundary();
                        wordFinder.toPreviousBoundary();

                        QString hyphenWord = text.mid(wordStart, wordLength + nextWordLength + 1);
                        //                    qDebug() << "hyphenWord_Highlighter : " + hyphenWord;
                        if(!m_spellChecker->spell(text.mid(wordStart, hyphenWord.size()))){
                            wordLength = hyphenWord.size();
                            wordFinder.toNextBoundary();
                            wordFinder.toNextBoundary();

                            for(int i = wordStart; i < wordStart + wordLength; ++i)
                                spellcheckerList.append(i);
                            continue;

                        }
                        else {

                            wordFinder.toNextBoundary();
                            wordFinder.toNextBoundary();
                            continue;
                        }
                    }




                    // list for later merging :
                    for(int i = wordStart; i < wordStart + wordLength; ++i)
                        spellcheckerList.append(i);

                }



            }
            setCurrentBlockState(2);

        }


    for(int k = 0; k < text.length(); ++k){
        QTextCharFormat finalFormat;

        if(findList.contains(k))
            finalFormat.merge(findFormat);
        if(spellcheckerList.contains(k))
            finalFormat.merge(spellcheckFormat);

        setFormat(k, 1, finalFormat);

    }


}
//-------------------------------------------------------------------


void Highlighter::setTextToHighlight(QString string)
{
    textToHighLight = string;
    this->rehighlight();
}

//-------------------------------------------------------------------

void Highlighter::setCaseSensitivity(bool isCaseSensitive)
{
    if(isCaseSensitive){
        sensitivity = Qt::CaseSensitive;
    }
    else{

        sensitivity = Qt::CaseInsensitive;
    }
}

//-------------------------------------------------------------------

void Highlighter::setSpellChecker(SpellChecker *spellChecker)
{
    if(spellChecker){
        m_spellChecker = spellChecker;
        spellCheckerSet = true;
    }
    else{
//        qWarning() << "TextHighlighter : no spellchecker set";
        spellCheckerSet = false;

    }
}
