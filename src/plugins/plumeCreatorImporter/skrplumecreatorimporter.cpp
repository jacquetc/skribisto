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
    QString   sqlFile = ":/sql/sqlite_project_1_8.sql";
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

    JlCompress::extractDir(plumeFileName.path(), tempDirPath);


    // ----------- create text folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_folder_creation");
        return result;
    }
    int textFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, textFolderId, 1000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, textFolderId, tr("Text")));


    // ----------- create attend folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "attend_folder_creation");
        return result;
    }
    int attendFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, attendFolderId, 80000000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, attendFolderId, tr("Attendance")));


    // ----------- create note folder---------------------------------------


    result = skrdata->treeHub()->addChildTreeItem(projectId, 0, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "note_creation");
        return result;
    }
    int noteFolderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setSortOrder(projectId, noteFolderId, 90000000));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, noteFolderId, tr("Notes")));


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
    QStringList rolesNames;
    QStringList levelsNames;
    QStringList box_1Names;
    QStringList box_2Names;
    QStringList box_3Names;
    QStringList spinBox_1Names;

    while (attendXml.readNext() != QXmlStreamReader::EndDocument) {
        qDebug() << "xml.name().toString()" << attendXml.name().toString() <<
            attendXml.attributes().value("name").toString();

        if ((attendXml.tokenType() == QXmlStreamReader::StartElement) &&
            (attendXml.name().toString() == "plume-attendance")) {
            rolesNames = attendXml.attributes().value("rolesNames").toString().split("--",
                                                                                     Qt::SkipEmptyParts);
            levelsNames =
                attendXml.attributes().value("levelsNames").toString().split("--", Qt::SkipEmptyParts);
            box_1Names     = attendXml.attributes().value("box_1").toString().split("--", Qt::SkipEmptyParts);
            box_2Names     = attendXml.attributes().value("box_2").toString().split("--", Qt::SkipEmptyParts);
            box_3Names     = attendXml.attributes().value("box_3").toString().split("--", Qt::SkipEmptyParts);
            spinBox_1Names =
                attendXml.attributes().value("spinBox_1").toString().split("--", Qt::SkipEmptyParts);
        }

        if ((attendXml.tokenType() == QXmlStreamReader::StartElement) && (attendXml.name() == QString("group"))) {
            result = this->createAttendFolder(projectId, 2,
                                              attendXml,
                                              tempDirPath,
                                              attendFolderId);

            IFOK(result) {
                int groupNoteId = result.getData("treeItemId", -2).toInt();

                int attendNumber = attendXml.attributes().value("number").toInt();

                m_attendanceConversionHash.insert(attendNumber, groupNoteId);

                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "rolesNames", rolesNames);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "levelsNames", rolesNames);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_1", box_1Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_2", box_2Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_3", box_3Names);
                result = this->createTagsFromAttend(projectId, groupNoteId, attendXml, "spinBox_1", spinBox_1Names);
            }
        }


        if ((attendXml.tokenType() == QXmlStreamReader::StartElement) && (attendXml.name() == QString("obj"))) {
            result = this->createAttendObject(projectId, 3,
                                              attendXml,
                                              tempDirPath,
                                              attendFolderId);

            IFOK(result) {
                int objNoteId = result.getData("treeItemId", -2).toInt();

                int attendNumber = attendXml.attributes().value("number").toInt();

                m_attendanceConversionHash.insert(attendNumber, objNoteId);


                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "rolesNames", rolesNames);
                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "levelsNames", rolesNames);
                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_1", box_1Names);
                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_2", box_2Names);
                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_3", box_3Names);
                result = this->createTagsFromAttend(projectId, objNoteId, attendXml, "spinBox_1", spinBox_1Names);


                QString objQuickDetail = attendXml.attributes().value("quickDetails").toString();
                skrdata->treePropertyHub()->setProperty(projectId, objNoteId, "label", objQuickDetail, true, true,
                                                        false);
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

    bool isInAct     = false;
    bool isInChapter = false;

    while (xml.readNext() != QXmlStreamReader::EndDocument) {
        qDebug() << "xml.name().toString()" << xml.name().toString() <<
            xml.attributes().value("name").toString();

        int indent = 0;

        if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("plume-tree"))) {
            skrdata->projectHub()->setProjectName(projectId, xml.attributes().value("projectName").toString());
        }

        if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("book"))) {
            indent = 2;
            result = this->createFolderAndAssociations(projectId, indent, xml, tempDirPath, textFolderId, noteFolderId);


            result = this->createSection(projectId, indent + 1, textFolderId, "book-beginning");
        }
        else if ((xml.tokenType() == QXmlStreamReader::EndElement) && (xml.name().toString() == QString("book"))) {
            indent = 3;
            result = this->createSection(projectId, indent, textFolderId, "book-end");
        }
        else if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("act"))) {
            isInAct = true;
            indent  = 3;
            this->createFolderAndAssociations(projectId, indent, xml, tempDirPath, textFolderId, noteFolderId);
        }
        else if ((xml.tokenType() == QXmlStreamReader::EndElement) && (xml.name() == QString("act"))) {
            isInAct = false;
        }
        else if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("chapter"))) {
            isInChapter = true;
            indent      = 3 + (isInAct ? 1 : 0);
            this->createFolderAndAssociations(projectId, indent, xml, tempDirPath, textFolderId, noteFolderId);

            result = this->createSection(projectId, indent + 1, textFolderId, "chapter");
        }
        else if ((xml.tokenType() == QXmlStreamReader::EndElement) && (xml.name() == QString("chapter"))) {
            isInChapter = false;
        }
        else if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("scene"))) {
            indent = 3 + (isInAct ? 1 : 0) + (isInChapter ? 1 : 0);
            this->createPapersAndAssociations(projectId, indent, xml, tempDirPath, textFolderId, noteFolderId);
        }
        else if ((xml.tokenType() == QXmlStreamReader::StartElement) && (xml.name() == QString("separator"))) {
            indent = 3 + (isInAct ? 1 : 0) + (isInChapter ? 1 : 0);

            result = this->createSection(projectId, indent, textFolderId, "separator");
        }


        //            while (xml.readNextStartElement()) {
        //                if (xml.name().toString() == "trash") {
        //                    xml.skipCurrentElement();
        //                    continue;
        //                }
        //                qDebug() << "xml.name().toString()" <<
        // xml.name().toString() <<
        //                            xml.attributes().value("name").toString();

        //                if (xml.name().toString() == "book") {
        //                    IFOKDO(result,
        //
        //
        //
        //                   this->readXMLRecursivelyAndCreatePaper(projectId,
        // 1, &xml, tempDirPath, textFolderId,
        //
        //
        //
        //
        //                                                       noteFolderId));
        //                }
        //            }
    }

    //    }

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

    // IFOKDO(result, transformParentsToFolder(projectId));


    // save

    IFOKDO(result, skrdata->projectHub()->saveProject(projectId));

    QUrl url = skrdata->projectHub()->getPath(projectId);

    IFOKDO(result, skrdata->projectHub()->closeProject(projectId));

    IFOKDO(result, skrdata->projectHub()->loadProject(url));

    return result;
}

