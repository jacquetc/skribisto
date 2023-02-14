#ifndef TOOLBOX_H
#define TOOLBOX_H

#include "desktopapplication.h"
#include "skribisto_desktop_common_global.h"
#include "treeitemaddress.h"
#include <QWidget>

class SKRDESKTOPCOMMONEXPORT Toolbox : public QWidget {
  Q_OBJECT

public:
  explicit Toolbox(QWidget *parent = nullptr) : QWidget(parent), m_projectId(-1), m_treeItemAddress(TreeItemAddress()) {


        // settings:

        connect(static_cast<DesktopApplication *>(qApp), &DesktopApplication::settingsChanged, this, &Toolbox::applySettingsChanges);

    }
  virtual ~Toolbox() {}
  virtual QString title() const = 0;
  virtual QIcon icon() const = 0;
  virtual void initialize() = 0;

    virtual void setIdentifiersAndInitialize(const TreeItemAddress &treeItemAddress =  TreeItemAddress()) {
        m_projectId = treeItemAddress.projectId;
        m_treeItemAddress = treeItemAddress;

        this->initialize();

        emit initialized(treeItemAddress);
    }
    int projectId() const;

    TreeItemAddress treeItemAddress() const;

protected:
    virtual void applySettingsChanges(const QHash<QString, QVariant> &newSettings){};


signals:
    void initialized(const TreeItemAddress &treeItemAddress);
    void aboutToBeDestroyed();


private:
    int m_projectId;
    TreeItemAddress m_treeItemAddress;


};

inline int Toolbox::projectId() const
{
    return m_projectId;
}

inline TreeItemAddress Toolbox::treeItemAddress() const
{
    return m_treeItemAddress;
}

#endif // TOOLBOX_H
