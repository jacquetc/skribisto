/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: minimap.cpp                                                   *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#include "minimap_obsolete.h"

#include <QTextBlock>
#include <QOpenGLWidget>
#include <QDebug>
#include <QCryptographicHash>
#include <QByteArray>

Minimap::Minimap(QWidget *parent) : QGraphicsView(parent)
{
    m_scene = new QGraphicsScene(this);
    this->setAutoFillBackground(true);
    this->setViewport(new QOpenGLWidget());
    this->setScene(m_scene);
    this->setAlignment(Qt::AlignTop);
    this->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    this->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    m_navCursor = new Cursor();
    //m_navCursor->setGraphicsView(this);
    m_scene->addItem(m_navCursor);
    m_imgGenerator = new ImageGenerator();
    connect(m_imgGenerator, &ImageGenerator::imageGenerated, this, &Minimap::updateWith);
}

QTextEdit *Minimap::texEdit()
{
    return m_textEdit;
}

void Minimap::setTextEdit(QTextEdit *textEdit)
{
    m_textEdit = textEdit;
    connect(m_textEdit, &QTextEdit::textChanged, this, &Minimap::update);
    m_scrollbar = m_textEdit->verticalScrollBar();
    connect(m_scrollbar, &QScrollBar::valueChanged, this, &Minimap::changeCursorPos);
}

void Minimap::update()
{
    if (this->isCursorMoved()) {
        return;
    }

    this->generateImage(m_textEdit);
}

void Minimap::setScrollbarValue(int value)
{
    m_scrollbar->setValue(value);
}

void Minimap::resizeEvent(QResizeEvent *event)
{
    return QGraphicsView::resizeEvent(event);
}

void Minimap::generateImage(QTextEdit *textEdit)
{
    QTextDocument *doc = textEdit->document();
    QList<DocItem *> itemList;

    for (int i = 0; i < doc->blockCount() ; ++i) {
        QTextBlock block = doc->findBlockByNumber(i);
        DocItem *item = new DocItem(this);
        item->setBlockText(block.text());
        item->setTextFormat(block.blockFormat());
        item->setWidth(textEdit->viewport()->width());
        QTextLayout *layout = block.layout();
        QList<QTextLine> lines;

        if (layout) {
            for (int i = 0; i > layout->lineCount(); ++i) {
                lines.append(layout->lineAt(i));
            }
        }

        item->setLines(lines);
        itemList.append(item);
        m_imgGenerator->setItems(itemList);
        m_imgGenerator->start();
    }
}


void Minimap::updateWith(QPixmap pixmap)
{
    foreach (QGraphicsItem *gItem, m_graphicsItemList) {
        if (gItem->data(0).isValid() && gItem->data(0).toString() == "map") {
            m_scene->removeItem(gItem);
            m_graphicsItemList.removeOne(gItem);
            delete gItem;
        }
    }

    QGraphicsItem *item = m_scene->addPixmap(pixmap);
    item->setData(0, "map");
    item->setPos(0, 0);
    m_pixmap = pixmap;
    m_graphicsItemList.append(item);
    this->setFixedWidth(pixmap.width());
    //cursor :
    m_navCursor->setZValue(1);
//width :
    m_navCursor->setWidth(pixmap.width());
    //height :
    int textEditHeight = m_textEdit->height();
    int docHeight = m_textEdit->document()->size().height();
    int minimapHeight = m_pixmap.height();
    m_navCursor->setMinimapHeight(minimapHeight);
    double ratio = docHeight / textEditHeight;
    int cursorHeight = minimapHeight / ratio;
    m_navCursor->setHeight(cursorHeight);
// position
    this->changeCursorPos();
}

void Minimap::setCursorMoved(bool isCursorMoved)
{
    m_navCursor->setIsCursorMoved(isCursorMoved);
}

