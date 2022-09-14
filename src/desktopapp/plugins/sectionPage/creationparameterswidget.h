#ifndef CREATIONPARAMETERSWIDGET_H
#define CREATIONPARAMETERSWIDGET_H

#include <QWidget>
#include "treeitemcreationparameterswidget.h"

namespace Ui {
class CreationParametersWidget;
}

class CreationParametersWidget : public TreeItemCreationParametersWidget
{
    Q_OBJECT

public:
    explicit CreationParametersWidget(QWidget *parent = nullptr);
    ~CreationParametersWidget();
    QVariantMap getItemCreationProperties() const override;
    void reset();

private:
    Ui::CreationParametersWidget *ui;
};

#endif // CREATIONPARAMETERSWIDGET_H
