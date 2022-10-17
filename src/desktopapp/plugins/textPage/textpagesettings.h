#ifndef TEXTPAGESETTINGS_H
#define TEXTPAGESETTINGS_H

#include <QWidget>
#include <settingspanel.h>

namespace Ui {
class TextPageSettings;
}

class TextPageSettings : public SettingsPanel
{
    Q_OBJECT

public:
    explicit TextPageSettings(QWidget *parent = nullptr);
    ~TextPageSettings();

private:
    Ui::TextPageSettings *centralWidgetUi;

    // SettingsPanel interface
public slots:
    void accept() override;
};

#endif // TEXTPAGESETTINGS_H
