#pragma once

#include "QtConcurrent/qtconcurrenttask.h"
#include "QtCore/qdebug.h"
#include "QtCore/qmetaobject.h"
#include "database/interface_database.h"
#include "database/interface_database_context.h"
#include "foreign_entity.h"
#include "persistence_global.h"
#include "result.h"
#include "tools.h"
#include <QDateTime>
#include <QMetaObject>
#include <QReadWriteLock>
#include <QRegularExpression>
#include <QSqlError>
#include <QSqlQuery>
#include <QTime>

using namespace Contracts::Database;

struct PropertyWithList
{
    Q_GADGET
  public:
    enum class ListType
    {
        Set,
        List
    };
    Q_ENUM(ListType)

    QString typeName;
    QString listTableName;
    ListType listType;
};

namespace Database
{ /**
   * @brief Template implementation of an InterfaceDatabase for a SQLite file database
   *
   * @tparam T Type of entity that the database will store
   */
template <class T> class SKR_PERSISTENCE_EXPORT InternalDatabase : public InterfaceDatabase<T>, public ForeignEntity<T>
{
  public:
    /**
     * @brief Construct a new Skrib File object
     *
     * @param context The context of the database
     */
    explicit InternalDatabase(InterfaceDatabaseContext *context);

    /**
     * @brief Copy constructor of Skrib File object
     *
     * @param other The object to be copied
     */
    InternalDatabase(const InternalDatabase &other);

    /**
     * @brief get retrieves an entity from the database with the specified UUID.
     * @param uuid The UUID of the entity to retrieve.
     * @return A Result object containing either the requested entity or an error message.
     */
    Result<T> get(const QUuid &uuid) override;

    /**
     * @brief getAll retrieves all entities of the type associated with this database.
     * @return A Result object containing either a list of entities or an error message.
     */
    Result<QList<T>> getAll() override;

    /**
     * @brief remove deletes an entity from the database.
     * @param entity The entity to delete.
     * @return A Result object containing either the deleted entity or an error message.
     */
    Result<T> remove(T &&entity) override;

    /**
     * @brief add adds an entity to the database.
     * @param entity The entity to add.
     * @return A Result object containing either the added entity or an error message.
     */
    Result<T> add(T &&entity) override;

    /**
     * @brief update updates an entity in the database.
     * @param entity The entity to update.
     * @return A Result object containing either the updated entity or an error message.
     */
    Result<T> update(T &&entity) override;

    /**
     * @brief exists checks whether an entity with the specified UUID exists in the database.
     * @param uuid The UUID of the entity to check for.
     * @return A Result object containing either a boolean indicating whether the entity exists
     * or an error message.
     */
    Result<bool> exists(const QUuid &uuid) override;

  private:
    QScopedPointer<InterfaceDatabaseContext>
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this SkribFile.
                            */

    const QHash<QString, PropertyWithList> m_listProperties = this->getEntityPropertiesWithList();

    // list properties:
    QString getListTableName(const QString &listPropertyName, PropertyWithList::ListType type);
    bool isCommonType(int typeId);
    QHash<QString, PropertyWithList> getEntityPropertiesWithList();
};

//--------------------------------------------

template <class T>
InternalDatabase<T>::InternalDatabase(InterfaceDatabaseContext *context)
    : ForeignEntity<T>(context), m_databaseContext(context)
{
}

//--------------------------------------------

template <class T>
InternalDatabase<T>::InternalDatabase(const InternalDatabase &other) : m_databaseContext(other.databaseContext())
{
}

//--------------------------------------------

template <class T> Result<T> InternalDatabase<T>::get(const QUuid &uuid)
{
    return QtConcurrent::task([this](QUuid uuid) {
               const QString &entityName = Tools<T>::getEntityClassName();
               const QStringList &properties = Tools<T>::getEntityProperties();
               QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());
               QHash<QString, QVariant> fieldWithValue;

               QString fields;
               for (const QString &property : properties)
               {
                   fields += property + ",";
               }
               fields.chop(1);

               {
                   QSqlQuery query(database);
                   QString queryStr = "SELECT " + fields + " FROM " + entityName + " WHERE uuid = :uuid";
                   query.prepare(queryStr);
                   query.bindValue(":uuid", QVariant(uuid));
                   query.exec();

                   if (query.lastError().isValid())
                   {
                       return Result<T>(
                           Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                   }

                   while (query.next())
                   {
                       for (int i = 0; i < properties.count(); i++)
                       {
                           fieldWithValue.insert(properties.at(i), query.value(i));
                       }
                   }
                   if (fieldWithValue.isEmpty())
                   {
                       return Result<T>(Error("SkribFile", Error::Critical, "sql_row_missing",
                                              "No row with uuid " + uuid.toString()));
                   }
               }

               return Tools<T>::mapToEntity(fieldWithValue);
           })
        .withArguments(uuid)
        .onThreadPool(m_databaseContext->threadPool())
        .spawn()
        .result();
}

//--------------------------------------------

template <class T> Result<QList<T>> InternalDatabase<T>::getAll()
{

    return QtConcurrent::task([this]() {
               const QString &entityName = Tools<T>::getEntityClassName();
               const QStringList &properties = Tools<T>::getEntityProperties();
               QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());
               QList<QHash<QString, QVariant>> fieldsWithValues;
               QList<T> entities;

               QString fields;
               for (const QString &property : properties)
               {
                   fields += property + ",";
               }
               fields.chop(1);

               {
                   QSqlQuery query(database);
                   QString queryStr = "SELECT " + fields + " FROM " + entityName;
                   query.prepare(queryStr);
                   query.exec();

                   if (query.lastError().isValid())
                   {
                       return Result<QList<T>>(
                           Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                   }

                   while (query.next())
                   {
                       QHash<QString, QVariant> fieldWithValue;
                       for (int i = 0; i < properties.count(); i++)
                       {
                           fieldWithValue.insert(properties.at(i), query.value(i));
                       }
                       fieldsWithValues.append(fieldWithValue);
                   }
               }

               for (const auto &fieldWithValue : fieldsWithValues)
               {
                   Result<T> entity = Tools<T>::mapToEntity(fieldWithValue);
                   if (entity.hasError())
                   {
                       return Result<QList<T>>(entity.error());
                   }
                   entities.append(entity.value());
               }

               return Result<QList<T>>(entities);
           })

        .onThreadPool(m_databaseContext->threadPool())
        .spawn()
        .result();
}

//--------------------------------------------

template <class T> Result<T> InternalDatabase<T>::remove(T &&entity)
{
    return QtConcurrent::task([this](T entity) {
               const QString &entityName = Tools<T>::getEntityClassName();
               QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());

               // Generate the SQL DELETE statement
               QString queryStr = "DELETE FROM " + entityName + " WHERE uuid = :uuid";

               {
                   QSqlQuery query(database);
                   query.prepare(queryStr);
                   query.bindValue(":uuid", entity.uuid().toString());

                   // Execute the DELETE statement with the entity UUID
                   if (!query.exec())
                   {
                       return Result<T>(
                           Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                   }

                   // Return an appropriate Result object based on the query execution result
                   if (query.numRowsAffected() == 1)
                   {
                       return Result<T>(std::forward<T>(entity));
                   }
                   else
                   {
                       return Result<T>(Error("SkribFile", Error::Critical, "sql_delete_failed",
                                              "Failed to delete row from database", entity.uuid().toString()));
                   }
               }
               return Result<T>(Error("SkribFile", Error::Fatal, "normaly_unreacheable", ""));
           })
        .withArguments(std::move(entity))
        .onThreadPool(m_databaseContext->threadPool())
        .spawn()
        .result();
}

//--------------------------------------------

template <class T> Result<T> InternalDatabase<T>::add(T &&entity)
{

    return /*WaitInEventLoop::basicWaitInEventLoop<T>(*/
        QtConcurrent::task([this](T entity) {
            // Acquire a write lock before writing to the database

            const QString &entityName = Tools<T>::getEntityClassName();
            const QStringList &properties = Tools<T>::getEntityProperties();
            QHash<QString, QVariant> fieldWithValue;

            // Get the entity properties and the corresponding values
            for (const QString &property : properties)
            {
                QVariant value = entity.property(property.toLatin1());
                fieldWithValue.insert(property, value);
            }

            // Generate the SQL INSERT statement with placeholders for the property values
            QString fields;
            QString placeholders;
            for (const QString &property : properties)
            {
                fields += property + ",";
                placeholders += ":" + property + ",";
            }
            fields.chop(1);
            placeholders.chop(1);
            QString queryStr = "INSERT INTO " + entityName + " (" + fields + ") VALUES (" + placeholders + ")";

            QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());

            {
                QSqlQuery query(database);
                query.prepare(queryStr);

                // Bind the property values to the placeholders in the SQL statement
                for (const QString &property : properties)
                {
                    QVariant value = fieldWithValue.value(property);
                    query.bindValue(":" + property, value);
                }

                // Execute the INSERT statement with the entity property values
                if (!query.exec())
                {
                    return Result<T>(
                        Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                }

                // Return an appropriate Result object based on the query execution result
                if (query.numRowsAffected() == 1)
                {
                    return Result<T>(std::forward<T>(entity));
                }
                else
                {
                    return Result<T>(
                        Error("SkribFile", Error::Critical, "sql_insert_failed", "Failed to insert row into database"));
                }
            }
            return Result<T>(Error("SkribFile", Error::Fatal, "normaly_unreacheable", ""));
        })
            .withArguments(std::move(entity))
            .onThreadPool(m_databaseContext->threadPool())
            .spawn()
            .result();
}

//--------------------------------------------

template <class T> Result<T> InternalDatabase<T>::update(T &&entity)
{
    return QtConcurrent::task([this](T entity) {
               const QString &entityName = Tools<T>::getEntityClassName();
               const QStringList &properties = Tools<T>::getEntityProperties();
               QHash<QString, QVariant> fieldWithValue;

               // Get the entity properties and the corresponding values
               for (const QString &property : properties)
               {
                   QVariant value = entity.property(property.toLatin1());
                   fieldWithValue.insert(property, value);
               }

               // Generate the SQL UPDATE statement with placeholders for the property values
               QString fields;
               for (const QString &property : properties)
               {
                   fields += property + " = :" + property + ",";
               }
               fields.chop(1);
               QString queryStr = "UPDATE " + entityName + " SET " + fields + " WHERE uuid = :uuid";

               QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());

               {

                   QSqlQuery query(database);
                   query.prepare(queryStr);

                   // Bind the property values to the placeholders in the SQL statement
                   for (const QString &property : properties)
                   {
                       QVariant value = fieldWithValue.value(property);
                       query.bindValue(":" + property, value);
                   }

                   // Bind the UUID value to the placeholder in the SQL statement
                   query.bindValue(":uuid", fieldWithValue.value("uuid"));

                   // Execute the UPDATE statement with the entity property values
                   if (!query.exec())
                   {
                       return Result<T>(
                           Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                   }

                   // Return an appropriate Result object based on the query execution result
                   if (query.numRowsAffected() == 1)
                   {
                       return Result<T>(std::forward<T>(entity));
                   }
                   else
                   {
                       return Result<T>(Error("SkribFile", Error::Critical, "sql_update_failed",
                                              "Failed to update row in database"));
                   }
               }
               return Result<T>(Error("SkribFile", Error::Fatal, "normaly_unreacheable", ""));
           })
        .withArguments(std::move(entity))
        .onThreadPool(m_databaseContext->threadPool())
        .spawn()
        .result();
}

//--------------------------------------------

template <class T> Result<bool> InternalDatabase<T>::exists(const QUuid &uuid)
{
    return QtConcurrent::task([this](QUuid uuid) {
               const QString &entityName = Tools<T>::getEntityClassName();
               QSqlDatabase database = QSqlDatabase::database(m_databaseContext->databaseName());

               {

                   QSqlQuery query(database);
                   QString queryStr = "SELECT COUNT(*) FROM " + entityName + " WHERE uuid = :uuid";
                   query.prepare(queryStr);
                   query.bindValue(":uuid", uuid.toString());
                   query.exec();

                   if (query.lastError().isValid())
                   {
                       return Result<bool>(
                           Error("SkribFile", Error::Critical, "sql_error", query.lastError().text(), queryStr));
                   }

                   if (query.next())
                   {
                       return Result<bool>(query.value(0).toBool());
                   }
                   else
                   {
                       return Result<bool>(Error("SkribFile", Error::Critical, "sql_row_missing",
                                                 "No row with uuid " + uuid.toString()));
                   }
               }
               return Result<bool>(Error("SkribFile", Error::Fatal, "normaly_unreacheable", ""));
           })
        .withArguments(uuid)
        .onThreadPool(m_databaseContext->threadPool())
        .spawn()
        .result();
}

//--------------------------------------------

// list properties

template <class T>
QString InternalDatabase<T>::getListTableName(const QString &listPropertyName, PropertyWithList::ListType type)
{
    QString tableName = Tools<T>::getEntityClassName();
    QString upperCaseListPropertyName = listPropertyName;
    upperCaseListPropertyName[0] = upperCaseListPropertyName[0].toUpper();

    switch (type)
    {
    case PropertyWithList::ListType::List:
        tableName += upperCaseListPropertyName + "List";
        break;
    case PropertyWithList::ListType::Set:
        tableName += upperCaseListPropertyName + "Set";
        break;
    }

    return tableName;
}

//--------------------------------------------

template <class T> bool InternalDatabase<T>::isCommonType(int typeId)
{
    static const QSet<int> commonTypes{
        qMetaTypeId<int>(),         qMetaTypeId<uint>(),         qMetaTypeId<long>(),         qMetaTypeId<ulong>(),
        qMetaTypeId<long long>(),   qMetaTypeId<double>(),       qMetaTypeId<float>(),        qMetaTypeId<bool>(),
        qMetaTypeId<QChar>(),       qMetaTypeId<QString>(),      qMetaTypeId<QByteArray>(),   qMetaTypeId<QDate>(),
        qMetaTypeId<QTime>(),       qMetaTypeId<QDateTime>(),    qMetaTypeId<QUrl>(),         qMetaTypeId<QUuid>(),
        qMetaTypeId<QVariantMap>(), qMetaTypeId<QVariantList>(), qMetaTypeId<QVariantHash>(),
    };
    return commonTypes.contains(typeId);
}

//--------------------------------------------

template <class T> QHash<QString, PropertyWithList> InternalDatabase<T>::getEntityPropertiesWithList()
{
    QHash<QString, PropertyWithList> propertiesWithList;
    const QMetaObject &metaObject = T::staticMetaObject;

    QRegularExpression listRegex(R"(QList<(.+)>)");
    QRegularExpression setRegex(R"(QSet<(.+)>)");

    for (int i = 0; i < metaObject.propertyCount(); ++i)
    {
        QMetaProperty property = metaObject.property(i);
        QString propertyTypeName = property.typeName();

        QRegularExpressionMatch listMatch = listRegex.match(propertyTypeName);
        QRegularExpressionMatch setMatch = setRegex.match(propertyTypeName);

        if (listMatch.hasMatch() || setMatch.hasMatch())
        {
            QString innerTypeName = listMatch.hasMatch() ? listMatch.captured(1) : setMatch.captured(1);
            QMetaType innerMetaType = QMetaType::fromName(innerTypeName.toLatin1().constData());

            if (isCommonType(innerMetaType.id()))
            {
                PropertyWithList::ListType listType =
                    listMatch.hasMatch() ? PropertyWithList::ListType::List : PropertyWithList::ListType::Set;
                QString listTableName = getListTableName(property.name(), listType);
                PropertyWithList propertyWithList{innerTypeName, listTableName, listType};
                propertiesWithList.insert(property.name(), propertyWithList);
            }
        }
    }

    return propertiesWithList;
}

} // namespace Database