// -----------------------------------------------------------
SKRResult SKRPlumeCreatorImporter::createSection(int            projectId,
                                                 int            indent,
                                                 int            textFolderId,
                                                 const QString& section_type) {
    SKRResult result(this);

    result = skrdata->treeHub()->addChildTreeItem(projectId, textFolderId, "SECTION");
    int newItemId = result.getData("treeItemId", -2).toInt();

    if (newItemId != -2) {
        IFOKDO(result, skrdata->treeHub()->setIndent(projectId, newItemId, indent));
        result = skrdata->treePropertyHub()->setProperty(projectId,
                                                         newItemId,
                                                         "section_type",
                                                         section_type,
                                                         false);
    }

    return result;
}

// ---------------------------------------------------

SKRResult SKRPlumeCreatorImporter::createFolderAndAssociations(int projectId,
                                                               int indent,
                                                               const QXmlStreamReader& xml,
                                                               const QString& tempDirPath,
                                                               int textFolderId, int noteFolderId)
{
    SKRResult result(this);

    int plumeId       = xml.attributes().value("number").toInt();
    QString name      = xml.attributes().value("name").toString();
    bool    isTrashed = xml.attributes().value("isTrashed").toString() == "yes";
    QString badge     = xml.attributes().value("badge").toString();

    // create folder

    result = skrdata->treeHub()->addChildTreeItem(projectId, textFolderId, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation");
        return result;
    }
    int folderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, folderId, indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, folderId, name));
    IFOKDO(result, skrdata->treeHub()->setTrashedWithChildren(projectId, folderId, isTrashed));
    IFOKDO(result, skrdata->treePropertyHub()->setProperty(projectId, folderId, "label", badge, true, true, false));


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


    QTextDocument folderDoc;

    folderDoc.setHtml(QString::fromUtf8(textLines));


    // create note


    result = this->createNote(projectId, 2, plumeId, name, tempDirPath + "/text/N", noteFolderId);
    bool ok;
    int  noteId = result.getData("treeItemId", -2).toInt(&ok);

    if (!ok) {
        result = SKRResult(SKRResult::Critical, this, "bad_conversion_to_int");
        return result;
    }

    if (result.getData("hasContent", false).toBool()) {
        result = skrdata->treeHub()->setTreeRelationship(projectId, noteId, folderId);
    }

    // create synopsis


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

    IFOKDO(result, skrdata->treeHub()->setSecondaryContent(projectId, folderId, synDoc.toHtml()));


    // create sheet

    result = skrdata->treeHub()->addChildTreeItem(projectId, textFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation_after_folder");
        return result;
    }
    int sheetId = result.getData("treeItemId", -2).toInt();


    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, sheetId, indent + 1));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, sheetId, tr("%1 (content)").arg(name)));


    //
    IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, sheetId, folderDoc.toHtml()));


    // associate "attendance" notes

    QStringList attendIds = xml.attributes().value("attend").toString().split("-", Qt::SkipEmptyParts);

    for (const QString& attendIdString : qAsConst(attendIds)) {
        int attendId = attendIdString.toInt();

        if (attendId == 0) {
            continue;
        }

        int skrNoteId = m_attendanceConversionHash.value(attendId);

        // associate :
        result = skrdata->treeHub()->setTreeRelationship(projectId, skrNoteId, folderId);
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

    int plumeId       = xml.attributes().value("number").toInt();
    QString name      = xml.attributes().value("name").toString();
    bool    isTrashed = xml.attributes().value("isTrashed").toString() == "yes";
    QString badge     = xml.attributes().value("badge").toString();

    // create sheet

    result = skrdata->treeHub()->addChildTreeItem(projectId, textFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation");
        return result;
    }
    int sheetId = result.getData("treeItemId", -2).toInt();


    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, sheetId, indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, sheetId, name));
    IFOKDO(result, skrdata->treeHub()->setTrashedWithChildren(projectId, sheetId, isTrashed));
    IFOKDO(result, skrdata->treePropertyHub()->setProperty(projectId, sheetId, "label", badge, true, true, false));


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


    result = this->createNote(projectId, 2, plumeId, name, tempDirPath + "/text/N", noteFolderId);
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

