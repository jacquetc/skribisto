#ifndef NEWTREEITEMDIALOG_H
#define NEWTREEITEMDIALOG_H

#include "treeitemcreationparameterswidget.h"
#include "skribisto_common_global.h"

#include <QDialog>
#include <QHash>
#include <QListWidgetItem>

namespace Ui {
class NewTreeItemDialog;
}

class SKRCOMMONEXPORT NewTreeItemDialog : public QDialog
{
    Q_OBJECT

public:

    enum ActionType {
        AddBefore,
        AddAfter,
        AddSubItem
    };
    Q_ENUM(ActionType)

    explicit NewTreeItemDialog(QWidget *parent = nullptr);
    ~NewTreeItemDialog();
    void setIdentifiers(int projectId = -1, int treeItemId = -1);

    NewTreeItemDialog::ActionType actionType() const;
    void setActionType(ActionType newActionType);

    TreeItemCreationParametersWidget *getCustomPropertiesWidget() const;
    void setCustomPropertiesWidget(TreeItemCreationParametersWidget *widget);
    void removeCustomPropetiesWidget();
    void reset();

private slots:
    void on_listWidget_currentItemChanged(QListWidgetItem *current, QListWidgetItem *previous);

private:
    Ui::NewTreeItemDialog *ui;
    NewTreeItemDialog::ActionType m_actionType;
    QHash<QString, TreeItemCreationParametersWidget *> m_typeWithparameterWidgetHash;
    TreeItemCreationParametersWidget *m_customPropertiesWidget;
    QString m_type;
    int m_projectId;
    int m_treeItemId;
    int m_numberToCreate;
};

#endif // NEWTREEITEMDIALOG_H
