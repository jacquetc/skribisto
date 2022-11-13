#ifndef INSTALLNEWDICTWIZARD_H
#define INSTALLNEWDICTWIZARD_H

#include "skribisto_desktop_common_global.h"
#include <QWizard>

namespace Ui {
class InstallNewDictWizard;
}

class SKRDESKTOPCOMMONEXPORT InstallNewDictWizard : public QWizard {
  Q_OBJECT

public:
  explicit InstallNewDictWizard(QWidget *parent = nullptr);
  ~InstallNewDictWizard();

private:
  Ui::InstallNewDictWizard *ui;
};

#endif // INSTALLNEWDICTWIZARD_H
