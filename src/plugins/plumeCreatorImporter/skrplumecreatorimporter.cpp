#include "skrplumecreatorimporter.h"
#include "skrdata.h"
#include <QUrl>
#include <QTemporaryDir>
#include <quazip/JlCompress.h>
#include <QTextDocument>
#include <QtSql/QSqlError>
#include <QtSql/QSqlRecord>
#include <QtSql/QSqlQuery>


SKRPlumeCreatorImporter::SKRPlumeCreatorImporter(QObject *parent) : QObject(parent)
{}

// -----------------------------------------------------------------------------------------------


SKRResult SKRPlumeCreatorImporter::importPlumeCreatorProject(const QUrl& plumeFileName, const QUrl& skribistoFileName)
{
    QString   sqlFile = ":/sql/sqlite_project_1_6.sql";
    SKRResult result  = skrdata->projectHub()->createSilentlyNewSpecificEmptyProject(skribistoFileName, sqlFile);

    int projectId = -2;

    IFOK(result) {
        projectId = result.getData("projectId", -2).toInt();
    }

    if (projectId == -2) {
        result = SKRResult(SKRResult::Critical, this, "plume_cant_create_empty_project");
        return result;
    }

    // create temp file
    QTemporaryDir tempDir;

    tempDir.setAutoRemove(true);

    if (!tempDir.isValid()) {
        result = SKRResult(SKRResult::Critical, this, "plume_no_temp_dir");
        return result;
    }

    QString tempDirPath = tempDir.path();

    // extract zip

    JlCompress::extractDir(plumeFileName.toLocalFile(), tempDirPath);

    //    QFile qf(plumeFileName.toLocalFile());
    //    qf.open(QIODevice::ReadOnly);
    //    int   fd        = qf.handle();
    //    FILE *plumeFile = fdopen(dup(fd), "rb");


    //    //    QFile tf(plumeFileName.toLocalFile());
    //    //    tf.open(QIODevice::ReadOnly);
    //    FILE *fp;
    //    QByteArray  ba     = tempDirPath.toLocal8Bit();
    //    const char *c_str2 = ba.data();
    //    fp = fopen(c_str2, "w");

    //    //    int   fd2 = tf.handle();
    //    //    FILE *t   = fdopen(dup(fd2), "rb");

    //    decompress(plumeFile, fp);
    //    fclose(plumeFile); // correct
    //    fclose(fp);

    // ----------- create text folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_folder_creation");
        return result;
    }
    int textFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, textFolderId, 1000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, textFolderId, "text"));


    // ----------- create attend folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "attend_folder_creation");
        return result;
    }
    int attendFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, attendFolderId, 80000000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, attendFolderId, "attendance"));


    // ----------- create note folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "note_creation");
        return result;
    }
    int noteFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, noteFolderId, 90000000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, noteFolderId, "note"));


    // ----------------------------- read
    // attend---------------------------------------

    QFileInfo attendFileInfo(tempDirPath + "/attendance");

    if (!attendFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_attend_file");
        return result;
    }


    QFile attendFile(attendFileInfo.absoluteFilePath());

    if (!attendFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_attend_file");
        return result;
    }


    QByteArray attendLines = attendFile.readAll();

    QXmlStreamReader attendXml(attendLines);


    m_attendanceConversionHash.clear();


    while (attendXml.readNextStartElement() && attendXml.name().toString() == "plume-attendance") {
        QStringList rolesNames  = attendXml.attributes().value("rolesNames").toString().split("--", Qt::SkipEmptyParts);
        QStringList levelsNames =
            attendXml.attributes().value("levelsNames").toString().split("--", Qt::SkipEmptyParts);

        QStringList box_1Names = attendXml.attributes().value("box_1").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_2Names = attendXml.attributes().value("box_2").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_3Names = attendXml.attributes().value("box_3").toString().split("--", Qt::SkipEmptyParts);

        QStringList spinBox_1Names =
            attendXml.attributes().value("spinBox_1").toString().split("--", Qt::SkipEmptyParts);


        while (attendXml.readNextStartElement() && attendXml.name().toString() == "group") {
            int attendNumber  = attendXml.attributes().value("number").toInt();
            QString groupName = attendXml.attributes().value("name").toString();
            result = this->createNote(projectId, 1, attendNumber, groupName, tempDirPath + "/attend/A", attendFolderId);
            IFOK(result) {
                int groupNoteId = result.getData("treeItemId", -2).toInt();

                m_attendanceConversionHash.insert(attendNumber, groupNoteId);

                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_1", box_1Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_2", box_2Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_3", box_3Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "spinBox_1", spinBox_1Names);
            }


            while (attendXml.readNextStartElement() && attendXml.name().toString() == "obj") {
                int attendNumber = attendXml.attributes().value("number").toInt();
                QString objName  = attendXml.attributes().value("name").toString();
                result =
                    this->createNote(projectId, 2, attendNumber, objName, tempDirPath + "/attend/A", attendFolderId);
                IFOK(result) {
                    int objNoteId = result.getData("treeItemId", -2).toInt();

                    m_attendanceConversionHash.insert(attendNumber, objNoteId);


                    result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_1", box_1Names);
                    result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_2", box_2Names);
                    result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_3", box_3Names);
                    result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "spinBox_1", spinBox_1Names);

                    QString objQuickDetail = attendXml.attributes().value("quickDetails").toString();

                    skrdata->treePropertyHub()->setProperty(projectId, objNoteId, "label", objQuickDetail);
                }

                attendXml.readElementText();
            }
        }
    }


    // ----------------------------- read
    // tree---------------------------------------


    QFileInfo treeFileInfo(tempDirPath + "/tree");

    if (!treeFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_tree_file");
        result.addData("filePath", treeFileInfo.absoluteFilePath());
        return result;
    }


    QFile treeFile(treeFileInfo.absoluteFilePath());

    if (!treeFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_tree_file");
        result.addData("filePath", treeFileInfo.absoluteFilePath());
        return result;
    }


    QByteArray lines = treeFile.readAll();

    QXmlStreamReader xml(lines);


    while (xml.readNextStartElement() && xml.name().toString() == "plume-tree") {
        // pick project name
        QString projectName = xml.attributes().value("projectName").toString();
        IFOKDO(result, skrdata->projectHub()->setProjectName(projectId, projectName));


        while (xml.readNextStartElement()) {
            if (xml.name().toString() == "trash") {
                xml.skipCurrentElement();
                continue;
            }

            if (xml.name().toString() == "book") {
                IFOKDO(result,
                       this->readXMLRecursivelyAndCreatePaper(projectId, 1, &xml, tempDirPath, textFolderId,
                                                              noteFolderId));
            }
        }

        while (xml.readNext() != QXmlStreamReader::EndDocument) {}
    }

    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "error_while_exploiting_tree");
        return result;
    }

    if (xml.hasError() && (xml.error() != QXmlStreamReader::PrematureEndOfDocumentError)) {
        result = SKRResult(SKRResult::Critical, this, "error_in_tree_xml");

        result.addData("xmlError", QString("%1\nLine %2, column %3")
                       .arg(xml.errorString())
                       .arg(xml.lineNumber())
                       .arg(xml.columnNumber()));

        return result;
    }


    // ----------------------------- user dict------
    // -----------------------------

    QFileInfo dictFileInfo(tempDirPath + "/dicts/userDict.dict_plume");

    if (!dictFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_dict_file");
        result.addData("filePath", dictFileInfo.absoluteFilePath());
        return result;
    }


    QFile dictFile(dictFileInfo.absoluteFilePath());

    if (!dictFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_dict_file");
        result.addData("filePath",  dictFileInfo.absoluteFilePath());
        result.addData("fileError", dictFile.error());
        return result;
    }


    QByteArray dictLines = dictFile.readAll();

    QString dictString = QString::fromUtf8(dictLines);

    QStringList dictWords = dictString.split(";", Qt::SkipEmptyParts);

    skrdata->projectDictHub()->setProjectDictList(projectId, dictWords);


    // transform texts with children in folders

    IFOKDO(result, transformParentsToFolder(projectId));


    // save

    IFOKDO(result, skrdata->projectHub()->saveProject(projectId));

    QUrl url = skrdata->projectHub()->getPath(projectId);

    IFOKDO(result, skrdata->projectHub()->closeProject(projectId));

    IFOKDO(result, skrdata->projectHub()->loadProject(url));

    return result;
}

