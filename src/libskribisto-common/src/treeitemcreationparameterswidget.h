#ifndef TREEITEMCREATIONPARAMETERSWIDGET_H
#define TREEITEMCREATIONPARAMETERSWIDGET_H

#include <QWidget>

class TreeItemCreationParametersWidget : public QWidget
{
    Q_OBJECT
public:
    explicit TreeItemCreationParametersWidget(QWidget *parent = nullptr);

    virtual QVariantMap getItemCreationProperties() const {
        return QVariantMap();
    }

signals:

};

#endif // TREEITEMCREATIONPARAMETERSWIDGET_H
