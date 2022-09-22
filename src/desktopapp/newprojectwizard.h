#ifndef NEWPROJECTWIZARD_H
#define NEWPROJECTWIZARD_H

#include <QWizard>

namespace Ui {
class NewProjectWizard;
}

class NewProjectWizard : public QWizard
{
    Q_OBJECT

public:
    explicit NewProjectWizard(QWidget *parent = nullptr);
    ~NewProjectWizard();

private:
    Ui::NewProjectWizard *ui;
};

#endif // NEWPROJECTWIZARD_H