// -----------------------------------------------------------


SKRResult SKRPlumeCreatorImporter::transformParentsToFolder(int projectId) {
    SKRResult result;
    QSqlDatabase sqlDb = plmProjectManager->project(projectId)->getSqlDb();


    // retrieve sorted id with indent list

    QList<int> treeIdList;
    QList<int> treeIndentList;
    QList<int> treeSortOrderList;
    QList<QString> typeList;
    QSqlQuery query(sqlDb);
    QString   queryStr = "SELECT l_tree_id, l_indent, l_sort_order, t_type FROM tbl_tree ORDER BY l_sort_order";


    query.prepare(queryStr);


    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, this, "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);

        return result;
    }

    while (query.next()) {
        treeIdList.append(query.value(0).toInt());
        treeIndentList.append(query.value(1).toInt());
        treeSortOrderList.append(query.value(2).toInt());
        typeList.append(query.value(2).toString());
    }

    // remove project item (the first one)
    treeIdList.takeFirst();
    treeIndentList.takeFirst();
    treeSortOrderList.takeFirst();
    typeList.takeFirst();

    // remove folders

    QList<int> indexesToRemove;

    for (int k = 0; k < typeList.count(); k++) {
        if (typeList.at(k) == "TEXT") {
            indexesToRemove.append(k);
        }
    }

    for (int m = indexesToRemove.count() - 1; m >= 0; m++) {
        treeIdList.removeAt(m);
        treeIndentList.removeAt(m);
        treeSortOrderList.removeAt(m);
        typeList.removeAt(m);
    }


    // create folders

    sqlDb.transaction();

    QString treeQueryStr = "INSERT INTO tbl_tree (t_title, l_indent, l_sort_order, t_type, dt_trashed, b_trashed)"
                           "                       VALUES ("
                           "                       (SELECT t_title FROM tbl_tree WHERE l_tree_id = :treeId),"
                           "                       (SELECT l_indent FROM tbl_tree WHERE l_tree_id = :treeId),"
                           "                       :newSortOrder,"
                           "                       'FOLDER',"
                           "                       (SELECT dt_trashed FROM tbl_tree WHERE l_tree_id = :treeId),"
                           "                       (SELECT b_trashed FROM tbl_tree WHERE l_tree_id = :treeId)"
                           ")";


    QString incrementIndentQueryStr = "UPDATE tbl_tree SET l_indent = :newIndent WHERE l_tree_id = :treeId";

    int previousIdIndent = -1;

    for (int i = treeIdList.count() - 1; i >= 0; i--) {
        int currentTreeId = treeIdList.at(i);
        int currentIndent = treeIndentList.at(i);

        if (i < treeIdList.count() - 1) {
            previousIdIndent = treeIndentList.at(i + 1);
        }


        if (previousIdIndent > currentIndent) {
            int currentSortOrder = treeSortOrderList.at(i);


            // create folder
            query.prepare(treeQueryStr);

            query.bindValue(":treeId",       currentTreeId);
            query.bindValue(":newSortOrder", currentSortOrder - 1);

            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, this, "sql_error");
                result.addData("SQLError",     query.lastError().text());
                result.addData("SQL string",   queryStr);
                result.addData("treeId",       currentTreeId);
                result.addData("newSortOrder", currentSortOrder - 1);
                sqlDb.rollback();

                return result;
            }

            int newFolderTreeId = query.lastInsertId().toInt();

            // increment current item's indent

            query.prepare(incrementIndentQueryStr);

            query.bindValue(":treeId",    currentTreeId);
            query.bindValue(":newIndent", currentIndent + 1);

            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, this, "sql_error");
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", queryStr);
                result.addData("treeId",     currentTreeId);
                result.addData("sortOrder",  currentSortOrder);
                result.addData("newIndent",  currentIndent + 1);
                sqlDb.rollback();

                return result;
            }
        }
    }

    IFOK(result) {
        sqlDb.commit();
    }


    return result;
}

