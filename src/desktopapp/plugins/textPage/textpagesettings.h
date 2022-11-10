#ifndef TEXTPAGESETTINGS_H
#define TEXTPAGESETTINGS_H

#include <QWidget>
#include "settingssubpanel.h"

namespace Ui {
class TextPageSettings;
}

class TextPageSettings : public SettingsSubPanel
{
    Q_OBJECT

public:
    explicit TextPageSettings(QWidget *parent = nullptr);
    ~TextPageSettings();

private:
    Ui::TextPageSettings *centralWidgetUi;
    QHash<QString, QVariant> m_defaultValuesHash;

private slots:
    void updateSampleTextFormat();

    // SettingsPanel interface
public slots:
    void accept() override;
    void reset() override;

};

#endif // TEXTPAGESETTINGS_H
