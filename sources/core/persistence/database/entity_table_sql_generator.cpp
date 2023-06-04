#include "entity_table_sql_generator.h"

#include <QMetaType>

using namespace Database;

EntityTableSqlGenerator::EntityTableSqlGenerator(const QStringList &entityClassNames)
{
    m_entityClassNames = entityClassNames;
}

const char *EntityTableSqlGenerator::qtMetaTypeToSqlType(int qtMetaType)
{
    switch (qtMetaType)
    {
    case QMetaType::Bool:
        return "BOOLEAN";
    case QMetaType::Int:
        return "INTEGER";
    case QMetaType::UInt:
        return "INTEGER";
    case QMetaType::LongLong:
        return "INTEGER";
    case QMetaType::ULongLong:
        return "INTEGER";
    case QMetaType::Float:
        return "REAL";
    case QMetaType::Double:
        return "REAL";
    case QMetaType::QString:
        return "TEXT";
    case QMetaType::QUuid:
        return "TEXT";
    case QMetaType::QDateTime:
        return "DATETIME";
    default:
        return nullptr;
    }
}
