#ifndef THEMEWIZARD_H
#define THEMEWIZARD_H

#include <QWizard>

namespace Ui {
class ThemeWizard;
}

class ThemeWizard : public QWizard
{
    Q_OBJECT

public:
    explicit ThemeWizard(QWidget *parent = nullptr);
    ~ThemeWizard();

private:
    Ui::ThemeWizard *ui;
};

#endif // THEMEWIZARD_H
