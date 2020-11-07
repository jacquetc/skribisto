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

#include <QtSql/QSqlQuery>
#include <QtSql/QSqlRecord>
#include <QVariant>
#include <QTemporaryFile>
#include <QDebug>
#include <QDir>
#include <QProcess>
#include <QByteArray>
#include <QRegularExpression>
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

//-----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createSQLiteDbFrom(const QString& type,
                                             const QUrl   & fileName,
                                             int            projectId,
                                             PLMError     & error)
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
            error.setSuccess(false);
            error.setErrorCode("E_IMPORTER_file_does_not_exist");
            error.addData("filePath", fileNameString);
            qWarning() << fileNameString + " doesn't exist";
            return QSqlDatabase();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            error.setSuccess(false);
            error.setErrorCode("E_IMPORTER_file_cant_be_opened");
            error.addData("filePath", fileNameString);
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
            error.setSuccess(false);
            error.setErrorCode("E_IMPORTER_cant_open_database");
            error.addData("filePath", tempFileName);

            return QSqlDatabase();
        }

        // upgrade :
        IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
        IFKO(error) {
            error.setSuccess(false);
            error.setErrorCode("E_IMPORTER_upgrade_sqlite_failed");
            error.addData("filePath", fileNameString);
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

        for(const QString& string : optimization) {
            QSqlQuery query(sqlDb);

            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();

        // clean-up :
        sqlDb.transaction();
        PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");
        IFOKDO(error, sheetQueries.renumberSortOrder());
        PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");
        IFOKDO(error, noteQueries.renumberSortOrder());
        sqlDb.commit();
        return sqlDb;
    }

    return QSqlDatabase();
}

//-----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createEmptySQLiteProject(int projectId, PLMError& error)
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
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_database");
        error.addData("filePath", tempFileName);
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

    for(const QString& string : optimization) {
        QSqlQuery query(sqlDb);

        query.prepare(string);
        query.exec();
    }

    // new project :
    IFOKDO(error, this->executeSQLFile(":/sql/sqlite_project.sql", sqlDb));
    QString sqlString =
            QString("INSERT INTO tbl_project (dbl_database_version) VALUES (%1)")
            .arg(1.1);

    IFOKDO(error, this->executeSQLString(sqlString, sqlDb));

    // upgrade :
    IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
    IFKO(error) {
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_upgrade_sqlite_failed");
        error.addData("filePath", tempFileName);
        return QSqlDatabase();
    }


    IFOK(error) {
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
    IFOK(error) {
        sqlDb.commit();
    }

    // clean-up :
    sqlDb.transaction();
    PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");

    IFOKDO(error, sheetQueries.renumberSortOrder());
    PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");

    IFOKDO(error, noteQueries.renumberSortOrder());
    sqlDb.commit();

    return sqlDb;
}

//-----------------------------------------------------------------------------------------------

