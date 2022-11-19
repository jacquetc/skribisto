#ifndef VIEW_H
#define VIEW_H

#include "toolbox.h"

#include "skribisto_desktop_common_global.h"
#include <QList>
#include <QMouseEvent>
#include <QToolBar>
#include <QUuid>
#include <QVBoxLayout>
#include <QWidget>

namespace Ui {
class View;
}

class SKRDESKTOPCOMMONEXPORT View : public QWidget {
  Q_OBJECT
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid NOTIFY uuidChanged)

public:
  explicit View(const QString &type, QWidget *parent = nullptr);
  ~View();
  virtual QList<Toolbox *> toolboxes() { return QList<Toolbox *>(); }

  void setIdentifiersAndInitialize(const TreeItemAddress &treeItemAddress = TreeItemAddress());

  const QString &type() const;

  int projectId() const;

  TreeItemAddress treeItemAddress() const;

  const QVariantMap &parameters() const;
  void setParameters(const QVariantMap &newParameters);
  virtual void applyParameters() {};


  QUuid uuid() const;
  void setUuid(const QUuid &newUuid);

  void setHistoryActions(QAction *goBackAction, QAction *goForwardAction);
  virtual void applyHistoryParameters(const QVariantMap &newParameters) {};

protected:
  virtual void initialize() = 0;

  void setCentralWidget(QWidget *widget);
  void setToolBar(QToolBar *toolBar);
  void setSecondToolBar(QToolBar *toolBar);
  void setSecondToolBarVisible(bool visible);

  virtual QVariantMap addOtherViewParametersBeforeSplit() {
    return QVariantMap();
  }
  //    void paintEvent(QPaintEvent *event);

protected slots:
  virtual void settingsChanged(const QHash<QString, QVariant> &newSettings){};

signals:
  void initialized(const TreeItemAddress &treeItemAddress);
    void aboutToBeDestroyed();
    void uuidChanged();
    void addToHistoryCalled(View* view, QVariantMap parameters);

private slots:
  void on_closeToolButton_clicked();

  void init();

private:
  Ui::View *ui;
  QUuid m_uuid;
  QString m_type;
  int m_projectId;
    TreeItemAddress m_treeItemAddress;
    QVariantMap m_parameters;
  QAction *m_goBackAction, *m_goForwardAction;

  // QWidget interface
protected:
  void mouseReleaseEvent(QMouseEvent *event) override;
};

#endif // VIEW_H
