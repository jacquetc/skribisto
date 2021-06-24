#ifndef SKRPLUMECREATORIMPORTER_H
#define SKRPLUMECREATORIMPORTER_H

#include <QObject>
#include <QUrl>
#include <QXmlStreamReader>
#include "skrresult.h"

class SKRPlumeCreatorImporter : public QObject {
    Q_OBJECT

public:

    explicit SKRPlumeCreatorImporter(QObject *parent = nullptr);

    Q_INVOKABLE SKRResult importPlumeCreatorProject(const QUrl& plumeFileName,
                                                    const QUrl& skribistoFileName);

signals:

private:

    //    QSqlDatabase copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
    // projectId, SKRResult &result);
    SKRResult transformParentsToFolder(int projectId);

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
                                   int                     noteId,
                                   const QXmlStreamReader& xml,
                                   const QString         & attributeName,
                                   const QStringList     & values);

    SKRResult readXMLRecursivelyAndCreatePaper(int               projectId,
                                               int               indent,
                                               QXmlStreamReader *xml,
                                               const QString   & tempDirPath,
                                               int               textFolderId,
                                               int               noteFolderId);

    QHash<int, int>m_attendanceConversionHash;
};
#endif // SKRPLUMECREATORIMPORTER_H
