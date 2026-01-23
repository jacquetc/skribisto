#pragma once
#include <QHash>
#include <QList>
#include <QSqlDatabase>
#include <optional>

namespace Skribisto::Common::Database::JunctionTableOps
{
class OneToOne
{
  public:
    static QHash<int, std::optional<int>> getRightIdMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                         const QString &junctionTableName);

    static std::optional<int> getRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, bool> removeWithLeftIdMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                 const QString &junctionTableName);

    static bool removeWithLeftId(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db, const QHash<int, int> &leftIdToRightId,
                                                    const QString &junctionTableName);

    // for optional variant
    static QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db,
                                                    const QHash<int, std::optional<int>> &leftIdToRightId,
                                                    const QString &junctionTableName);

    static QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName, int right_id);

    // for optional
    static QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                    std::optional<int> right_id);

    static QHash<int, int> getLeftIdMany(QSqlDatabase &db, const QString &junctionTableName,
                                         const QList<int> &rightIds);

    static int getLeftId(QSqlDatabase &db, const QString &junctionTableName, int rightId);

    static int getRightIdCount(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QList<int> getRightIdInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    // Validation function to check if left_id already exists with a different right_id
    static bool validateUniqueLeftId(QSqlDatabase &db, int leftId, int rightId, const QString &junctionTableName);

    // Validation function to check if right_id already exists with a different left_id
    static bool validateUniqueRightId(QSqlDatabase &db, int leftId, int rightId, const QString &junctionTableName);
};

} // namespace Skribisto::Common::Database::JunctionTableOps
