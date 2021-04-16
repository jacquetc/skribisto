#include "skrspellchecker.h"

#include <QFile>
#include <QTextStream>
#include <QStringList>
#include <QDebug>
#include <QDir>
#include <QCoreApplication>

//#ifdef Q_OS_WIN
//# include "externals/hunspell/hunspell.hxx"
//#endif // Q_OS_WIN

#include <hunspell.hxx>
#include "plmutils.h"

SKRSpellChecker::SKRSpellChecker(QObject *parent) :
    QObject(parent), m_isActive(false), m_hunspellLaunched(false), m_encodingFix("utf8"),
    m_dictionaryPath("")
{}

SKRSpellChecker::~SKRSpellChecker()
{
    if (m_hunspellLaunched) delete m_hunspell;
}

void SKRSpellChecker::setDict(const QString& dictionaryPath)
{
    if (m_hunspellLaunched) delete m_hunspell;

    m_dictionaryPath = dictionaryPath;

    QString dictFile  = dictionaryPath + ".dic";
    QString affixFile = dictionaryPath + ".aff";

    QFile testFile(dictFile);

    if (!testFile.exists()) this->deactivate();


    QByteArray dictFilePathBA  = dictFile.toUtf8();
    QByteArray affixFilePathBA = affixFile.toUtf8();

    m_hunspell = new Hunspell(affixFilePathBA.constData(), dictFilePathBA.constData());


    m_encodingFix = this->testHunspellForEncoding();

    m_hunspellLaunched = true;

    // test :


    //    qWarning() << "encodingFix : " + encodingFix;


    //    m_isActive = true;
}
bool SKRSpellChecker::isHunspellLaunched() const{
    return m_hunspellLaunched;
}

bool SKRSpellChecker::spell(const QString& word)
{
    //    qWarning() << "word  : " << word;
    bool f_ignore_numbers   = false;
    bool f_ignore_uppercase = true;


    int  index        = -1;
    int  length       = 0;
    int  chars        = 1;
    bool is_number    = false;
    bool is_uppercase = f_ignore_uppercase;
    bool is_word      = false;


    int count = word.length() - 1;

    for (int i = 0; i <= count; ++i) {
        QChar c = word.at(i);

        switch (c.category()) {
        case QChar::Separator_Space:
            return 2;

        case QChar::Number_DecimalDigit:
        case QChar::Number_Letter:
        case QChar::Number_Other:
            is_number = f_ignore_numbers;
            goto Letter;

        case QChar::Letter_Lowercase:
            is_uppercase = false;

Letter:
        case QChar::Letter_Uppercase:
        case QChar::Letter_Titlecase:
        case QChar::Letter_Modifier:
        case QChar::Letter_Other:
        case QChar::Mark_NonSpacing:
        case QChar::Mark_SpacingCombining:
        case QChar::Mark_Enclosing:

            if (index == -1) {
                index  = i;
                chars  = 1;
                length = 0;
            }
            length += chars;
            chars   = 1;
            break;


        case QChar::Punctuation_Other:

            if ((c == QChar(0x0027)) || (c == QChar(0x2019))) {
                ++chars;
                break;
            }

        default:

            if (index != -1) {
                is_word = true;
            }
            break;
        }

        if (is_word || ((i == count) && (index != -1))) {
            if (!is_uppercase && !is_number) {
                if (m_encodingFix == "latin1") return m_hunspell->spell(
                        word.toLatin1().toStdString());

                if (m_encodingFix == "utf8") return m_hunspell->spell(
                        word.toUtf8().toStdString());
            }
            index        = -1;
            is_word      = false;
            is_number    = false;
            is_uppercase = f_ignore_uppercase;
        }
    }
    return true;
}

QStringList SKRSpellChecker::suggest(const QString& word)
{
    if (!m_isActive || !m_hunspell) return QStringList();


    std::vector<std::string> suggestionsVector;

    if (m_encodingFix == "latin1") suggestionsVector = m_hunspell->suggest(
            word.toLatin1().toStdString());

    if (m_encodingFix == "utf8") suggestionsVector = m_hunspell->suggest(
            word.toUtf8().toStdString());


    QVector<std::string> suggestionsQVect =  QVector<std::string>(
        suggestionsVector.begin(),
        suggestionsVector.end());

    // maybe differenciate between latin1 and utf8
    QStringList suggestions;

    for (const std::string& string: qAsConst(suggestionsQVect)) {
        suggestions.append(QString::fromStdString(string));
    }


    return suggestions;
}

// NOTE maybe useless
void SKRSpellChecker::ignoreWord(const QString& word)
{
    addWordToDict(word);
}

void SKRSpellChecker::addWordToDict(const QString& word)
{
    if (word == "") return;

    if (m_encodingFix == "latin1") m_hunspell->add(word.toLatin1().toStdString());

    if (m_encodingFix == "utf8") m_hunspell->add(word.toUtf8().toStdString());
}

