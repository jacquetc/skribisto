#ifndef COMBOBOX_H
#define COMBOBOX_H

#include <QComboBox>
#include <QTimer>
#include "skribisto_desktop_common_global.h"

class SKRDESKTOPCOMMONEXPORT ComboBox : public QComboBox
{
public:
    ComboBox(QWidget *parent = nullptr);




    // QWidget interface
    void shakePalette();
protected:
    void paintEvent(QPaintEvent *event) override;

private:
    QTimer *m_timer;

    // QWidget interface
protected:
    void showEvent(QShowEvent *event) override;
};

#endif // COMBOBOX_H
