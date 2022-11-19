#ifndef VIEWMANAGER_H
#define VIEWMANAGER_H

#include "skribisto_desktop_common_global.h"
#include "view.h"

#include <QObject>
#include <QSplitter>
#include <QUuid>
#include "viewholder.h"

class ViewSplitter;
class SKRDESKTOPCOMMONEXPORT ViewManager : public QObject {
  Q_OBJECT
public:
    explicit ViewManager(QObject *parent, QWidget *viewWidget, bool restoreViewEnabled);
    ~ViewManager();
  void openSpecificView(const QString &pageType, const TreeItemAddress &treeItemAddress);

  ViewHolder *split(ViewHolder *viewHolder, Qt::Orientation orientation = Qt::Horizontal);
  void removeSplit(ViewHolder *viewHolder);
  View *currentView();
  ViewHolder *currentViewHolder();
  ViewHolder *nextViewHolder(ViewHolder *viewHolder);
  void addViewParametersBeforeCreation(const QVariantMap &parameters);
  void removeSplitWithView(View *view);

public slots:
  void openViewAtCurrentViewHolder(const QString &type, const TreeItemAddress &treeItemAddress = TreeItemAddress());
  void openViewInAnotherViewHolder(const QString &type, const TreeItemAddress &treeItemAddress = TreeItemAddress());
  View *openViewAt(ViewHolder *atViewHolder, const QString &type, const TreeItemAddress &treeItemAddress);

  View *splitForSamePage(View *view, Qt::Orientation orientation);

  void clear();
  void saveSplitterStructure();

signals:
  void currentViewChanged(View *view);
  void currentViewHolderChanged(ViewHolder *viewHolder);
  void aboutToRemoveView(View *view);

private slots:
  void init();
  void determineCurrentView();

private:
  QWidget *m_viewWidget;
  QList<ViewHolder *> m_viewHolderList;
  ViewSplitter *m_rootSplitter;
  QVariantMap m_parameters;
  View *m_currentView;
  ViewHolder *m_currentViewHolder;

private:
  void extracted(QList<QVariantHash> &variantHashList);
  void restoreSplitterStructure();
  QList<QVariantHash> saveSplitterRecursively(ViewSplitter *viewSplitter) const;
};

//---------------------------------------------------------------
//---------------------------------------------------------------

class ViewSplitter : public QSplitter {

    Q_OBJECT
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid NOTIFY uuidChanged)
public:
  explicit ViewSplitter(Qt::Orientation orientation, QWidget *parent = nullptr);

  ViewHolder *getViewHolder(int index);
  bool isFirstSplitASplitter();
  ViewSplitter *getSplitter(int index);
  bool isSecondSplitASplitter();


  QList<ViewSplitter *> listAllSplittersRecursively();


  QUuid uuid() const;
  void setUuid(const QUuid &newUuid);

signals:

  void uuidChanged();

private:
  QUuid m_uuid;

};


#endif // VIEWMANAGER_H
