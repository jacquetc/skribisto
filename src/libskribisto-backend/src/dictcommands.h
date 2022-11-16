#ifndef DICTCOMMANDS_H
#define DICTCOMMANDS_H

#include <QObject>
#include <QUndoCommand>
#include "skribisto_backend_global.h"
#include "interfaces/invokablecommandgroupinterface.h"

#define dictCommands DictCommands::instance()

class SKRBACKENDEXPORT DictCommands : public QObject, public InvokableCommandGroupInterface
{
    Q_OBJECT
public:
    explicit DictCommands(QObject *parent, QUndoStack *undoStack);

    static DictCommands* instance()
    {
        return m_instance;
    }
    QUndoStack *undoStack() const;
    void addWordToProjectDict(int projectId, const QString &word);

signals:
private:
    static DictCommands *m_instance;
    QUndoStack *m_undoStack;

    // InvokableCommandGroupInterface interface
public:
    Command *getCommand(const QString &action, const QVariantMap &parameters) override;
    QString address() const override
    {
        return "dict";
    }

};


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class AddWordToProjectDictCommand : public Command
{
public:
    AddWordToProjectDictCommand(int projectId, const QString &word);
    void undo();
    void redo();
private:
    QString m_word;
    int m_projectId;
};

#endif // DICTCOMMANDS_H
