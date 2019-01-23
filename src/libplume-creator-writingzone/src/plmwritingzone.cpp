#include "plmwritingzone.h"
#include "ui_plmwritingzone.h"

#include <QLayout>

PLMWritingZone::PLMWritingZone(QWidget *parent) : QWidget(parent),
    ui(new Ui::PLMWritingZone)
{
    ui->setupUi(this);

    m_fixedWidth = 600;

    // connect scrollbars:
    ui->verticalScrollBar->setValue(ui->richTextEdit->verticalScrollBar()->value());
    connect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::valueChanged,
            ui->verticalScrollBar, &QScrollBar::setValue);

    ui->verticalScrollBar->setMinimum(ui->richTextEdit->verticalScrollBar()->minimum());
    ui->verticalScrollBar->setMaximum(ui->richTextEdit->verticalScrollBar()->maximum());
    connect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::rangeChanged,
            ui->verticalScrollBar, &QScrollBar::setRange);

    connect(ui->verticalScrollBar,                 &QScrollBar::valueChanged,
            [ = ](int value) {
        disconnect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::rangeChanged,
                   ui->verticalScrollBar, &QScrollBar::setRange);
        disconnect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::valueChanged,
                   ui->verticalScrollBar, &QScrollBar::setValue);

        ui->richTextEdit->verticalScrollBar()->setValue(value);
        connect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::rangeChanged,
                ui->verticalScrollBar, &QScrollBar::setRange);
        connect(ui->richTextEdit->verticalScrollBar(), &QScrollBar::valueChanged,
                ui->verticalScrollBar, &QScrollBar::setValue);
    });


    // default :
    this->setHasMinimap(false);
    this->setHasScrollbar(false);
    this->setHasSideToolBar(false);
    this->setIsResizable(false);
    connect(ui->sizeHandle, &SizeHandle::moved, this, &PLMWritingZone::widenTextEdit);
}

// -----------------------------------------------------------------------------

PLMWritingZone::~PLMWritingZone()
{}

void PLMWritingZone::setUse(const QString& use)
{
    m_use = use;
}

bool PLMWritingZone::hasMinimap() const
{
    return m_hasMinimap;
}

void PLMWritingZone::setHasMinimap(bool hasMinimap)
{
    ui->minimap->setVisible(hasMinimap);
    m_hasMinimap = hasMinimap;
}

bool PLMWritingZone::hasScrollbar() const
{
    return m_hasScrollbar;
}

void PLMWritingZone::setHasScrollbar(bool hasScrollbar)
{
    m_hasScrollbar = hasScrollbar;
}

bool PLMWritingZone::hasSideToolBar() const
{
    return m_hasSideToolBar;
}

void PLMWritingZone::setHasSideToolBar(bool hasSideToolBar)
{
    m_hasSideToolBar = hasSideToolBar;
}

bool PLMWritingZone::isResizable() const
{
    return m_isResizable;
}

void PLMWritingZone::setIsResizable(bool isResizable)
{
    QSizePolicy policy =   ui->richTextEdit->sizePolicy();

    if (isResizable) {
        policy.setHorizontalPolicy(QSizePolicy::Maximum);
        ui->richTextEdit->setMinimumSize(QSize(100, 100));
        ui->richTextEdit->setMaximumWidth(m_fixedWidth);

        // ui->horizontalLayout->sets
    } else {
        policy.setHorizontalPolicy(QSizePolicy::Ignored);
        ui->richTextEdit->setMinimumSize(QSize(100, 100));
        ui->richTextEdit->setMaximumSize(QSize(QWIDGETSIZE_MAX, QWIDGETSIZE_MAX));
    }
    ui->richTextEdit->setSizePolicy(policy);

    m_isResizable = isResizable;
}

void PLMWritingZone::setFixedWidth(int width)
{
    m_fixedWidth = width;
}

bool PLMWritingZone::isMarkdown() const
{
    return m_isMarkdown;
}

void PLMWritingZone::setIsMarkdown(bool isMarkdown)
{
    m_isMarkdown = isMarkdown;
}

QString PLMWritingZone::markdownText() const
{
    return m_markdownText;
}

void PLMWritingZone::setMarkdownText(const QString& markdownText)
{
    m_markdownText = markdownText;
}

QString PLMWritingZone::htmlText() const
{
    return m_htmlText;
}

void PLMWritingZone::setHtmlText(const QString& htmlText)
{
    m_htmlText = htmlText;
}

void PLMWritingZone::widenTextEdit(int diff)
{
    ui->richTextEdit->setMaximumWidth(ui->richTextEdit->width() + diff * 2);
}

// -----------------------------------------------------------------------------
