/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmimporter.cpp                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/

#include "plmimporter.h"
#include "plmupgrader.h"
#include "tasks/plmsqlqueries.h"
#include "plmdata.h"
#include "skrsqltools.h"

#include <QtSql/QSqlQuery>
#include <QtSql/QSqlRecord>
#include <QVariant>
#include <QTemporaryFile>
#include <QDebug>
#include <QDir>
#include <QProcess>
#include <QByteArray>
#include <QSqlDriver>
#include <QSqlError>
#include <QRandomGenerator>
#include <QUrl>
#include <QTemporaryDir>
#include <QXmlStreamReader>
#include <quazip/JlCompress.h>
#include <QTextDocument>

PLMImporter::PLMImporter(QObject *parent) :
    QObject(parent)
{}

// -----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createSQLiteDbFrom(const QString& type,
                                             const QUrl   & fileName,
                                             int            projectId,
                                             SKRResult     & result)
{
    if (type == "SQLITE") {
        // create temp file
        QTemporaryFile tempFile;
        tempFile.open();
        tempFile.setAutoRemove(false);
        QString tempFileName = tempFile.fileName();

        // copy db file to temp
        QString fileNameString;

        if (fileName.scheme() == "qrc") {
            fileNameString = fileName.toString(QUrl::RemoveScheme);
            fileNameString = ":" + fileNameString;
        }
        else {
            fileNameString = fileName.toLocalFile();
        }

        QFile file(fileNameString);

        if (!file.exists()) {
            result = SKRResult(SKRResult::Critical, this, "file_does_not_exist");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " doesn't exist";
            return QSqlDatabase();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            result = SKRResult(SKRResult::Critical, this, "file_cant_be_opened");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " can't be opened";
            return QSqlDatabase();
        }

        QByteArray array(file.readAll());
        tempFile.write(array);
        tempFile.close();

        // open temp file
        QSqlDatabase sqlDb =
                QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));
        sqlDb.setHostName("localhost");
        sqlDb.setDatabaseName(tempFileName);

        // QSqlDatabase memoryDb = copySQLiteDbToMemory(sqlDb, projectId);
        //        QSqlDatabase db = QSqlDatabase::database("db_to_be_imported",
        // false);
        //        db.removeDatabase("db_to_be_imported");
        bool ok = sqlDb.open();

        if (!ok) {
            result = SKRResult(SKRResult::Critical, this, "cant_open_database");
            result.addData("filePath", tempFileName);

            return QSqlDatabase();
        }

        // upgrade :
        IFOKDO(result, PLMUpgrader::upgradeSQLite(sqlDb));
        IFKO(result) {
            result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
            result.addData("filePath", fileNameString);
            return QSqlDatabase();
        }

        // optimization :
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY")
                     << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                     << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers=true");
        sqlDb.transaction();

        for (const QString& string : optimization) {
            QSqlQuery query(sqlDb);

            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();

        // clean-up :
        sqlDb.transaction();
        PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");
        IFOKDO(result, sheetQueries.renumberSortOrder());
        PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");
        IFOKDO(result, noteQueries.renumberSortOrder());
        sqlDb.commit();
        return sqlDb;
    }

    return QSqlDatabase();
}