SKRResult SKRPlumeCreatorImporter::createAttendFolder(int                     projectId,
                                                      int                     indent,
                                                      const QXmlStreamReader& xml,
                                                      const QString         & tempDirPath,
                                                      int                     attendFolderId)
{
    SKRResult result(this);

    int plumeId  = xml.attributes().value("number").toInt();
    QString name = xml.attributes().value("name").toString();


    // create folder

    result = skrdata->treeHub()->addChildTreeItem(projectId, attendFolderId, "FOLDER");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation");
        return result;
    }
    int folderId = result.getData("treeItemId", -2).toInt();

    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, folderId, indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, folderId, name));


    // - fetch text

    QFileInfo textFileInfo(tempDirPath + "/attend/A" + QString::number(plumeId) + ".html");

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


    QTextDocument folderDoc;

    folderDoc.setHtml(QString::fromUtf8(textLines));


    // create sheet

    result = skrdata->treeHub()->addChildTreeItem(projectId, attendFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation_after_folder");
        return result;
    }
    int sheetId = result.getData("treeItemId", -2).toInt();


    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, sheetId, indent + 1));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, sheetId, tr("%1 (content)").arg(name)));


    //
    IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, sheetId, folderDoc.toHtml()));

    result.addData("treeItemId", folderId);

    return result;
}