PLMError PLMImporter::importPlumeCreatorProject(int projectId, const QUrl &plumeFileName)
{
    PLMError error;

    // create temp file
    QTemporaryDir tempDir;
    tempDir.setAutoRemove(true);
    if (!tempDir.isValid()) {
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_temp_dir");
        return error;
    }

    QString tempDirPath = tempDir.path();

    // extract zip

    JlCompress::extractDir(plumeFileName.toLocalFile(), tempDirPath);

    //----------------------------- read attend---------------------------------------

    QFileInfo attendFileInfo(tempDirPath + "/attendance");

    if(!attendFileInfo.exists()){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_attend_file");
        return error;
    }



    QFile attendFile(attendFileInfo.absoluteFilePath());
    if (!attendFile.open(QIODevice::ReadOnly | QIODevice::Text)){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_attend_file");
        return error;
    }


    QByteArray attendLines = attendFile.readAll();

    QXmlStreamReader attendXml(attendLines);



    m_attendanceConversionHash.clear();


    while (attendXml.readNextStartElement() && attendXml.name() == "plume-attendance") {
        QStringList rolesNames = attendXml.attributes().value("rolesNames").toString().split("--", Qt::SkipEmptyParts);
        QStringList levelsNames = attendXml.attributes().value("levelsNames").toString().split("--", Qt::SkipEmptyParts);

        QStringList box_1Names = attendXml.attributes().value("box_1").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_2Names = attendXml.attributes().value("box_2").toString().split("--", Qt::SkipEmptyParts);
        QStringList box_3Names = attendXml.attributes().value("box_3").toString().split("--", Qt::SkipEmptyParts);

        QStringList spinBox_1Names = attendXml.attributes().value("spinBox_1").toString().split("--", Qt::SkipEmptyParts);




        while (attendXml.readNextStartElement() && attendXml.name() == "group") {
            int attendNumber = attendXml.attributes().value("number").toInt();
            QString groupName = attendXml.attributes().value("name").toString();
            error = this->createNote(projectId, 0, attendNumber, groupName, tempDirPath + "/attend/A");
            IFOK(error){
                int groupNoteId = error.getData("noteId", -2).toInt();
                m_attendanceConversionHash.insert(attendNumber, groupNoteId);

                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_1", box_1Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_2", box_2Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "box_3", box_3Names);
                this->createTagsFromAttend(projectId, groupNoteId, attendXml, "spinBox_1", spinBox_1Names);
            }


            while (attendXml.readNextStartElement() && attendXml.name() == "obj") {
                int attendNumber = attendXml.attributes().value("number").toInt();
                QString objName = attendXml.attributes().value("name").toString();
                error = this->createNote(projectId, 1, attendNumber, objName, tempDirPath + "/attend/A");
                IFOK(error){
                    int objNoteId = error.getData("noteId", -2).toInt();
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



    //----------------------------- read tree---------------------------------------

    QFileInfo treeFileInfo(tempDirPath + "/tree");

    if(!treeFileInfo.exists()){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_tree_file");
        error.addData("filePath", treeFileInfo.absoluteFilePath());
        return error;
    }



    QFile treeFile(treeFileInfo.absoluteFilePath());
    if (!treeFile.open(QIODevice::ReadOnly | QIODevice::Text)){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_tree_file");
        error.addData("filePath", treeFileInfo.absoluteFilePath());
        return error;
    }


    QByteArray lines = treeFile.readAll();

    QXmlStreamReader xml(lines);




    while (xml.readNextStartElement() && xml.name() == "plume-tree") {
        // pick project name
        QString projectName = xml.attributes().value("projectName").toString();
        plmdata->projectHub()->setProjectName(projectId, projectName);


        while (xml.readNextStartElement() && xml.name() == "book") {
            this->createPapersAndAssociations(projectId, 0, xml, tempDirPath);

            while (xml.readNextStartElement()) {

                if(xml.name() == "act"){
                    //qDebug() << "a name" << xml.name();
                    this->createPapersAndAssociations(projectId, 1, xml, tempDirPath);

                    while (xml.readNextStartElement() && xml.name() == "chapter") {
                        //qDebug() << "c name" << xml.name();
                        this->createPapersAndAssociations(projectId, 2, xml, tempDirPath);



                        while (xml.readNextStartElement() && xml.name() == "scene") {
                            //qDebug() << "s name" << xml.name();
                            this->createPapersAndAssociations(projectId, 3, xml, tempDirPath);
                            xml.readElementText();

                            //            for(const QXmlStreamAttribute &attribute : xml.attributes()){
                            //                qDebug() << "attr" << attribute.name() << ":" << attribute.value();
                            //            }

                            // qDebug() << "decl" << xml.namespaceDeclarations().first().namespaceUri();
                        }
                    }

                }
                else if(xml.name() == "chapter"){


                    //qDebug() << "c name" << xml.name();
                    this->createPapersAndAssociations(projectId, 1, xml, tempDirPath);



                    while (xml.readNextStartElement() && xml.name() == "scene") {
                        //qDebug() << "s name" << xml.name();
                        this->createPapersAndAssociations(projectId, 2, xml, tempDirPath);
                        xml.readElementText();
                        //qDebug() << "read" << xml.readElementText();

                        //            for(const QXmlStreamAttribute &attribute : xml.attributes()){
                        //                qDebug() << "attr" << attribute.name() << ":" << attribute.value();
                        //            }

                        // qDebug() << "decl" << xml.namespaceDeclarations().first().namespaceUri();
                    }

                }

            }
        }
    }
    if (xml.hasError()) {

        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_error_in_tree_xml");

        error.addData("xmlError", QString("%1\nLine %2, column %3")
                      .arg(xml.errorString())
                      .arg(xml.lineNumber())
                      .arg(xml.columnNumber()));

        return error;

    }


    //----------------------------- user dict---------------------------------------
    //

    QFileInfo dictFileInfo(tempDirPath + "dicts/userDict.dict_plume");

    if(!dictFileInfo.exists()){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_dict_file");
        error.addData("filePath", dictFileInfo.absoluteFilePath());
        return error;
    }



    QFile dictFile(dictFileInfo.absoluteFilePath());
    if (!treeFile.open(QIODevice::ReadOnly | QIODevice::Text)){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_dict_file");
        error.addData("filePath", dictFileInfo.absoluteFilePath());
        return error;
    }


    QByteArray dictLines = dictFile.readAll();

    QString dictString = QString::fromUtf8(dictLines);

    QStringList dictWords = dictString.split(";", Qt::SkipEmptyParts);

    plmdata->projectDictHub()->setProjectDictList(projectId , dictWords);



    return error;
}

PLMError PLMImporter::createPapersAndAssociations(int projectId, int indent, const QXmlStreamReader &xml, const QString &tempDirPath)
{
    PLMError error;

    int plumeId = xml.attributes().value("number").toInt();
    QString name = xml.attributes().value("name").toString();

    // create sheet

    error = plmdata->sheetHub()->addChildPaper(projectId, -1);
    IFKO(error){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_sheet_creation");
        return error;
    }
    int sheetId = error.getData("paperId", -2).toInt();


    IFOKDO(error, plmdata->sheetHub()->setIndent(projectId, sheetId, indent));
    IFOKDO(error, plmdata->sheetHub()->setTitle(projectId, sheetId, name));


    // - fetch text

    QFileInfo textFileInfo(tempDirPath + "/text/T" + QString::number(plumeId) + ".html");

    if(!textFileInfo.exists()){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_text_file");
        error.addData("filePath", textFileInfo.absoluteFilePath());
        return error;
    }



    QFile textFile(textFileInfo.absoluteFilePath());
    if (!textFile.open(QIODevice::ReadOnly | QIODevice::Text)){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_text_file");
        error.addData("filePath", textFileInfo.absoluteFilePath());
        return error;
    }

    QByteArray textLines = textFile.readAll();


    QTextDocument sheetDoc;
    sheetDoc.setHtml(QString::fromUtf8(textLines));

    IFOKDO(error, plmdata->sheetHub()->setContent(projectId, sheetId, sheetDoc.toMarkdown()));


    IFKO(error){
        return error;
    }

    // create note


    error = this->createNote(projectId, indent, plumeId, name, tempDirPath + "/text/N");
    bool ok;
    int noteId = error.getData("noteId", -2).toInt(&ok);
    if(!ok){
        error.setSuccess(false);
        return error;
    }

    plmdata->noteHub()->setSheetNoteRelationship(projectId, sheetId, noteId, false);




    // create synopsis note

    error = this->createNote(projectId, indent, plumeId, name, tempDirPath + "/text/S");
    int synopsisId = error.getData("noteId", -2).toInt(&ok);
    if(!ok){
        error.setSuccess(false);
        return error;
    }
    plmdata->noteHub()->setSheetNoteRelationship(projectId, sheetId, synopsisId, true);

    // associate "attendance" notes
    QStringList attendIds = xml.attributes().value("attend").toString().split("-", Qt::SkipEmptyParts);

    for(const QString &attendIdString : attendIds){
        int attendId = attendIdString.toInt();

        if(attendId == 0){
            continue;
        }

        int skrNoteId = m_attendanceConversionHash.value(attendId);

        //associate :
        plmdata->noteHub()->setSheetNoteRelationship(projectId, sheetId, skrNoteId, false);
    }


    // save

    IFOKDO(error, plmdata->projectHub()->saveProject(projectId));

    QUrl url = plmdata->projectHub()->getPath(projectId);
    plmdata->projectHub()->closeProject(projectId);

    plmdata->projectHub()->loadProject(url);


    return error;
}

//-----------------------------------------------------------------------------------------------

PLMError PLMImporter::createNote(int projectId, int indent, int plumeId, const QString &name, const QString &tempDirPath)
{
    PLMError error;



    error = plmdata->noteHub()->addChildPaper(projectId, -1);
    IFKO(error){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_note_creation");
        return error;
    }
    int noteId = error.getData("paperId", -2).toInt();
    error.addData("noteId", noteId);


    IFOKDO(error, plmdata->noteHub()->setIndent(projectId, noteId, indent));
    IFOKDO(error, plmdata->noteHub()->setTitle(projectId, noteId, name));


    // fetch text

    QFileInfo attendFileInfo(tempDirPath + QString::number(plumeId) + ".html");

    if(!attendFileInfo.exists()){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_no_attend_file");
        error.addData("filePath", attendFileInfo.absoluteFilePath());
        return error;
    }



    QFile attendFile(attendFileInfo.absoluteFilePath());
    if (!attendFile.open(QIODevice::ReadOnly | QIODevice::Text)){
        error.setSuccess(false);
        error.setErrorCode("E_IMPORTER_cant_open_attend_file");
        error.addData("filePath", attendFileInfo.absoluteFilePath());
        return error;
    }

    QByteArray lines = attendFile.readAll();


    QTextDocument noteDoc;
    noteDoc.setHtml(QString::fromUtf8(lines));
    IFOKDO(error, plmdata->noteHub()->setContent(projectId, noteId, noteDoc.toMarkdown()));



    return error;

}

//-----------------------------------------------------------------------------------------------

PLMError PLMImporter::createTagsFromAttend(int projectId, int noteId, const QXmlStreamReader &xml, const QString &attributeName, const QStringList &values)
{
    PLMError error;

    if(values.isEmpty()){
        return error; // return successfully
    }




    bool ok;
    int index = xml.attributes().value(attributeName).toInt(&ok);
    if(!ok){
        error.setErrorCode("E_IMPORTER_conversion_to_int");
        error.setSuccess(false);
        return error;
    }

    if(values.count() <= index){
        return error;// return successfully
    }


    IFOK(error){

        error = plmdata->tagHub()->addTag(projectId, values.at(index));
        IFOK(error){
            error = plmdata->tagHub()->setTagRelationship(projectId, SKRTagHub::Note, noteId, error.getData("tagId", -2).toInt());
        }
    }

    return error;

}

// QSqlDatabase PLMImporter::copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
// projectId, PLMError &error)
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
//-----------------------------------------------------------------------------------------------

PLMError PLMImporter::executeSQLFile(const QString& fileName, QSqlDatabase& sqlDB) {
    PLMError error;
    QFile    file(fileName);

    // Read query file content
    file.open(QIODevice::ReadOnly);
    error = this->executeSQLString(file.readAll(), sqlDB);
    file.close();

    return error;
}

//-----------------------------------------------------------------------------------------------

PLMError PLMImporter::executeSQLString(const QString& sqlString, QSqlDatabase& sqlDB)
{
    PLMError error;

    QSqlQuery query(sqlDB);


    QString queryStr = sqlString + "\n";

    // Check if SQL Driver supports Transactions
    if (sqlDB.driver()->hasFeature(QSqlDriver::Transactions)) {
        // Replace comments and tabs and new lines with space
        queryStr =
                queryStr.replace(QRegularExpression("(\\/\\*(.)*?\\*\\/|^--.*\\n|\\t)",
                                                    QRegularExpression::CaseInsensitiveOption
                                                    | QRegularExpression::MultilineOption),
                                 " ");
        queryStr = queryStr.trimmed();


        // Extracting queries
        QStringList qList = queryStr.split('\n', Qt::SkipEmptyParts);


        QRegularExpression re_transaction("\\bbegin.transaction.*",
                                          QRegularExpression::CaseInsensitiveOption);
        QRegularExpression re_commit("\\bcommit.*",
                                     QRegularExpression::CaseInsensitiveOption);

        // Check if query file is already wrapped with a transaction
        bool isStartedWithTransaction = false;

        if (qList.size() > 1) {
            isStartedWithTransaction = qMax(re_transaction.match(qList.at(0)).hasMatch(),
                                            re_transaction.match(qList.at(1)).hasMatch());
        }

        if (!isStartedWithTransaction) sqlDB.transaction();

        // Execute each individual queries
        bool success = true;

        for (const QString& s : qList) {
            if (re_transaction.match(s).hasMatch()) sqlDB.transaction();
            else if (re_commit.match(s).hasMatch()) sqlDB.commit();
            else {
                query.exec(s);

                if (query.lastError().type() != QSqlError::NoError) {
                    qDebug() << query.lastError().text();
                    sqlDB.rollback();

                    success = false;

                    //
                }
            }
        }

        if (!isStartedWithTransaction) sqlDB.commit();

        if (success == false) {
            error.setSuccess(false);
            return error;
        }

        // Sql Driver doesn't supports transaction
    } else {
        // ...so we need to remove special queries (`begin transaction` and
        // `commit`)
        queryStr =
                queryStr.replace(QRegularExpression(
                                     "(\\bbegin.transaction.*;|\\bcommit.*;|\\/\\*(.|\\n)*?\\*\\/|^--.*\\n|\\t|\\n)",
                                     QRegularExpression::CaseInsensitiveOption
                                     | QRegularExpression::MultilineOption),
                                 " ");
        queryStr = queryStr.trimmed();

        // Execute each individual queries
        QStringList qList = queryStr.split(';', Qt::SkipEmptyParts);
        for(const QString& s : qList) {
            query.exec(s);

            if (query.lastError().type() != QSqlError::NoError) {
                qDebug() << query.lastError().text();
                error.setSuccess(false);
                return error;
            }
        }
    }
    return error;
}

//-----------------------------------------------------------------------------------------------