// -----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createEmptySQLiteProject(int projectId, SKRResult& result)
{
    // create temp file
    QTemporaryFile tempFile;

    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();

    qDebug() << "tempFileName :" << tempFileName;

    // open temp file

    QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));

    sqlDb.setHostName("localhost");
    sqlDb.setDatabaseName(tempFileName);
    bool ok = sqlDb.open();

    if (!ok) {
        result = SKRResult(SKRResult::Critical, this, "cant_open_database");
        result.addData("filePath", tempFileName);
        return QSqlDatabase();
    }


    // optimization :
    QStringList optimization;

    optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                 << QStringLiteral("PRAGMA journal_mode=MEMORY")
                 << QStringLiteral("PRAGMA temp_store=MEMORY")
                 << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                 << QStringLiteral("PRAGMA synchronous = OFF")
                 << QStringLiteral("PRAGMA recursive_triggers=true");
    sqlDb.transaction();

    for (const QString& string : optimization) {
        QSqlQuery query(sqlDb);

        query.prepare(string);
        query.exec();
    }

    // new project :
    IFOKDO(result, SKRSqlTools::executeSQLFile(":/sql/sqlite_project.sql", sqlDb));

    // fetch db version

    QString dbVersion = "-2";
    IFOK(result){
        dbVersion = SKRSqlTools::getProjectTemplateDBVersion(&result);
    }



    QString sqlString =
            QString("INSERT INTO tbl_project (dbl_database_version) VALUES (%1)")
            .arg(dbVersion);

    IFOKDO(result, SKRSqlTools::executeSQLString(sqlString, sqlDb));

    // upgrade :
    IFOKDO(result, PLMUpgrader::upgradeSQLite(sqlDb));
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
        result.addData("filePath", tempFileName);
        return QSqlDatabase();
    }


    IFOK(result) {
        // create unique identifier


        QString randomString;

        {
            const QString possibleCharacters(
                        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
            const int randomStringLength = 12;

            for (int i = 0; i < randomStringLength; ++i)
            {
                quint32 generatedNumber = QRandomGenerator::system()->generate();
                int     index           = generatedNumber % possibleCharacters.length();
                QChar   nextChar        = possibleCharacters.at(index);
                randomString.append(nextChar);
            }
        }
        qDebug() << "randomString" << randomString;


        QString   value = randomString;
        int       id    = 1;
        QSqlQuery query(sqlDb);
        QString   queryStr = "UPDATE tbl_project SET t_project_unique_identifier = :value"
                             " WHERE l_project_id = :id"
                ;

        query.prepare(queryStr);
        query.bindValue(":value", value);
        query.bindValue(":id",    id);
        query.exec();
    }
    IFOK(result) {
        sqlDb.commit();
    }

    // clean-up :
    sqlDb.transaction();
    PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");

    IFOKDO(result, sheetQueries.renumberSortOrder());
    PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");

    IFOKDO(result, noteQueries.renumberSortOrder());
    sqlDb.commit();

    return sqlDb;
}

// -----------------------------------------------------------------------------------------------