// -----------------------------------------------------------------------------------------------


SKRResult SKRPlumeCreatorImporter::createAttendObject(int                     projectId,
                                                      int                     indent,
                                                      const QXmlStreamReader& xml,
                                                      const QString         & tempDirPath,
                                                      int                     attendFolderId)
{
    SKRResult result(this);

    int plumeId  = xml.attributes().value("number").toInt();
    QString name = xml.attributes().value("name").toString();


    QFileInfo textFileInfo(tempDirPath + "/attend/A" + QString::number(plumeId) + ".html");

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


    QTextDocument attendDoc;

    attendDoc.setHtml(QString::fromUtf8(textLines));


    // create sheet

    result = skrdata->treeHub()->addChildTreeItem(projectId, attendFolderId, "TEXT");
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "text_creation_after_folder");
        return result;
    }
    int sheetId = result.getData("treeItemId", -2).toInt();


    IFOKDO(result, skrdata->treeHub()->setIndent(projectId, sheetId, indent));
    IFOKDO(result, skrdata->treeHub()->setTitle(projectId, sheetId, name));

    IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, sheetId, attendDoc.toHtml()));


    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRPlumeCreatorImporter::createNote(int projectId, int indent, int plumeId, const QString& name,
                                              const QString& tempDirPath, int parentFolderId)
{
    SKRResult result(this);

    // fetch text

    QFileInfo noteFileInfo(tempDirPath + QString::number(plumeId) + ".html");

    if (!noteFileInfo.exists()) {
        result = SKRResult(SKRResult::Critical, this, "no_attend_file");
        result.addData("filePath", noteFileInfo.absoluteFilePath());
        return result;
    }


    QFile noteFile(noteFileInfo.absoluteFilePath());

    if (!noteFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_attend_file");
        result.addData("filePath", noteFileInfo.absoluteFilePath());
        return result;
    }

    QByteArray lines = noteFile.readAll();


    QTextDocument noteDoc;

    noteDoc.setHtml(QString::fromUtf8(lines));


    if (noteDoc.characterCount() > 1) {
        result.addData("hasContent", true);

        result = skrdata->treeHub()->addChildTreeItem(projectId, parentFolderId, "TEXT");
        IFKO(result) {
            result = SKRResult(SKRResult::Critical, this, "note_creation");
            return result;
        }
        int noteId = result.getData("treeItemId", -2).toInt();


        IFOKDO(result, skrdata->treeHub()->setIndent(projectId, noteId, indent));
        IFOKDO(result, skrdata->treeHub()->setTitle(projectId, noteId, name));
        IFOKDO(result, skrdata->treeHub()->setType(projectId, noteId, "TEXT"));


        IFOKDO(result, skrdata->treeHub()->setPrimaryContent(projectId, noteId, noteDoc.toHtml()));
    }
    else {
        result.addData("hasContent", false);
    }
    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRPlumeCreatorImporter::createTagsFromAttend(int                     projectId,
                                                        int                     attendId,
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
            int tagId = result.getData("tagId", -2).toInt();

            result = skrdata->tagHub()->setTagRelationship(projectId,
                                                           attendId,
                                                           tagId);

            skrdata->tagHub()->setTagRandomColors(projectId, tagId);
        }
    }


    return result;
}
