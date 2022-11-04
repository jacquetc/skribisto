#include "projectcommands.h"
#include "skrdata.h"
#include "project/plmprojectmanager.h"
#include "project/plmproject.h"
#include "exporter.h"
#include "importer.h"

ProjectCommands::ProjectCommands(QObject *parent, QUndoStack *undoStack)
    : QObject{parent}, m_undoStack(undoStack)
{
    m_instance = this;

    Exporter::init();
    Importer::init();
}

int ProjectCommands::createNewEmptyProject()
{
    SKRResult result(this);
    PLMProject *project = new PLMProject(plmProjectManager, &result);

    plmProjectManager->loadProject(project);

    skrdata->projectHub()->setProjectLoaded(project->id());

    return project->id();
}


// ----------------------------------------------------------------------------

SKRResult ProjectCommands::save(int projectId)
{
    PLMProject * project = plmProjectManager->project(projectId);
    SKRResult result = Exporter::exportProject(projectId, project->getFileName(), project->getType(), QVariantMap());
    skrdata->projectHub()->setProjectSaved(projectId);

    return result;
}

// ----------------------------------------------------------------------------

SKRResult ProjectCommands::saveAProjectCopy(int            projectId,
                                          const QString& extension,
                                          const QUrl   & path)
{
    SKRResult result(this);

    // firstly, save the project
    result = this->save(projectId);

    // then create a copy
    IFOK(result) {
        result =  Exporter::exportProject(projectId, path, extension, QVariantMap());
    }



//    IFKO(result) {
//        emit errorSent(result);
//    }

    return result;
}



// ----------------------------------------------------------------------------

SKRResult ProjectCommands::backupAProject(int            projectId,
                                        const QString& type,
                                        const QUrl   & folderPath)
{
    SKRResult result(this);

    PLMProject * project = plmProjectManager->project(projectId);
    QUrl projectPath = project->getFileName();

    if (projectPath.isEmpty()) {
        result = SKRResult(SKRResult::Warning, this, "no_path");
        result.addData("projectId", projectId);
    }

//    if (projectPath.scheme() == "qrc") {
//        result = SKRResult(SKRResult::Warning, this, "qrc_projects_cant_back_up");
//        result.addData("projectId", projectId);
//    }


    IFOK(result) {
        // verify backup path
        QFileInfo folderInfo(folderPath.toLocalFile());

        if (!folderInfo.exists()) {
            result = SKRResult(SKRResult::Critical, this, "path_dont_exist");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isDir()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_a_directory");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isWritable()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_writable");
            result.addData("projectId", projectId);
        }
    }


    // determine file base
    QFileInfo info(projectPath.toLocalFile());
    QFileInfo backupFolderInfo(folderPath.toLocalFile());

    QString   backupFile;
    if(projectPath.scheme() == "qrc"){
        backupFile = backupFolderInfo.filePath() + "/" + projectPath.path().split("/").last();
        backupFile.remove(".skrib");
    }
    else{
        backupFile = backupFolderInfo.filePath() + "/" + info.completeBaseName();
    }

    // add date and time :
    QDateTime now     = QDateTime::currentDateTime();
    QString   nowText = now.toString("_yyyy-MM-dd-HHmmss");

    backupFile = backupFile + nowText;

    // add suffix :
    backupFile = backupFile + "." + type;

    // firstly, save the project
    if(projectPath.scheme() != "qrc"){
        IFOKDO(result, this->save(projectId));
    }

    // then create a copy
    IFOK(result) {
        result = this->saveAProjectCopy(projectId,
                                             type,
                                             QUrl::fromLocalFile(backupFile));
    }

//    IFKO(result) {
//        emit errorSent(result);
//    }

    return result;
}

// ----------------------------------------------------------------------------


void ProjectCommands::closeProject(int projectId)
{
    m_undoStack->clear();
    skrdata->projectHub()->closeProject(projectId);
}


// ----------------------------------------------------------------------------

SKRResult ProjectCommands::saveAs(int projectId, const QUrl &url, const QString &extension)
{
    SKRResult result(this);

    result = Exporter::exportProject(projectId, url, extension, QVariantMap());
    skrdata->projectHub()->setProjectSaved(projectId);
    plmProjectManager->project(projectId)->setPath(url);

    return result;
}

