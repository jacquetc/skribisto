#include "plmwritingzone.h"
#include "ui_plmwritingzone.h"


PLMWritingZone::PLMWritingZone(QWidget *parent) : QWidget(parent),
    ui(new Ui::PLMWritingZone)
{
    ui->setupUi(this);
    // default :
    this->setHasMinimap(false);
    this->setHasScrollbar(false);
    this->setHasSideToolBar(false);
    this->setIsResizable(false);
    connect(ui->sizeHandle, &SizeHandle::moved, this, &PLMWritingZone::widenTextEdit);
}


//-----------------------------------------------------------------------------

PLMWritingZone::~PLMWritingZone()
{
}

bool PLMWritingZone::hasMinimap() const
{
    return m_hasMinimap;
}
void PLMWritingZone::setHasMinimap(bool hasMinimap)
{
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
    m_isResizable = isResizable;
}

void PLMWritingZone::setWidth(int width)
{
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

void PLMWritingZone::setMarkdownText(const QString &markdownText)
{
    m_markdownText = markdownText;
}

QString PLMWritingZone::htmlText() const
{
    return m_htmlText;
}

void PLMWritingZone::setHtmlText(const QString &htmlText)
{
    m_htmlText = htmlText;
}

void PLMWritingZone::widenTextEdit(int diff)
{
    ui->richTextEdit->setMaximumWidth(ui->richTextEdit->width() + diff * 2);
}

//-----------------------------------------------------------------------------
