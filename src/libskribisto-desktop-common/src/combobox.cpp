#include "combobox.h"

#include "thememanager.h"

#include <QTimer>

ComboBox::ComboBox(QWidget *parent): QComboBox(parent)
{

    connect(themeManager, &ThemeManager::currentThemeTypeChanged, this, [&](){

        QString buttonArgb = this->palette().button().color().name(QColor::NameFormat::HexArgb);
        QString buttonTextArgb = this->palette().buttonText().color().name(QColor::NameFormat::HexArgb);
        QString styleSheet = QString("QComboBox{ background: %1; color: %2; }").arg(buttonArgb).arg(buttonTextArgb);
        this->setStyleSheet(styleSheet);

    }, Qt::QueuedConnection);
}

void ComboBox::paintEvent(QPaintEvent *event)
{

    QComboBox::paintEvent(event);
}
