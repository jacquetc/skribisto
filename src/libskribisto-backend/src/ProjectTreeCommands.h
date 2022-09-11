#ifndef PROJECTTREECOMMANDS_H
#define PROJECTTREECOMMANDS_H

#include "projecttreemodel.h"
#include "skribisto_backend_global.h"

#include <QObject>
#define projectTreeCommands ProjectTreeCommands::instance()

class SKRBACKENDEXPORT ProjectTreeCommands : public QObject
{
    Q_OBJECT
public:
    explicit ProjectTreeCommands(QObject *parent, QUndoStack *undoStack, ProjectTreeModel *treeModel);

    static ProjectTreeCommands* instance()
    {
        return m_instance;
    }

    QUndoStack *undoStack() const;

    void addItemAfter(int projectId, int targetId, const QString &type, const QVariantMap &properties = QVariantMap());
    void addItemBefore(int projectId, int targetId, const QString &type, const QVariantMap &properties = QVariantMap());
    void addSubItem(int projectId, int targetId, const QString &type, const QVariantMap &properties = QVariantMap());


    void addSeveralSubItems(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties);
    void addSeveralItemsAfter(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties);
    void addSeveralItemsBefore(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties);

    void setItemProperties(int projectId, int targetId, const QVariantMap &properties, bool isSystem = true);


signals:
private:
    static ProjectTreeCommands *m_instance;
QUndoStack *m_undoStack;
ProjectTreeModel *m_treeModel;
};

#endif // PROJECTTREECOMMANDS_H