// -----------------------------------------------------------

SKRResult SKRPlumeCreatorImporter::readXMLRecursivelyAndCreatePaper(int projectId,
                                                                    int indent,
                                                                    QXmlStreamReader *xml,
                                                                    const QString& tempDirPath,
                                                                    int textFolderId, int noteFolderId)
{
    SKRResult result;

    if (!xml->readNextStartElement()) {
        xml->readElementText();
        return result;
    }
    else {
        if (xml->name().toString() == "separator") {
            xml->skipCurrentElement();
        }
        else {
            IFOKDO(result,
                   this->createPapersAndAssociations(projectId, indent, *xml, tempDirPath, textFolderId, noteFolderId));
            IFOKDO(result,
                   readXMLRecursivelyAndCreatePaper(projectId, indent + 1, xml, tempDirPath, textFolderId,
                                                    noteFolderId));
        }
    }

    while (xml->readNextStartElement()) {
        if (xml->name().toString() == "separator") {
            xml->skipCurrentElement();
            continue;
        }
        IFOKDO(result,
               this->createPapersAndAssociations(projectId, indent, *xml, tempDirPath, textFolderId, noteFolderId));
        IFOKDO(result,
               readXMLRecursivelyAndCreatePaper(projectId, indent + 1, xml, tempDirPath, textFolderId, noteFolderId));
    }

    return result;
}