// ---------------------------------------------------------------------------------

void SKRSpellChecker::addWordToUserDict(const QString& word, bool emitSignal)
{
    addWordToDict(word);

    if (!m_userDict.contains(word)) {
        m_userDict.append(word);
    }

    if (emitSignal) {
        emit userDictChanged(m_userDict);
    }
}

// ---------------------------------------------------------------------------------

bool SKRSpellChecker::isActive()
{
    return m_isActive;
}

// ---------------------------------------------------------------------------------

void SKRSpellChecker::removeWordFromUserDict(const QString& word, bool emitSignal)
{
    if (m_encodingFix == "latin1") m_hunspell->remove(word.toLatin1().toStdString());

    if (m_encodingFix == "utf8") m_hunspell->remove(word.toUtf8().toStdString());

    m_userDict.removeAll(word);


    if (emitSignal) {
        emit userDictChanged(m_userDict);
    }
}

// ---------------------------------------------------------------------------------


void SKRSpellChecker::clearUserDict()
{
    for (const QString& word : m_userDict) {
        removeWordFromUserDict(word, false);
    }
    emit userDictChanged(m_userDict);
}

// ---------------------------------------------------------------------------------
bool SKRSpellChecker::isInUserDict(const QString& word)
{
    return m_userDict.contains(word);
}

bool SKRSpellChecker::activate(bool value)
{
    if (!value) {
        deactivate();
        return false;
    }

    if (m_dictionaryPath.isEmpty()) {
        m_isActive = false;
        emit activated(false);
        return false;
    }

    m_isActive = true;

    emit activated(true);

    return true;
}

void SKRSpellChecker::deactivate()
{
    m_isActive = false;
    emit activated(false);
}

QStringList SKRSpellChecker::dictsPaths()
{
    QStringList list;

    QDir dir;

#ifdef Q_OS_LINUX

    dir.setPath("/usr/share/hunspell/");
    list.append(dir.path());

    dir.setPath("/usr/share/myspell/");
    list.append(dir.path());

#endif // ifdef Q_OS_LINUX
#ifdef Q_OS_MAC
    dir.setPath("/Library/Spelling/");

    if (dir.isReadable()) {
        list.append(dir.path());
    }
#endif // ifdef Q_OS_MAC

    QStringList addonsList = PLMUtils::Dir::addonsPathsList();

    for (const QString& string : qAsConst(addonsList)) {
        list.append(string + "/dicts/");
    }


    return list;
}

// --------------------------------------------------------------------


QMap<QString, QString>SKRSpellChecker::dictAndPathMap()
{
    QMap<QString, QString> map;

    QDir dir;
    QStringList filters;

    filters << "*.dic";

    for (const QString& path : SKRSpellChecker::dictsPaths()) {
        dir.setPath(path);


        for (QString dict : dir.entryList(filters, QDir::Files)) {
            dict.chop(4);

            if (QFile(path + "/" + dict + ".aff").exists()) map.insert(path + "/" + dict,
                                                                       dict);
        }
    }


    return map;
}

// --------------------------------------------------------------------

QStringList SKRSpellChecker::dictList()
{
    return SKRSpellChecker::dictAndPathMap().values();
}

// --------------------------------------------------------------------

QStringList SKRSpellChecker::dictPathList()
{
    return SKRSpellChecker::dictAndPathMap().keys();
}

QString SKRSpellChecker::testHunspellForEncoding()
{
    QString encoding(m_hunspell->get_dic_encoding());

    // qWarning() << "_hunspell->get_dic_encoding() : " + encoding;


    if (encoding == "ISO8859-1") return "latin1";
    else return "utf8";

    return "utf8";
}

// --------------------------------------------------------------------

void SKRSpellChecker::setLangCode(const QString& newLangCode)
{
    if (m_langCode == newLangCode) {
        return;
    }

    QString dictPath = SKRSpellChecker::dictAndPathMap().key(newLangCode, "en_US");

    if (newLangCode != "en_US") { // means default was used
        qWarning() << QString("Dict %1 not found, using en_US").arg(newLangCode);
    }
    this->setDict(dictPath);


    emit langCodeChanged(newLangCode);
}

// --------------------------------------------------------------------

QString SKRSpellChecker::getLangCode() const
{
    return m_langCode;
}

// --------------------------------------------------------------------

void SKRSpellChecker::setUserDict(const QStringList& userDict)
{
    m_userDict = userDict;

    for (const QString& name : m_userDict) {
        addWordToUserDict(name, false);
    }

    emit userDictChanged(m_userDict);
}

// --------------------------------------------------------------------

QStringList SKRSpellChecker::getUserDict() const
{
    return m_userDict;
}
