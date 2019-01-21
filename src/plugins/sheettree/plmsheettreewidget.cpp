#include "plmsheettreewidget.h"
#include "plmwidget.h"
#include "plmdata.h"
#include <QToolButton>
#include <QVBoxLayout>

PLMSheetTreeWidget::PLMSheetTreeWidget(QObject *parent) : QObject(parent),
    m_name("SheetTree")
{
    this->setProperty("name", m_name);
}

// -------------------------------------------------------------------

PLMSheetTreeWidget::~PLMSheetTreeWidget()
{}

// -------------------------------------------------------------------

QString PLMSheetTreeWidget::use() const
{
    return m_name;
}

// -------------------------------------------------------------------


void PLMSheetTreeWidget::instanciate(QWidget *parent)
{
    // dockBody

    m_dockBody = new PLMWidget(parent);


    // dockHeader


    m_dockHeader = new QWidget(parent);
    QHBoxLayout *hlayout = new QHBoxLayout();

    m_dockHeader->setLayout(hlayout);
    QLabel *label = new QLabel();
    label->setText("sheets");
    hlayout->addWidget(label);
    QToolButton *button = new QToolButton();
    button->setText("filtre");
    hlayout->addWidget(button);

    // connections
}

// -------------------------------------------------------------------

PLMBaseDockWidget * PLMSheetTreeWidget::dockBodyWidget(QWidget *parent)
{
    if (!m_dockBody) this->instanciate(parent);

    Q_ASSERT(!m_dockBody.isNull());
    return m_dockBody.data();
}

// -------------------------------------------------------------------
QWidget * PLMSheetTreeWidget::dockHeaderWidget(QWidget *parent)
{
    if (!m_dockHeader) this->instanciate(parent);

    Q_ASSERT(!m_dockHeader.isNull());
    return m_dockHeader.data();
}

PLMDockWidgetInterface * PLMSheetTreeWidget::clone() const
{
    return new PLMSheetTreeWidget(nullptr);
}
