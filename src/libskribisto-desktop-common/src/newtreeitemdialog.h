#ifndef NEWTREEITEMDIALOG_H
#define NEWTREEITEMDIALOG_H

#include "skribisto_desktop_common_global.h"
#include "treeitemaddress.h"
#include "treeitemcreationparameterswidget.h"

#include <QDialog>
#include <QHash>
#include <QListWidgetItem>

namespace Ui {
class NewTreeItemDialog;
}

class SKRDESKTOPCOMMONEXPORT NewTreeItemDialog : public QDialog {
  Q_OBJECT

public:
  enum ActionType { AddBefore, AddAfter, AddSubItem };
  Q_ENUM(ActionType)

  explicit NewTreeItemDialog(QWidget *parent = nullptr);
  ~NewTreeItemDialog();
  void setIdentifiers(const TreeItemAddress &treeItemAddress);

  NewTreeItemDialog::ActionType actionType() const;
  void setActionType(ActionType newActionType);

  TreeItemCreationParametersWidget *getCustomPropertiesWidget() const;
  void setCustomPropertiesWidget(TreeItemCreationParametersWidget *widget);
  void removeCustomPropertiesWidget();
  void reset();

private slots:
  void on_listWidget_currentItemChanged(QListWidgetItem *current,
                                        QListWidgetItem *previous);

private:
  Ui::NewTreeItemDialog *ui;
  NewTreeItemDialog::ActionType m_actionType;
  QHash<QString, TreeItemCreationParametersWidget *>
      m_typeWithparameterWidgetHash;
  TreeItemCreationParametersWidget *m_customPropertiesWidget;
  QString m_type;
  TreeItemAddress m_treeItemAddress;
  int m_numberToCreate;
};

class EmptyCreationParametersWidget : public TreeItemCreationParametersWidget {
  Q_OBJECT

public:
  EmptyCreationParametersWidget(QWidget *parent = nullptr)
      : TreeItemCreationParametersWidget(parent) {}

  void reset() {}
};

#endif // NEWTREEITEMDIALOG_H
