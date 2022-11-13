#include "basicsettingspanel.h"

BasicSettingsPanel::BasicSettingsPanel(QWidget *parent)
    : QWidget{parent}
{

}

void BasicSettingsPanel::accept() { emit this->accepted(); }
void BasicSettingsPanel::reset() { emit this->reseted(); }
