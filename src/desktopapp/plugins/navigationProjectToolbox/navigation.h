#ifndef NAVIGATION_H
#define NAVIGATION_H

#include "toolbox.h"
#include <QWidget>

namespace Ui {
class Navigation;
}

class Navigation : public Toolbox
{
    Q_OBJECT

public:
    explicit Navigation(QWidget *parent = nullptr);
    ~Navigation();
    QString title() const {
        return tr("Navigation");
    }

private:
    Ui::Navigation *ui;
    QWidget * QWidget;
};

#endif // NAVIGATION_H
