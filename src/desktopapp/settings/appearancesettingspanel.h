#ifndef APPEARANCESETTINGSPANEL_H
#define APPEARANCESETTINGSPANEL_H

#include "basicsettingspanel.h"
#include <QWidget>

namespace Ui {
class AppearanceSettingsPanel;
}

class AppearanceSettingsPanel : public BasicSettingsPanel
{
    Q_OBJECT

public:
    explicit AppearanceSettingsPanel(QWidget *parent = nullptr);
    ~AppearanceSettingsPanel();

    void accept() override;
    void reset() override;
private:
    Ui::AppearanceSettingsPanel *ui;
};

#endif // APPEARANCESETTINGSPANEL_H
