#ifndef INSTALLNEWDICTWIZARD_H
#define INSTALLNEWDICTWIZARD_H

#include <QWizard>

namespace Ui {
class InstallNewDictWizard;
}

class InstallNewDictWizard : public QWizard
{
    Q_OBJECT

public:
    explicit InstallNewDictWizard(QWidget *parent = nullptr);
    ~InstallNewDictWizard();

private:
    Ui::InstallNewDictWizard *ui;
};

#endif // INSTALLNEWDICTWIZARD_H
