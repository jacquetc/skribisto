#ifndef INSTALLNEWDICTWIZARD_H
#define INSTALLNEWDICTWIZARD_H

#include <QWizard>
#include "skribisto_common_global.h"

namespace Ui {
class InstallNewDictWizard;
}

class SKRCOMMONEXPORT InstallNewDictWizard : public QWizard
{
    Q_OBJECT

public:
    explicit InstallNewDictWizard(QWidget *parent = nullptr);
    ~InstallNewDictWizard();

private:
    Ui::InstallNewDictWizard *ui;
};

#endif // INSTALLNEWDICTWIZARD_H
