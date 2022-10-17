#ifndef PROJECTCOMMANDS_H
#define PROJECTCOMMANDS_H

#include <QObject>
#include <QUndoCommand>
#include "skribisto_backend_global.h"
#include "interfaces/invokablecommandgroupinterface.h"

#define projectCommands ProjectCommands::instance()

class SKRBACKENDEXPORT ProjectCommands : public QObject, public InvokableCommandGroupInterface
{
    Q_OBJECT
public:
    explicit ProjectCommands(QObject *parent, QUndoStack *undoStack);

    static ProjectCommands* instance()
    {
        return m_instance;
    }

    void save(int projectId);
    void closeProject(int projectId);
    void saveAs(int projectId, const QUrl &url);

    void setProjectName(int projectId, const QString &name);
    void setAuthor(int projectId, const QString &author);
    void setLanguageCode(int projectId, const QString &newLanguage);

    void loadProject(const QUrl &url);
signals:

private:
    static ProjectCommands *m_instance;
    QUndoStack *m_undoStack;

    // InvokableCommandGroupInterface interface
public:
    QString address() const override
    {
        return "project";
    }

    // InvokableCommandGroupInterface interface
public:
    Command *getCommand(const QString &action, const QVariantMap &parameters) override;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetProjectNameCommand : public Command
{
public:
    SetProjectNameCommand(int projectId, const QString &name);
    void undo();
    void redo();
private:
    QString m_newName, m_oldName;
    int m_projectId;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetAuthorCommand : public Command
{
public:
    SetAuthorCommand(int projectId, const QString &author);
    void undo();
    void redo();
private:
    QString m_newName, m_oldName;
    int m_projectId;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetLanguageCodeCommand : public Command
{
public:
    SetLanguageCodeCommand(int projectId, const QString &newLanguage);
    void undo();
    void redo();
private:
    QString m_oldLanguageCode, m_newLanguageCode;
    int m_projectId;
};

#endif // PROJECTCOMMANDS_H