bool Minimap::isCursorMoved() const
{
    return m_navCursor->isCursorMoved();
}
void Minimap::changeCursorPos()
{
    if (this->isCursorMoved()) {
        return;
    }

    int scrollValue = m_scrollbar->value();
    int scrollMax = m_scrollbar->maximum();
    int availableHeightForCursor = m_pixmap.height() - m_navCursor->height();

    if (scrollMax != 0) {
        double posRatio = scrollValue / scrollMax;
        m_navCursor->setPosRatio(posRatio);
        m_navCursor->setPos(0, availableHeightForCursor * posRatio);
    } else {
        m_navCursor->setPos(0, 0);
    }
}

//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------


//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------

DocItem::DocItem(QObject *parent) : QObject(parent)
{
    // char size:
    m_charHeight = 2;
    m_charWidth = 1;
    m_spaceBetweenLines = 1;
}

DocItem::DocItem(DocItem *other)
{
    m_charHeight = other->width();
    m_hash = other->hash();
    m_blockText = other->blockText();
    m_textFormat = other->textFormat();
    m_image = other->image();
    m_lines = other->getLines();
}

DocItem::~DocItem()
{
}

QByteArray DocItem::hash()
{
    // generate hash from text and format
    if (m_hash.isEmpty()) {
        QByteArray byteArray;
        QDataStream dataStream(byteArray);
        dataStream << m_blockText << m_width << m_textFormat;
        m_hash = QCryptographicHash::hash(byteArray, QCryptographicHash::Md5);
    }

    return m_hash;
}

QString DocItem::blockText() const
{
    return m_blockText;
}

void DocItem::setBlockText(const QString &blockText)
{
    m_blockText = blockText;
}

QTextFormat DocItem::textFormat() const
{
    return m_textFormat;
}

void DocItem::setTextFormat(const QTextFormat &textFormat)
{
    m_textFormat = textFormat;
}

int DocItem::index() const
{
    return m_index;
}

void DocItem::setIndex(int index)
{
    m_index = index;
}

QImage DocItem::image()
{
    if (!m_image.isNull()) {
        m_image = this->generateImage();
    }

    return m_image;
}

int DocItem::width() const
{
    return m_width;
}

void DocItem::setWidth(int width)
{
    m_width = width;
}

QList<QTextLine> DocItem::getLines() const
{
    return m_lines;
}

void DocItem::setLines(const QList<QTextLine> &value)
{
    m_lines = value;
}

//-----------------------------------------------------------------
void DocItem::drawChar(QPainter &painter, int position, const QColor &penColor)
{
    painter.setPen(QPen(penColor));
    QRect rect(position * m_charWidth + position, m_spaceBetweenLines, m_charWidth, m_charHeight);
    painter.drawRect(rect);
    painter.fillRect(rect, penColor);
}
//-----------------------------------------------------------------
QImage DocItem::generateImage()
{
// determine the range of tab:
    int tabValue = 80;
    QString text = "m";
    QPixmap pixmap(100, 100);
    QPainter painter(&pixmap);
    QFontMetrics fontMetrics = painter.fontMetrics();
    int width = fontMetrics.width(text);
    painter.end();
    int numberOfCharsInOneTab = int(tabValue / width);
// prepare :
    text = this->preparedText();
    QStringList splitText = text.split("N");
    QList<QImage> pixmapList;

    foreach (const QString &text, splitText) {
        int i = 0;
        int lenght = text.count();

        if (lenght == 0) { // empty line
            lenght = 1;
        }

        QImage img(lenght * m_charWidth + lenght, m_charHeight * 2, QImage::Format_ARGB32_Premultiplied);
        img.fill(Qt::white);
        QPainter painter(&img);
        int drift = 0;

        for ( int i = 0; i < text.size(); ++i) {
            QChar character = text.at(i);

            if (character == "c") {
                this->drawChar(painter, i + drift, Qt::black);
            }

            if (character == "t") {
                for (int k = 0; k < numberOfCharsInOneTab; ++k) {
                    this->drawChar(painter, i + drift, Qt::transparent);
                    drift += 1;
                }
            }

            if (character == "_") {
                this->drawChar(painter, i + drift , Qt::transparent);
            }
        }

        painter.end();
        pixmapList.append(img);
    }

// find max width:
    QList<int> widthList;

    foreach (const QImage &pix, pixmapList) {
        widthList.append(pix.width());
    }

    int maxWidth = 0;

    foreach (int width , widthList) {
        maxWidth = qMax(width, maxWidth);
    }

    QImage image = QImage(maxWidth, pixmapList.at(0).height() * pixmapList.count(), QImage::Format_ARGB32_Premultiplied);
    image.fill(Qt::transparent);
    QPainter painter2(&image);

    foreach (const QImage &pix, pixmapList) {
        int y = pixmapList.at(0).height() * pixmapList.indexOf(pix);
        painter2.drawImage(0, y, pix);
    }

    painter2.end();
    return image;
}
//-----------------------------------------------------------------
QString DocItem::preparedText() const
{
    QList<int> returnPosList;

    foreach (const QTextLine &line, m_lines) {
        returnPosList.append(line.textStart());
    }

//    int indent = m_textFormat.textIndent();
//    int leftMargin = m_textFormat.leftMargin();
//    int rightMargin = m_textFormat.rightMargin();
    QString text = m_blockText;
    QString final = "";

    for (int i = 0; i < text.count(); ++i) {
        if (returnPosList.contains(i) && i != 0) {
            final += "N"; // line return
        }

        if (text.at(i) == QChar::Tabulation) {
            final += "t"; // tab
        }

        if (text.at(i).isLetterOrNumber()) {
            final += "c"; // common char
        } else {
            final += "_"; // space
        }
    }

    return final;
}

