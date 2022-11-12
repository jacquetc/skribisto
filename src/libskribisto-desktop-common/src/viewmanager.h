#ifndef VIEWMANAGER_H
#define VIEWMANAGER_H

#include "skribisto_desktop_common_global.h"
#include "view.h"

#include <QObject>
#include <QSplitter>

class ViewSplitter;
class SKRDESKTOPCOMMONEXPORT ViewManager : public QObject {
  Q_OBJECT
public:
    explicit ViewManager(QObject *parent, QWidget *viewWidget, bool restoreViewEnabled);
    ~ViewManager();
  void openSpecificView(const QString &pageType, int projectId, int treeItemId);

  View *split(View *view, Qt::Orientation orientation = Qt::Horizontal);
  void removeSplit(View *view);
  View *currentView();
  View *nextView(View *view);
  void addViewParametersBeforeCreation(const QVariantMap &parameters);

public slots:
  void openViewAtCurrentView(const QString &type, int projectId = -1,
                             int treeItemId = -1);
  void openViewInAnotherView(const QString &type, int projectId = -1,
                             int treeItemId = -1);
  View *openViewAt(View *atView, const QString &type, int projectId = -1,
                   int treeItemId = -1);

  View *splitForSamePage(View *view, Qt::Orientation orientation);

  void clear();
  void saveSplitterStructure();

signals:
  void currentViewChanged(View *view);
  void aboutToRemoveView(View *view);

private slots:
  void init();
  void determineCurrentView();

private:
  QWidget *m_viewWidget;
  QList<View *> m_viewList;
  ViewSplitter *m_rootSplitter;
  QVariantMap m_parameters;
  View *m_currentView;

private:
  void extracted(QList<QVariantHash> &variantHashList);
  void restoreSplitterStructure();
  QList<QVariantHash> saveSplitterRecursively(ViewSplitter *viewSplitter) const;
};

//---------------------------------------------------------------
//---------------------------------------------------------------

class ViewSplitter : public QSplitter {

    Q_OBJECT
    Q_PROPERTY(QString uuid READ uuid WRITE setUuid NOTIFY uuidChanged)
public:
  explicit ViewSplitter(Qt::Orientation orientation, QWidget *parent = nullptr);

  void setView(int index, View *view);
  View *getView(int index);

  ViewSplitter *addChildSplitter(int index);

  bool isFirstSplitASplitter();
  ViewSplitter *getSplitter(int index);
  bool isSecondSplitASplitter();


  QList<ViewSplitter *> listAllSplittersRecursively();

  QString uuid() const;
  void setUuid(const QString &newUuid);

signals:
  void uuidChanged();

private:
  QString m_uuid;

};


#endif // VIEWMANAGER_H
