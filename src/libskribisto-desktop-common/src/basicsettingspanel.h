#ifndef BASICSETTINGSPANEL_H
#define BASICSETTINGSPANEL_H

#include <QWidget>
#include "skribisto_desktop_common_global.h"

class SKRDESKTOPCOMMONEXPORT BasicSettingsPanel : public QWidget
{
    Q_OBJECT
public:
    explicit BasicSettingsPanel(QWidget *parent = nullptr);
    virtual void accept();
    virtual void reset();

  signals:
    void accepted();
    void reseted();

signals:

};

#endif // BASICSETTINGSPANEL_H
