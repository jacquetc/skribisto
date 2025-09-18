#pragma once


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
    void reset() override;

private:
    Ui::CreationParametersWidget *ui;
};


