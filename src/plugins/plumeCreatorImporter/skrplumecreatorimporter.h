#ifndef SKRPLUMECREATORIMPORTER_H
#define SKRPLUMECREATORIMPORTER_H

#include <QObject>
#include <QUrl>
#include <QXmlStreamReader>
#include <QtSql/QSqlDatabase>
#include "skrresult.h"
#include "plumecreatorimporter_global.h"

class EXPORT SKRPlumeCreatorImporter : public QObject {
    Q_OBJECT

public:

    explicit SKRPlumeCreatorImporter(QObject *parent = nullptr);

    Q_INVOKABLE SKRResult importPlumeCreatorProject(const QUrl& plumeFileName,
                                                    const QUrl& skribistoFileName);

signals:

private:

    //    QSqlDatabase copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
    // projectId, SKRResult &result);

    SKRResult createPapersAndAssociations(int                     projectId,
                                          int                     indent,
                                          const QXmlStreamReader& xml,
                                          const QString         & tempDirPath,
                                          int                     textFolderId,
                                          int                     noteFolderId);
    SKRResult createNote(int            projectId,
                         int            indent,
                         int            plumeId,
                         const QString& name,
                         const QString& tempDirPath,
                         int            parentFolderId);

    SKRResult createTagsFromAttend(int                     projectId,
                                   int                     attendId,
                                   const QXmlStreamReader& xml,
                                   const QString         & attributeName,
                                   const QStringList     & values);

    QHash<int, int>m_attendanceConversionHash;
    SKRResult createSection(int            projectId,
                            int            indent,
                            int            textFolderId,
                            const QString& section_type);

    SKRResult createFolderAndAssociations(int                     projectId,
                                          int                     indent,
                                          const QXmlStreamReader& xml,
                                          const QString         & tempDirPath,
                                          int                     textFolderId,
                                          int                     noteFolderId);
    SKRResult createAttendFolder(int                     projectId,
                                 int                     indent,
                                 const QXmlStreamReader& xml,
                                 const QString         & tempDirPath,
                                 int                     attendFolderId);
    SKRResult createAttendObject(int                     projectId,
                                 int                     indent,
                                 const QXmlStreamReader& xml,
                                 const QString         & tempDirPath,
                                 int                     attendFolderId);
};
#endif // SKRPLUMECREATORIMPORTER_H
