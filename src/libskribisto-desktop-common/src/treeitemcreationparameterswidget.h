#pragma once


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


