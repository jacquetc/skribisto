#include "skrprojectdicthub.h"
#include "tasks/plmsqlqueries.h"
#include "tasks/plmprojectmanager.h"
#include "tools.h"
#include <QDebug>

SKRProjectDictHub::SKRProjectDictHub(QObject *parent) : QObject(parent), m_tableName(
        "tbl_project_dict")
{}

// --------------------------------------------------------------------


PLMError SKRProjectDictHub::getError()
{
    return m_error;
}

// --------------------------------------------------------------------


QStringList SKRProjectDictHub::getProjectDictList(int projectId) const
{
    PLMError error;
    QHash<int, QVariant> result;
    QStringList final;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("t_word", result);
    IFOK(error) {
        final = HashIntQVariantConverter::convertToIntQString(result).values();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return final;
}

// --------------------------------------------------------------------


PLMError SKRProjectDictHub::setProjectDictList(int                projectId,
                                               const QStringList& projectDictList)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();


    int newId;
    QHash<QString, QVariant> values;

    for (const QString& word : projectDictList) {
        values.insert("t_word", word);

        error = queries.add(values, newId);
        error.addData("wordId", newId);

        IFKO(error) {
            break;
        }
    }
    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
        emit projectDictFullyChanged(projectId, projectDictList);
    }
    IFKO(error) {
        emit errorSent(error);
    }

    return error;
}

// --------------------------------------------------------------------


PLMError SKRProjectDictHub::addWordToProjectDict(int projectId, const QString& newWord)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);
    int newId;
    QHash<QString, QVariant> values;

    values.insert("t_word", newWord);

    queries.beginTransaction();
    error = queries.add(values, newId);
    error.addData("wordId", newId);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
        emit projectDictWordAdded(projectId, newWord);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

// --------------------------------------------------------------------


PLMError SKRProjectDictHub::removeWordFromProjectDict(int            projectId,
                                                      const QString& wordtoRemove)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);

    // find id :
    QHash<int, QVariant> result;
    QString where      = "t_word";
    QString whereValue = wordtoRemove;
    int     wordId     = -1;

    error = queries.getValueByIds("t_word", result, where, wordtoRemove);

    IFOK(error) {
        if (result.isEmpty()) {
            error.setSuccess(false);
            error.setErrorCode("E_PROJECTDICT_word_missing_from_dict");
        }
    }
    IFOK(error) {
        wordId = result.key(wordtoRemove);
    }

    IFOK(error) {
        queries.beginTransaction();
        error = queries.remove(wordId);
    }
    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
        emit projectDictWordRemoved(projectId, wordtoRemove);
    }
    IFKO(error) {
        emit errorSent(error);
    }

    return error;
}

// --------------------------------------------------------------------

void SKRProjectDictHub::setError(const PLMError& error)
{
    m_error = error;
}
