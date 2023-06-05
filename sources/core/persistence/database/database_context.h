#pragma once

#include "database/interface_database_context.h"
#include "persistence_global.h"
#include "result.h"
#include <QMutexLocker>
#include <QReadWriteLock>
#include <QThreadPool>
#include <QUrl>

namespace Database
{

/**
 * @brief The DatabaseContext class represents the context for a internal database.
 */
class SKR_PERSISTENCE_EXPORT DatabaseContext : public QObject, public Contracts::Database::InterfaceDatabaseContext
{
    Q_OBJECT
  public:
    /**
     * @brief Constructs a new DatabaseContext object.
     */
    explicit DatabaseContext();

    /**
     * @brief Destructor for the DatabaseContext object.
     */
    ~DatabaseContext();

    /**
     * @brief Initializes the internal database.
     * @return A Result object with a value of nullptr if successful, or an Error object if an error occurred.
     */
    Result<void> init() override;

    QSqlDatabase getConnection() override;

    using SqlEmptyDatabaseQueryFunction = std::function<QStringList()>;

    void setSqlEmptyDatabaseQueryFunction(const SqlEmptyDatabaseQueryFunction &function);
    QStringList entityClassNames() const override;
    void setEntityClassNames(const QStringList &newEntityClassNames) override;

  signals:

  private:
    QMutex mutex;
    QUrl m_fileName;        /**< The file name of the internal database. */
    QString m_databaseName; /**< The name of the internal database. */

    SqlEmptyDatabaseQueryFunction m_sqlEmptyDatabaseQueryFunction;
    QStringList m_entityClassNames;

    /**
     * @brief Loads the internal database from the given file name.
     * @param fileName The file name of the internal database to load.
     * @return A Result object with the name of the loaded database if successful, or an Error object if an error
     * occurred.
     */
    Result<QString> createEmptyDatabase();
    QStringList sqlEmptyDatabaseQuery() const;
};
} // namespace Database
