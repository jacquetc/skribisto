#ifndef SETTINGSPANEL_H
#define SETTINGSPANEL_H

#include <QWidget>
#include "skribisto_common_global.h"

namespace Ui {
class SettingsPanel;
}

class SKRCOMMONEXPORT SettingsPanel : public QWidget
{
    Q_OBJECT

public:
    explicit SettingsPanel(QWidget *parent = nullptr);
    ~SettingsPanel();
protected:
    void setCentralWidget(QWidget *widget);

private:
    Ui::SettingsPanel *ui;
};

#endif // SETTINGSPANEL_H
