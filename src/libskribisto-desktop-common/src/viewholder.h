#ifndef VIEWHOLDER_H
#define VIEWHOLDER_H

#include "view.h"

#include <QUuid>
#include <QWidget>

namespace Ui {
class ViewHolder;
}

class ViewHolder : public QWidget
{
    Q_OBJECT

public:
    explicit ViewHolder(QWidget *parent = nullptr);
    ~ViewHolder();

    QList<View *> viewList() const;
    void addView(View *view);
    void removeView(View *view);
    void removeViews(int projectId, int treeItemId = -1);
    void clear();
    View *currentView() const;
    void setCurrentView(View *view);

    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

signals:
    void viewtoBeAdded(View *view);
    void viewAdded(View *view);
    void viewToBeRemoved(View *view);
    void viewRemoved();
    void aboutToBeDestroyed();
    void uuidChanged();

private:
    Ui::ViewHolder *ui;
    QUuid m_uuid;
    QList<View *> m_viewList;

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid NOTIFY uuidChanged)
};

#endif // VIEWHOLDER_H