// ----------------------------------------------------------------------------

QString ProjectCommands::getSaveFilter() const
{
    return Exporter::getSaveFilter();
}

QList<QPair<QString, QString> > ProjectCommands::getExportExtensions() const
{
    return Exporter::getExportExtensions();
}

QStringList ProjectCommands::getExportExtensionHumanNames() const
{
    return Exporter::getExportExtensionHumanNames();
}


// ----------------------------------------------------------------------------

SKRResult ProjectCommands::exportProject(int projectId, const QUrl &url, const QString &extension, const QVariantMap &parameters, QList<int> treeItemIds)
{
    return Exporter::exportProject(projectId, url, extension, parameters, treeItemIds);
}

// ----------------------------------------------------------------------------

QTextDocument *ProjectCommands::getPrintTextDocument(int projectId, const QVariantMap &parameters, QList<int> treeItemIds) const
{
    SKRResult result(this);
    QTextDocument *textDocument = Exporter::getPrintTextDocument(projectId, parameters, treeItemIds, &result);



    return textDocument;
}

// ----------------------------------------------------------------------------

void ProjectCommands::loadProject(const QUrl &url)
{
    m_undoStack->clear();

    SKRResult result(this);


    QString suffix = url.toString().split(".").last();

    int projectId = Importer::importProject(url, suffix, QVariantMap(), result);

    skrdata->projectHub()->setProjectLoaded(projectId);

}



// ----------------------------------------------------------------------------


void ProjectCommands::setProjectName(int projectId, const QString &name)
{
    m_undoStack->push(new SetProjectNameCommand(projectId, name));

}


// ----------------------------------------------------------------------------

void ProjectCommands::setAuthor(int projectId, const QString &author)
{
    m_undoStack->push(new SetAuthorCommand(projectId, author));
}


// ----------------------------------------------------------------------------

void ProjectCommands::setLanguageCode(int projectId, const QString &newLanguage)
{
    m_undoStack->push(new SetLanguageCodeCommand(projectId, newLanguage));
}

// ----------------------------------------------------------------------------

void ProjectCommands::setActiveProject(int projectId)
{
    skrdata->projectHub()->setActiveProject(projectId);
}
// ----------------------------------------------------------------------------


ProjectCommands *ProjectCommands::m_instance = nullptr;



// ----------------------------------------------------------------------------

Command *ProjectCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
    if(action == "set_project_name"){
        return new SetProjectNameCommand(parameters.value("projectId").toInt(), parameters.value("name").toString());
    }

    return nullptr;
}


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

SetProjectNameCommand::SetProjectNameCommand(int projectId, const QString &name) :
    Command("Set project name to " + name),
    m_newName(name), m_projectId(projectId)
{

}

void SetProjectNameCommand::undo()
{
    skrdata->projectHub()->setProjectName(m_projectId, m_oldName);
}

void SetProjectNameCommand::redo()
{
    m_oldName = skrdata->projectHub()->getProjectName(m_projectId);
    skrdata->projectHub()->setProjectName(m_projectId, m_newName);
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

SetAuthorCommand::SetAuthorCommand(int projectId, const QString &author) :
    Command("Set author to " + author),
    m_newName(author), m_projectId(projectId)
{

}

void SetAuthorCommand::undo()
{
    skrdata->projectHub()->setAuthor(m_projectId, m_oldName);
}

void SetAuthorCommand::redo()
{
    m_oldName = skrdata->projectHub()->getAuthor(m_projectId);
    skrdata->projectHub()->setAuthor(m_projectId, m_newName);
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

SetLanguageCodeCommand::SetLanguageCodeCommand(int projectId, const QString &languageCode) :
    Command("Set project language to " + languageCode),
    m_newLanguageCode(languageCode), m_projectId(projectId)
{

}

void SetLanguageCodeCommand::undo()
{
    skrdata->projectHub()->setLangCode(m_projectId, m_oldLanguageCode);
}

void SetLanguageCodeCommand::redo()
{
    m_oldLanguageCode = skrdata->projectHub()->getLangCode(m_projectId);
    skrdata->projectHub()->setLangCode(m_projectId, m_newLanguageCode);
}