SKRResult PLMImporter::importPlumeCreatorProject(const QUrl& plumeFileName, const QUrl& skribistoFileName)
{

    SKRResult result = plmdata->projectHub()->createNewEmptyProject(skribistoFileName);

    int projectId = -2;
    IFOK(result){
        projectId = result.getData("projectId", -2).toInt();
    }

    if(projectId == -2){
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


    while (attendXml.readNextStartElement() && attendXml.name() == "plume-attendance") {
        QStringList rolesNames  = attendXml.attributes().value("rolesNames").toString().split("--", Qt::SkipEmptyParts);
        QStringList levelsNames =
                attendXml.attributes().value("levelsNames").toString().split("--", Qt::SkipEmptyParts);

        QStringList box_1Names = attendXml.attributes().value("box_1").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_2Names = attendXml.attributes().value("box_2").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_3Names = attendXml.attributes().value("box_3").toString().split("--", Qt::SkipEmptyParts);

        QStringList spinBox_1Names =
                attendXml.attributes().value("spinBox_1").toString().split("--", Qt::SkipEmptyParts);


        while (attendXml.readNextStartElement() && attendXml.name() == "group") {
            int attendNumber  = attendXml.attributes().value("number").toInt();
            QString groupName = attendXml.attributes().value("name").toString();
            result = this->createNote(projectId, 0, attendNumber, groupName, tempDirPath + "/attend/A");
            IFOK(result) {
                int groupNoteId = result.getData("noteId", -2).toInt();

                m_attendanceConversionHash.insert(attendNumber, groupNoteId);

                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_1", box_1Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_2", box_2Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_3", box_3Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "spinBox_1", spinBox_1Names);
            }


            while (attendXml.readNextStartElement() && attendXml.name() == "obj") {
                int attendNumber = attendXml.attributes().value("number").toInt();
                QString objName  = attendXml.attributes().value("name").toString();
                result = this->createNote(projectId, 1, attendNumber, objName, tempDirPath + "/attend/A");
                IFOK(result) {
                    int objNoteId = result.getData("noteId", -2).toInt();

                    m_attendanceConversionHash.insert(attendNumber, objNoteId);


                    this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_1", box_1Names);
                    this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_2", box_2Names);
                    this->createTagsFromAttend(projectId, objNoteId, attendXml, "box_3", box_3Names);
                    this->createTagsFromAttend(projectId, objNoteId, attendXml, "spinBox_1", spinBox_1Names);

                    QString objQuickDetail = attendXml.attributes().value("quickDetails").toString();

                    plmdata->notePropertyHub()->setProperty(projectId, objNoteId, "label", objQuickDetail);
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


    while (xml.readNextStartElement() && xml.name() == "plume-tree") {
        // pick project name
        QString projectName = xml.attributes().value("projectName").toString();
        IFOKDO(result, plmdata->projectHub()->setProjectName(projectId, projectName));




        while (xml.readNextStartElement()){
            if(xml.name() == "trash") {
                xml.skipCurrentElement();
                continue;
            }
            if(xml.name() == "book") {
                IFOKDO(result, this->readXMLRecursivelyAndCreatePaper(projectId, 0, &xml, tempDirPath));

            }
        }
    }

    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "error_while_exploiting_tree");
        return result;
    }

    if (xml.hasError()) {
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
        result.addData("filePath", dictFileInfo.absoluteFilePath());
        result.addData("fileError", dictFile.error());
        return result;
    }


    QByteArray dictLines = dictFile.readAll();

    QString dictString = QString::fromUtf8(dictLines);

    QStringList dictWords = dictString.split(";", Qt::SkipEmptyParts);

    plmdata->projectDictHub()->setProjectDictList(projectId, dictWords);


    // save

    IFOKDO(result, plmdata->projectHub()->saveProject(projectId));

    QUrl url = plmdata->projectHub()->getPath(projectId);

    IFOKDO(result, plmdata->projectHub()->closeProject(projectId));

    IFOKDO(result, plmdata->projectHub()->loadProject(url));

    return result;
}

SKRResult PLMImporter::readXMLRecursivelyAndCreatePaper(int                     projectId,
                                                        int                     indent,
                                                        QXmlStreamReader *xml,
                                                        const QString         & tempDirPath)
{
    SKRResult result;

    if(!xml->readNextStartElement()){
        xml->readElementText();
        return result;
    }
    else {
        IFOKDO(result, this->createPapersAndAssociations(projectId, indent, *xml, tempDirPath));
        IFOKDO(result, readXMLRecursivelyAndCreatePaper(projectId, indent + 1, xml, tempDirPath));
    }

    while (xml->readNextStartElement()) {

        IFOKDO(result, this->createPapersAndAssociations(projectId, indent, *xml, tempDirPath));
        IFOKDO(result, readXMLRecursivelyAndCreatePaper(projectId, indent + 1, xml, tempDirPath));

    }
}


SKRResult PLMImporter::createPapersAndAssociations(int                     projectId,
                                                   int                     indent,
                                                   const QXmlStreamReader& xml,
                                                   const QString         & tempDirPath)
{
    SKRResult result(this);

    int plumeId  = xml.attributes().value("number").toInt();
    QString name = xml.attributes().value("name").toString();

    // create sheet

    result = plmdata->sheetHub()->addChildPaper(projectId, -1);
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "sheet_creation");
        return result;
    }
    int sheetId = result.getData("sheetId", -2).toInt();


    IFOKDO(result, plmdata->sheetHub()->setIndent(projectId, sheetId, indent));
    IFOKDO(result, plmdata->sheetHub()->setTitle(projectId, sheetId, name));


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

    IFOKDO(result, plmdata->sheetHub()->setContent(projectId, sheetId, sheetDoc.toMarkdown()));


    IFKO(result) {
        return result;
    }

    // create note


    result = this->createNote(projectId, indent, plumeId, name, tempDirPath + "/text/N");
    bool ok;
    int  noteId = result.getData("noteId", -2).toInt(&ok);

    if (!ok) {
        result = SKRResult(SKRResult::Critical, this, "bad_conversion_to_int");
        return result;
    }

    result = plmdata->noteHub()->setSheetNoteRelationship(projectId, sheetId, noteId, false);


    IFKO(result) {
        return result;
    }

    // create synopsis note


    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, sheetId);

    if (synopsisId == -2) {
        result = SKRResult(SKRResult::Critical, this, "no_synopsis_id");
        return result;
    }

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
    IFOKDO(result, plmdata->noteHub()->setContent(projectId, synopsisId, synDoc.toMarkdown()));


    // associate "attendance" notes

    QStringList attendIds = xml.attributes().value("attend").toString().split("-", Qt::SkipEmptyParts);

    for (const QString& attendIdString : attendIds) {
        int attendId = attendIdString.toInt();

        if (attendId == 0) {
            continue;
        }

        int skrNoteId = m_attendanceConversionHash.value(attendId);

        // associate :
        result = plmdata->noteHub()->setSheetNoteRelationship(projectId, sheetId, skrNoteId, false);
    }

    IFKO(result) {
        return result;
    }



    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult PLMImporter::createNote(int projectId, int indent, int plumeId, const QString& name,
                                  const QString& tempDirPath)
{
    SKRResult result(this);


    result = plmdata->noteHub()->addChildPaper(projectId, -1);
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "note_creation");
        return result;
    }
    int noteId = result.getData("paperId", -2).toInt();

    result.addData("noteId", noteId);


    IFOKDO(result, plmdata->noteHub()->setIndent(projectId, noteId, indent));
    IFOKDO(result, plmdata->noteHub()->setTitle(projectId, noteId, name));


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
    IFOKDO(result, plmdata->noteHub()->setContent(projectId, noteId, noteDoc.toMarkdown()));


    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult PLMImporter::createTagsFromAttend(int                     projectId,
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
        result = plmdata->tagHub()->addTag(projectId, values.at(index));
        IFOK(result) {
            result = plmdata->tagHub()->setTagRelationship(projectId,
                                                           SKRTagHub::Note,
                                                           noteId,
                                                           result.getData("tagId", -2).toInt());
        }
    }

    return result;
}

