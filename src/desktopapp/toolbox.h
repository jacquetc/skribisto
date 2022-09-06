#ifndef TOOLBOX_H
#define TOOLBOX_H

#include <QWidget>

class Toolbox : public QWidget
{

public:
    explicit Toolbox(QWidget *parent = nullptr) : QWidget(parent) {}
    virtual ~Toolbox() {}
    virtual QString title() const = 0;

};

#endif // TOOLBOX_H
