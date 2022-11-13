#ifndef BASICSETTINGSPANEL_H
#define BASICSETTINGSPANEL_H

#include <QWidget>

class BasicSettingsPanel : public QWidget
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