//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------

ImageGenerator::ImageGenerator()
{
    m_maxWidthFound = 30;
}

void ImageGenerator::setItems(const QList<DocItem *> &itemList)
{
    m_itemList = itemList;
}

void ImageGenerator::run()
{
    this->sleep(1);

// create hash dict of current list

    foreach (DocItem *docItem, m_itemList) {
        m_hashItemDict.insert(docItem->hash(), docItem);
    }

// compare and swap between lists if new item is not generated

    foreach (const QByteArray &hash, m_hashItemDict.keys()) {
        if (m_oldHashItemDict.contains(hash)) {
// swap to avoid generating uselessly :
            m_hashItemDict.insert(hash, m_oldHashItemDict.value(hash));
        } else {
            // generate image
            QImage image = m_hashItemDict.value(hash)->image();
            Q_UNUSED(image)
        }
    }

// append generated images
    QList<QImage> pixList;

    foreach (DocItem *docItem, m_itemList) {
        DocItem *usableItem = m_hashItemDict.value(docItem->hash());
        pixList.append(usableItem->image());
    }

    //draw image into one
    QImage image(20, 20, QImage::Format_ARGB32_Premultiplied);
    image.fill(Qt::transparent);
    QPoint point(0, 0);
    // find max width:
    QList<int> widthList;

    foreach (const QImage &pix, pixList) {
        widthList.append(pix.width());
    }

    widthList.append(m_maxWidthFound);
    int maxWidth = 0;

    foreach (int width , widthList) {
        maxWidth = qMax(width, maxWidth);
    }

    m_maxWidthFound = maxWidth;
    QImage previousImage;

    foreach (const QImage &pix, pixList) {
        previousImage = image;
        image = QImage(m_maxWidthFound, image.height(
                       ) + pix.height(), QImage::Format_ARGB32_Premultiplied);
        QPainter pixPaint(&image);
        image.fill(Qt::white);
        pixPaint.drawImage(0, 0, previousImage);
        pixPaint.drawImage(point, pix);
        point = QPoint(0, point.y() + pix.height());
        pixPaint.end();
    }

    QPixmap finalPixmap = QPixmap::fromImage(image, Qt::AutoColor);
    emit imageGenerated(finalPixmap);
    m_oldHashItemDict = m_hashItemDict;
    m_oldItemList = m_itemList;
}
