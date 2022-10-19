#include "skrprojectdicthub.h"
#include "sql/plmsqlqueries.h"
#include "tools.h"
#include <QDebug>

SKRProjectDictHub::SKRProjectDictHub(QObject *parent) : QObject(parent), m_tableName(
        "tbl_project_dict")
{}

// --------------------------------------------------------------------


SKRResult SKRProjectDictHub::getError()
{
    return m_error;
}

// --------------------------------------------------------------------


//QStringList SKRProjectDictHub::getProjectDynamicDictList(int projectId) const
//{
//    SKRResult result(this);
//    QHash<int, QVariant> hash;
//    QStringList final;

//    PLMSqlQueries queries(projectId, "tbl_tree");

//    result = queries.getValueByIds("t_title", hash);
//    IFOK(result) {
//        final = HashIntQVariantConverter::convertToIntQString(hash).values();

//        QList<QString>::iterator i = final.begin();

//        while (i != final.end()) {
//            if ((*i).contains(" ")) {
//                QStringList splitTitle = (*i).split(" ", Qt::SkipEmptyParts);
//                *i = splitTitle.takeFirst();

//                for (const QString& titlePart : qAsConst(splitTitle)) {
//                    final.append(titlePart);
//                }
//            }
//            ++i;
//        }


//        // sort alphabetically
//        final.sort(Qt::CaseInsensitive);
//    }

//    IFKO(result) {
//        emit errorSent(result);
//    }
//    return final;
//}

// --------------------------------------------------------------------


QStringList SKRProjectDictHub::getProjectDictList(int projectId) const
{
    SKRResult result(this);
    QHash<int, QVariant> hash;
    QStringList final;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("t_word", hash);
    IFOK(result) {
        final = HashIntQVariantConverter::convertToIntQString(hash).values();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return final;
}

// --------------------------------------------------------------------


SKRResult SKRProjectDictHub::setProjectDictList(int                projectId,
                                                const QStringList& projectDictList)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();


    int newId;
    QHash<QString, QVariant> values;

    for (const QString& word : projectDictList) {
        values.insert("t_word", word);

        result = queries.add(values, newId);
        result.addData("wordId", newId);

        IFKO(result) {
            break;
        }
    }
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        emit projectDictFullyChanged(projectId, projectDictList);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------


SKRResult SKRProjectDictHub::addWordToProjectDict(int projectId, const QString& newWord)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);
    int newId;
    QHash<QString, QVariant> values;

    values.insert("t_word", newWord);

    queries.beginTransaction();
    result = queries.add(values, newId);
    result.addData("wordId", newId);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        emit projectDictWordAdded(projectId, newWord);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// --------------------------------------------------------------------


SKRResult SKRProjectDictHub::removeWordFromProjectDict(int            projectId,
                                                       const QString& wordToRemove)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    // find id :
    QHash<int, QVariant> hash;
    QString where      = "t_word";
    QString whereValue = wordToRemove;
    int     wordId     = -1;

    result = queries.getValueByIds("t_word", hash, where, wordToRemove);

    IFOK(result) {
        if (hash.isEmpty()) {
            result = SKRResult(SKRResult::Critical, this, "word_missing_from_dict");
        }
    }
    IFOK(result) {
        wordId = hash.key(wordToRemove);
    }

    IFOK(result) {
        queries.beginTransaction();
        result = queries.remove(wordId);
    }
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        emit projectDictWordRemoved(projectId, wordToRemove);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------

void SKRProjectDictHub::setError(const SKRResult& result)
{
    m_error = result;
}
