#ifndef EMPTYVIEW_H
#define EMPTYVIEW_H

#include "skribisto_desktop_common_global.h"
#include "view.h"
#include <QWidget>

namespace Ui {
class EmptyView;
}

class SKRDESKTOPCOMMONEXPORT EmptyView : public View {
  Q_OBJECT

public:
  explicit EmptyView(QWidget *parent = nullptr);
  ~EmptyView();
  QList<Toolbox *> toolboxes();

protected:
  void initialize();

private:
  Ui::EmptyView *centralWidgetUi;
};

#endif // EMPTYVIEW_H
