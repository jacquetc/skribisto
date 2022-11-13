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
    if(!url.isValid()){
        return;
    }

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

