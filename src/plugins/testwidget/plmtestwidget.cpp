#include "plmtestwidget.h"
#include "plmdata.h"
#include <QVBoxLayout>

PLMTestWidget::PLMTestWidget(QObject *parent) : QObject(parent),
                                                m_name("testWidget")
{
    this->setProperty("name", m_name);
    connect(plmdata->projectHub(), &PLMProjectHub::projectLoaded, this, &PLMTestWidget::setText);
}

// -------------------------------------------------------------------

PLMTestWidget::~PLMTestWidget()
{}

// -------------------------------------------------------------------

QString PLMTestWidget::use() const
{
    return m_name;
}


void PLMTestWidget::setText()
{
    m_label->setText("loadedd");
}
// -------------------------------------------------------------------

PLMBaseWidget *PLMTestWidget::dockBodyWidget(QWidget *parent)
{
    PLMBaseWidget *widget = new PLMBaseWidget(parent);
    QVBoxLayout *layout = new QVBoxLayout();
    widget->setLayout(layout);
    m_label = new QLabel();
    m_label->setText("label !");
    layout->addWidget(m_label);

    return widget;
}


// -------------------------------------------------------------------
