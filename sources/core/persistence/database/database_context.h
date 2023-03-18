#pragma once

#include "database/interface_database_context.h"
#include "persistence_global.h"
#include "result.h"
#include <QReadWriteLock>
#include <QThreadPool>
#include <QUrl>

namespace Database
{

/**
 * @brief The SkribFileContext class represents the context for a Skrib file database.
 */
class SKR_PERSISTENCE_EXPORT DatabaseContext : public QObject, public Contracts::Database::InterfaceDatabaseContext
{
    Q_OBJECT
  public:
    /**
     * @brief Constructs a new SkribFileContext object.
     */
    explicit DatabaseContext();

    /**
     * @brief Destructor for the SkribFileContext object.
     */
    ~DatabaseContext();

    /**
     * @brief Initializes the Skrib file database.
     * @return A Result object with a value of nullptr if successful, or an Error object if an error occurred.
     */
    Result<void> init();

    /**
     * @brief Returns the name of the Skrib file database.
     * @return The name of the Skrib file database.
     */
    QString databaseName() const override;

    /**
     * @brief Returns the file name of the Skrib file database.
     * @return The file name of the Skrib file database.
     */
    QUrl fileName() const;

    /**
     * @brief Sets the file name of the Skrib file database.
     * @param newFileName The new file name of the Skrib file database.
     */
    void setFileName(const QUrl &newFileName);

    /**
     * @brief Returns the thread pool used by the SkribFileContext object.
     * Necesssary so the QSqlDatabase can stay on the same thread.
     * @return The thread pool used by the SkribFileContext object.
     */
    QThreadPool &threadPool() override;

    QReadWriteLock &lock();

  signals:

  private:
    QUrl m_fileName;          /**< The file name of the Skrib file database. */
    QString m_databaseName;   /**< The name of the Skrib file database. */
    QThreadPool m_threadPool; /**< The thread pool used for loading the Skrib file database. */

    /**
     * @brief Loads the Skrib file database from the given file name.
     * @param fileName The file name of the Skrib file database to load.
     * @return A Result object with the name of the loaded database if successful, or an Error object if an error
     * occurred.
     */
    Result<QString> createEmptyDatabase();
    QStringList SqlEmptyDatabaseQuery() const;
};
} // namespace Database
