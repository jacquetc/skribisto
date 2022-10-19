#ifndef TAGCOMMANDS_H
#define TAGCOMMANDS_H

#include <QColor>
#include <QObject>
#include <QUndoCommand>
#include <QVariant>
#include "skribisto_backend_global.h"

#include "interfaces/invokablecommandgroupinterface.h"

#define tagCommands TagCommands::instance()

class SKRBACKENDEXPORT TagCommands : public QObject, public InvokableCommandGroupInterface
{
    Q_OBJECT
public:
    explicit TagCommands(QObject *parent, QUndoStack *undoStack);

    static TagCommands* instance()
    {
        return m_instance;
    }
    QUndoStack *undoStack() const;

    int addTag(int projectId, const QString &name, const QColor &tagColor, const QColor &textColor);
    int addTag(int projectId, const QString &name);
    int modifyTag(int projectId, int tagId, const QString &name, const QColor &tagColor, const QColor &textColor);
    QList<int> addSeveralTags(int projectId, QList<QString> nameList);
    void setTagName(int projectId, int tagId, const QString &name);
    void setTagColor(int projectId, int tagId, const QColor &color);
    void setTagTextColor(int projectId, int tagId, const QColor &color);
    void removeTag(int projectId, int tagId);
    void removeSeveralTags(int projectId, QList<int> tagIds);

signals:

private:
    static TagCommands *m_instance;
    QUndoStack *m_undoStack;

    // InvokableCommandGroupInterface interface
public:
    QString address() const override;
    Command *getCommand(const QString &action, const QVariantMap &parameters) override;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class AddTagCommand : public Command
{
public:
    AddTagCommand(int projectId, const QString &name);
    void undo();
    void redo();
    int result();
private:
    int m_projectId, m_newId;
    QString m_name;

};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetTagNameCommand : public Command
{
public:
    SetTagNameCommand(int projectId, int tagId, const QString &name);
    void undo();
    void redo();
private:
    QString m_newName, m_oldName;
    int m_projectId, m_tagId;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetTagColorCommand : public Command
{
public:
    SetTagColorCommand(int projectId, int tagId, const QColor &color);
    void undo();
    void redo();
private:
    QColor m_newColor, m_oldColor;
    int m_projectId, m_tagId;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetTagTextColorCommand : public Command
{
public:
    SetTagTextColorCommand(int projectId, int tagId, const QColor &color);
    void undo();
    void redo();
private:
    QColor m_newColor, m_oldColor;
    int m_projectId, m_tagId;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class RemoveTagCommand : public Command
{
public:
    RemoveTagCommand(int projectId, int tagId);
    void undo();
    void redo();
private:
    int m_projectId, m_tagId;
    QString m_name;
    QVariantMap m_savedTagValues;
    QList<int> m_relatedTreeItemIds;


};


#endif // TAGCOMMANDS_H
