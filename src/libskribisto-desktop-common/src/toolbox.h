#ifndef TOOLBOX_H
#define TOOLBOX_H

#include "skribisto_desktop_common_global.h"
#include <QWidget>

class SKRDESKTOPCOMMONEXPORT Toolbox : public QWidget {
  Q_OBJECT

public:
  explicit Toolbox(QWidget *parent = nullptr) : QWidget(parent) {}
  virtual ~Toolbox() {}
  virtual QString title() const = 0;
  virtual QIcon icon() const = 0;
};

#endif // TOOLBOX_H
