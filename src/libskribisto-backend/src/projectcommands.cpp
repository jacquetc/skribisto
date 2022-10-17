#include "projectcommands.h"
#include "skrdata.h"

ProjectCommands::ProjectCommands(QObject *parent, QUndoStack *undoStack)
    : QObject{parent}, m_undoStack(undoStack)
{
    m_instance = this;

}

void ProjectCommands::save(int projectId)
{
    m_undoStack->clear();
    skrdata->projectHub()->saveProject(projectId);
}

void ProjectCommands::closeProject(int projectId)
{
    m_undoStack->clear();
    skrdata->projectHub()->closeProject(projectId);
}

void ProjectCommands::saveAs(int projectId, const QUrl &url)
{
    m_undoStack->clear();
    skrdata->projectHub()->saveProjectAs(projectId, "skrib", url);
}

void ProjectCommands::loadProject(const QUrl &url)
{
    m_undoStack->clear();
    SKRResult result = skrdata->projectHub()->loadProject(url);

    if(result.isSuccess()){
        skrdata->projectHub()->setActiveProject(result.getData("projectId", -1).toInt());
    }
}



void ProjectCommands::setProjectName(int projectId, const QString &name)
{
    m_undoStack->push(new SetProjectNameCommand(projectId, name));

}

void ProjectCommands::setAuthor(int projectId, const QString &author)
{
    m_undoStack->push(new SetAuthorCommand(projectId, author));
}

void ProjectCommands::setLanguageCode(int projectId, const QString &newLanguage)
{
    m_undoStack->push(new SetLanguageCodeCommand(projectId, newLanguage));
}

ProjectCommands *ProjectCommands::m_instance = nullptr;


Command *ProjectCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
    if(action == "set_project_name"){
        return new SetProjectNameCommand(parameters.value("projectId").toInt(), parameters.value("name").toString());
    }

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

