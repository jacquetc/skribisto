#ifndef TAGMANAGERTOOLBOX_H
#define TAGMANAGERTOOLBOX_H

#include "toolbox.h"
#include <QWidget>
#include "skribisto_desktop_common_global.h"

namespace Ui {
class TagToolbox;
}

class SKRDESKTOPCOMMONEXPORT TagToolbox : public Toolbox {
  Q_OBJECT

public:
  explicit TagToolbox(QWidget *parent, int projectId, int treeItemId);
  ~TagToolbox();

public slots:
  void reloadTags();

private:
  Ui::TagToolbox *ui;
  int m_projectId;
  int m_treeItemId;

  // Toolbox interface
public:
  QString title() const override { return tr("Tags"); }
  QIcon icon() const override { return QIcon(":/icons/backup/tag.svg"); }

  // Toolbox interface
public:
  void initialize() override;
};

#endif // TAGMANAGERTOOLBOX_H
