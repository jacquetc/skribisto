#include "combobox.h"

#include "thememanager.h"


ComboBox::ComboBox(QWidget *parent): QComboBox(parent)
{

    QString buttonArgb = this->palette().button().color().name(QColor::NameFormat::HexArgb);
    QString buttonTextArgb = this->palette().buttonText().color().name(QColor::NameFormat::HexArgb);
    QString styleSheet = QString("QComboBox{ background: %1; color: %2; }").arg(buttonArgb, buttonTextArgb);
    //this->setStyleSheet(styleSheet);

    connect(themeManager, &ThemeManager::currentThemeTypeChanged, this, [&](){

        QString buttonArgb = this->palette().button().color().name(QColor::NameFormat::HexArgb);
        QString buttonTextArgb = this->palette().buttonText().color().name(QColor::NameFormat::HexArgb);
        QString styleSheet = QString("QComboBox{ background: %1; color: %2; }").arg(buttonArgb, buttonTextArgb);
        //this->setStyleSheet(styleSheet);
        shakePalette();

    }, Qt::QueuedConnection);

    m_timer = new QTimer(this);
    m_timer->setInterval(0);
    m_timer->setSingleShot(true);
    connect(m_timer, &QTimer::timeout, this, [&](){
    shakePalette();

    });


}

void ComboBox::shakePalette(){
    QString buttonArgb = this->palette().button().color().name(QColor::NameFormat::HexArgb);
    QString buttonTextArgb = this->palette().buttonText().color().name(QColor::NameFormat::HexArgb);
    QString styleSheet = QString("QComboBox{ background: %1; color: %2; }").arg(buttonArgb, buttonTextArgb);
    //this->setStyleSheet(styleSheet);

    for(int i = 40; i < 500 ; i++){
        this->setFixedWidth(i);
        this->repaint();

    }
    this->setFixedSize(QWIDGETSIZE_MAX, QWIDGETSIZE_MAX);
    this->update();
}

void ComboBox::paintEvent(QPaintEvent *event)
{
    QComboBox::paintEvent(event);
}


void ComboBox::showEvent(QShowEvent *event)
{
    m_timer->start();



}