SKRResult SKRPlumeCreatorImporter::createPapersAndAssociations(int projectId,
                                                               int indent,
                                                               const QXmlStreamReader& xml,
                                                               const QString& tempDirPath,
                                                               int textFolderId, int noteFolderId)
{
    SKRResult result(this);

    int plumeId  = xml.attributes().value("number").toInt();
    QString name = xml.attributes().value("name").toString();

    // create sheet

    result = skrdata->treeHub()->addChildTreeItem(projectId, textFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation");
        return result;
    }
    int sheetId = result.getData("treeItemId", -2).toInt();

    int textFolderIndent = skrdata->treeHub()->getIndent(projectId, textFolderId);

    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, sheetId, textFolderIndent + indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, sheetId, name));
    IFOKDO(result, skrdata->treeHub()->setType(projectId, sheetId, "TEXT"));


    // - fetch text

    QFileInfo textFileInfo(tempDirPath + "/text/T" + QString::number(plumeId) + ".html");

    if (!textFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_text_file");
        result.addData("filePath", textFileInfo.absoluteFilePath());
        return result;
    }


    QFile textFile(textFileInfo.absoluteFilePath());

    if (!textFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_text_file");
        result.addData("filePath", textFileInfo.absoluteFilePath());
        return result;
    }

    QByteArray textLines = textFile.readAll();


    QTextDocument sheetDoc;

    sheetDoc.setHtml(QString::fromUtf8(textLines));

    IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, sheetId, sheetDoc.toHtml()));


    IFKO(result) {
        return result;
    }

    // create note


    result = this->createNote(projectId, indent, plumeId, name, tempDirPath + "/text/N", noteFolderId);
    bool ok;
    int  noteId = result.getData("treeItemId", -2).toInt(&ok);

    if (!ok) {
        result = SKRResult(SKRResult::Critical, this, "bad_conversion_to_int");
        return result;
    }


    result = skrdata->treeHub()->setTreeRelationship(projectId, noteId, sheetId);


    IFKO(result) {
        return result;
    }

    // create write synopsis


    QFileInfo synFileInfo(tempDirPath + "/text/S" + QString::number(plumeId) + ".html");

    if (!synFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_synopsis_file");
        result.addData("filePath", synFileInfo.absoluteFilePath());
        return result;
    }


    QFile synFile(synFileInfo.absoluteFilePath());

    if (!synFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_synopsis_file");
        result.addData("filePath", synFileInfo.absoluteFilePath());
        return result;
    }

    QByteArray lines = synFile.readAll();


    QTextDocument synDoc;

    synDoc.setHtml(QString::fromUtf8(lines));
    IFOKDO(result, skrdata->treeHub()->setSecondaryContent(projectId, sheetId, synDoc.toHtml()));


    // associate "attendance" notes

    QStringList attendIds = xml.attributes().value("attend").toString().split("-", Qt::SkipEmptyParts);

    for (const QString& attendIdString : qAsConst(attendIds)) {
        int attendId = attendIdString.toInt();

        if (attendId == 0) {
            continue;
        }

        int skrNoteId = m_attendanceConversionHash.value(attendId);

        // associate :
        result = skrdata->treeHub()->setTreeRelationship(projectId, skrNoteId, sheetId);
    }

    IFKO(result) {
        return result;
    }


    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRPlumeCreatorImporter::createNote(int projectId, int indent, int plumeId, const QString& name,
                                              const QString& tempDirPath, int parentFolderId)
{
    SKRResult result(this);


    result = skrdata->treeHub()->addChildTreeItem(projectId, parentFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "note_creation");
        return result;
    }
    int noteId = result.getData("treeItemId", -2).toInt();


    int parentFolderIndent = skrdata->treeHub()->getIndent(projectId, parentFolderId);

    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, noteId, parentFolderIndent + indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, noteId, name));
    IFOKDO(result, skrdata->treeHub()->setType(projectId, noteId, "TEXT"));


    // fetch text

    QFileInfo attendFileInfo(tempDirPath + QString::number(plumeId) + ".html");

    if (!attendFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_attend_file");
        result.addData("filePath", attendFileInfo.absoluteFilePath());
        return result;
    }


    QFile attendFile(attendFileInfo.absoluteFilePath());

    if (!attendFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_attend_file");
        result.addData("filePath", attendFileInfo.absoluteFilePath());
        return result;
    }

    QByteArray lines = attendFile.readAll();


    QTextDocument noteDoc;

    noteDoc.setHtml(QString::fromUtf8(lines));
    IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, noteId, noteDoc.toHtml()));


    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRPlumeCreatorImporter::createTagsFromAttend(int                     projectId,
                                                        int                     noteId,
                                                        const QXmlStreamReader& xml,
                                                        const QString         & attributeName,
                                                        const QStringList     & values)
{
    SKRResult result(this);

    if (values.isEmpty()) {
        return result; // return successfully
    }


    bool ok;
    int  index = xml.attributes().value(attributeName).toInt(&ok);

    if (!ok) {
        result = SKRResult(SKRResult::Critical, this, "conversion_to_int");
        return result;
    }

    if (values.count() <= index) {
        return result; // return successfully
    }


    IFOK(result) {
        result = skrdata->tagHub()->addTag(projectId, values.at(index));
        IFOK(result) {
            result = skrdata->tagHub()->setTagRelationship(projectId,
                                                           noteId,
                                                           result.getData("tagId", -2).toInt());
        }
    }

    return result;
}
