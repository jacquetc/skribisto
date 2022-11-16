#include "spellchecker.h"

#include <QFile>
#include <QTextStream>
#include <QStringList>
#include <QList>
#include <QDebug>
#include <QDir>
#include <QCoreApplication>

#ifdef Q_OS_IOS
# include "../../hunspell/src/hunspell/hunspell.hxx"
#else
#include <hunspell.hxx>
#endif // Q_OS_IOS


#include "plmutils.h"

SpellChecker::SpellChecker(QObject *parent) :
    QObject(parent), m_isActive(false), m_hunspellLaunched(false), m_encodingFix("utf8"),
    m_dictionaryPath("")
{}

SpellChecker::~SpellChecker()
{
    if (m_hunspellLaunched) delete m_hunspell;
}

void SpellChecker::setDict(const QString& dictionaryPath)
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

bool SpellChecker::isHunspellLaunched() const {
    return m_hunspellLaunched;
}

bool SpellChecker::spell(const QString& word)
{
    if (!m_hunspellLaunched) {
        return true;
    }

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

QStringList SpellChecker::suggest(const QString& word)
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
void SpellChecker::ignoreWord(const QString& word)
{
    addWordToDict(word, "");
}

void SpellChecker::addWordToDict(const QString& word, const QString& affix)
{
    if (word == "") return;

    qDebug() << "word added to user dict" << word << affix;

    if(affix.isEmpty()){

        if (m_encodingFix == "latin1") m_hunspell->add(word.toLatin1().toStdString());

        if (m_encodingFix == "utf8") m_hunspell->add(word.toUtf8().toStdString());

    }
    else{

        if (m_encodingFix == "latin1") m_hunspell->add_with_affix(word.toLatin1().toStdString(), affix.toLatin1().toStdString());

        if (m_encodingFix == "utf8") m_hunspell->add_with_affix(word.toUtf8().toStdString(), affix.toUtf8().toStdString());

    }
}

// ---------------------------------------------------------------------------------

void SpellChecker::addWordToUserDict(const QString& word, bool emitSignal)
{
    // forbid if word already in the dictionary
    if (this->spell(word)) {
        return;
    }

    QString affix = "";
    //qDebug() << "m_langCode" << m_langCode;
    if(m_langCode.contains("en")){
        affix = "M";
    }


    addWordToDict(word, affix);
    std::vector<std::string> stringVector = m_hunspell->suffix_suggest(word.toUtf8().toStdString());
    QList<std::string> stringList;
    stringList.reserve(stringVector.size());
    std::copy(stringVector.begin(), stringVector.end(), std::back_inserter(stringList));

    QStringList suffixList;
    for(const std::string &str : stringList){
       // qDebug()<< "suffix_suggest" << QString::fromStdString(str);
    }



    if (!m_userDict.contains(word)) {
        m_userDict.append(word);
    }

    if (emitSignal) {
        emit userDictChanged(m_userDict);
    }
}

// ---------------------------------------------------------------------------------

bool SpellChecker::isActive()
{
    return m_isActive;
}

// ---------------------------------------------------------------------------------

void SpellChecker::removeWordFromUserDict(const QString& word, bool emitSignal)
{
    // doesn't remove from spellchecker if it exists by default in the original
    // dict
    if (!m_langCode.isEmpty()) {
        SpellChecker tempSpellChecker(this);
        tempSpellChecker.setLangCode(m_langCode);

        if (!tempSpellChecker.spell(word)) {
            if (m_encodingFix == "latin1") m_hunspell->remove(word.toLatin1().toStdString());

            if (m_encodingFix == "utf8") m_hunspell->remove(word.toUtf8().toStdString());
        }
    }

    m_userDict.removeAll(word);


    if (emitSignal) {
        emit userDictChanged(m_userDict);
    }
}

// ---------------------------------------------------------------------------------


void SpellChecker::clearUserDict()
{
    for (const QString& word : qAsConst(m_userDict)) {
        removeWordFromUserDict(word, false);
    }
    emit userDictChanged(m_userDict);
}

// ---------------------------------------------------------------------------------
bool SpellChecker::isInUserDict(const QString& word)
{
    return m_userDict.contains(word);
}

bool SpellChecker::activate(bool value)
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

void SpellChecker::deactivate()
{
    m_isActive = false;
    emit activated(false);
}

QStringList SpellChecker::dictsPaths()
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


QMap<QString, QString>SpellChecker::dictAndPathMap()
{
    QMap<QString, QString> map;

    QDir dir;
    QStringList filters;

    filters << "*.dic";

    for (const QString& path : SpellChecker::dictsPaths()) {
        dir.setPath(path);


        for (QString dict : dir.entryList(filters, QDir::Files)) {
            dict.chop(4);

            QString possiblePath = path + "/" + dict + ".aff";

            if (QFile(possiblePath).exists()) map.insert(dict, path + "/" + dict);
        }
    }


    return map;
}

// --------------------------------------------------------------------

QStringList SpellChecker::dictList()
{
    return SpellChecker::dictAndPathMap().keys();
}

// --------------------------------------------------------------------

QStringList SpellChecker::dictPathList()
{
    return SpellChecker::dictAndPathMap().values();
}

QString SpellChecker::testHunspellForEncoding()
{
    QString encoding(m_hunspell->get_dic_encoding());

    // qWarning() << "_hunspell->get_dic_encoding() : " + encoding;


    if (encoding == "ISO8859-1") return "latin1";
    else return "utf8";

    return "utf8";
}

// --------------------------------------------------------------------

void SpellChecker::setLangCode(const QString& newLangCode)
{
    if (m_langCode == newLangCode) {
        return;
    }

    QString dictPath = SpellChecker::dictAndPathMap().value(newLangCode, "");

    if (dictPath == "") { // means default was used
        dictPath = SpellChecker::dictAndPathMap().value("en_US", "");
        qWarning() << QString("Dict %1 not found, using en_US").arg(newLangCode);
    }
    this->setDict(dictPath);
    m_langCode = newLangCode;


    emit langCodeChanged(newLangCode);
}

// --------------------------------------------------------------------

QString SpellChecker::getLangCode() const
{
    return m_langCode;
}

// --------------------------------------------------------------------

void SpellChecker::setUserDict(const QStringList& userDict)
{
    m_userDict = userDict;

    for (const QString& name : m_userDict) {
        addWordToUserDict(name, false);
    }

    emit userDictChanged(m_userDict);
}

// --------------------------------------------------------------------

QStringList SpellChecker::getUserDict() const
{
    return m_userDict;
}
