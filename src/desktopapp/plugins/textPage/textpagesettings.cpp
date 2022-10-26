#include "textpagesettings.h"
#include "ui_textpagesettings.h"

TextPageSettings::TextPageSettings(QWidget *parent) :
    SettingsSubPanel(parent),
    centralWidgetUi(new Ui::TextPageSettings)
{
    QWidget *widget = new QWidget;
    centralWidgetUi->setupUi(widget);
    setCentralWidget(widget);
}

TextPageSettings::~TextPageSettings()
{
    delete centralWidgetUi;
}


void TextPageSettings::accept()
{
}
