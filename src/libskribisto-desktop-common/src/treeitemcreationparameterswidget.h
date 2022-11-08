#ifndef TREEITEMCREATIONPARAMETERSWIDGET_H
#define TREEITEMCREATIONPARAMETERSWIDGET_H

#include "skribisto_desktop_common_global.h"
#include <QWidget>
#include <QVariantMap>

class SKRDESKTOPCOMMONEXPORT TreeItemCreationParametersWidget : public QWidget {
  Q_OBJECT
public:
  explicit TreeItemCreationParametersWidget(QWidget *parent = nullptr);

  virtual QVariantMap getItemCreationProperties() const {
    return QVariantMap();
  }

  virtual void reset() = 0;

signals:
};

#endif // TREEITEMCREATIONPARAMETERSWIDGET_H
