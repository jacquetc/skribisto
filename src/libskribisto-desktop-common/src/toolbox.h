#ifndef TOOLBOX_H
#define TOOLBOX_H

#include "skribisto_desktop_common_global.h"
#include <QWidget>

class SKRDESKTOPCOMMONEXPORT Toolbox : public QWidget {
  Q_OBJECT

public:
  explicit Toolbox(QWidget *parent = nullptr) : QWidget(parent), m_projectId(-1), m_treeItemId(-1) {}
  virtual ~Toolbox() {}
  virtual QString title() const = 0;
  virtual QIcon icon() const = 0;
  virtual void initialize() = 0;

    virtual void setIdentifiersAndInitialize(int projectId = -1,
                                             int treeItemId = -1) {
        m_projectId = projectId;
        m_treeItemId = treeItemId;

        this->initialize();

        emit initialized(projectId, treeItemId);
    }
    int projectId() const;

    int treeItemId() const;

signals:
    void initialized(int projectId, int treeItemId);
    void aboutToBeDestroyed();


private:
    int m_projectId;
    int m_treeItemId;


};

inline int Toolbox::projectId() const
{
    return m_projectId;
}

inline int Toolbox::treeItemId() const
{
    return m_treeItemId;
}

#endif // TOOLBOX_H
