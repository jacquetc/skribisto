#ifndef COMBOBOX_H
#define COMBOBOX_H

#include <QComboBox>

class ComboBox : public QComboBox
{
public:
    ComboBox(QWidget *parent = nullptr);




    // QWidget interface
protected:
    void paintEvent(QPaintEvent *event) override;
};

#endif // COMBOBOX_H