// QSqlDatabase PLMImporter::copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
// projectId, SKRResult &result)
// {
//    //TODO: not good, to remove ? !!!!
//    QString table("mytable");
//    QSqlDatabase srcDB = sourceSqlDb;
//    srcDB.open();
//    QSqlDatabase destDB = QSqlDatabase::addDatabase("QSQLITE",
// QString::number(projectId));
//    destDB.setHostName("localhost");
//    destDB.setDatabaseName(":memory:");
//    destDB.open();
//    {
//        QSqlQuery srcQuery(srcDB);
//        QSqlQuery destQuery(destDB);

//        // get table schema
//        if (!srcQuery.exec(QString("SHOW CREATE TABLE %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        QString tableCreateStr;

//        while (srcQuery.next()) {
//            tableCreateStr = srcQuery.value(1).toString();
//        }

//        // drop destTable if exists
//        if (!destQuery.exec(QString("DROP TABLE IF EXISTS %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        // create new one
//        if (!destQuery.exec(tableCreateStr)) {
//            return QSqlDatabase();
//        }

//        // copy all entries
//        if (!srcQuery.exec(QString("SELECT * FROM %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        while (srcQuery.next()) {
//            QSqlRecord record = srcQuery.record();
//            QStringList names;
//            QStringList placeholders;
//            QList<QVariant > values;

//            for (int i = 0; i < record.count(); ++i) {
//                names << record.fieldName(i);
//                placeholders << ":" + record.fieldName(i);
//                QVariant value = srcQuery.value(i);

//                if (value.type() == QVariant::String) {
//                    values << "\"" + value.toString() + "\"";
//                } else {
//                    values << value;
//                }
//            }

//            // build new query
//            QString queryStr;
//            queryStr.append("INSERT INTO " + table);
//            queryStr.append(" (" + names.join(", ") + ") ");
//            queryStr.append(" VALUES (" + placeholders.join(", ") + ");");
//            destQuery.prepare(queryStr);

//            foreach (QVariant value, values) {
//                destQuery.addBindValue(value);
//            }

//            if (!destQuery.exec()) {
//                return QSqlDatabase();
//            }
//        }
//    }
//    return destDB;
// }

// -----------------------------------------------------------------------------------------------
