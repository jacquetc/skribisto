#include "tagitemdelegate.h"

#include <QModelIndex>
#include <QPainter>
#include <skrdata.h>

TagItemDelegate::TagItemDelegate(QObject *parent)
    : QStyledItemDelegate{parent}
{

}


void TagItemDelegate::paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const
{


    QString textBrush(index.data(Qt::UserRole + 1).toString());
    QString tagBrush(index.data(Qt::UserRole + 2).toString());
    QRect rect = option.rect;

    painter->save();

    painter->setRenderHint(QPainter::Antialiasing, true);

    painter->setPen(Qt::NoPen);

    // selection highlight
    if (option.state & QStyle::State_Selected){
        painter->setBrush(option.palette.highlight());
        painter->drawRect(option.rect);
    }




    // tag body
    painter->translate(option.rect.x(), option.rect.y());

    painter->setBrush(QColor(tagBrush));

    const QList<QPoint> polygonPoints {
        QPoint(20, rect.height() - 1),
        QPoint(1, rect.height() / 2),
        QPoint(20, 1),
        QPoint(rect.width(), 1),
        QPoint(rect.width(), rect.height() - 1),

    };


    QPolygon tagPolygon(polygonPoints);

    painter->setPen(QPen(Qt::black, 1));
    painter->drawPolygon(tagPolygon, Qt::WindingFill);
    qDebug() << "tagPolygon.boundingRect()" << tagPolygon.boundingRect() ;
    // hole

    painter->drawEllipse(QPoint(14, rect.height() / 2), 3, 3);
    painter->setCompositionMode(QPainter::CompositionMode_Clear);
    painter->drawEllipse(QPoint(14, rect.height() / 2), 2, 2);
    painter->setCompositionMode(QPainter::CompositionMode_SourceOver);


    //qDebug() << "length" << rect.width();

    // text:
    painter->setPen(QPen(QColor(textBrush), 1));
    QRect boundingRect;
    painter->drawText(QRect(20, 0, rect.width() - 20, rect.height())
                      , Qt::AlignVCenter | Qt::TextWordWrap
                      , index.data(Qt::DisplayRole).toString(),
                      &boundingRect);




    painter->restore();

}



QSize TagItemDelegate::sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    QSize defaultSize = QStyledItemDelegate::sizeHint(option,index);

    qDebug() << defaultSize;

    QImage image(200, 200, QImage::Format_ARGB32);
    QPainter painter(&image);
    painter.setPen(QPen(Qt::black, 1));
    QRect boundingRect;
    painter.drawText(QRect(0, 0, defaultSize.width(), 25)
                      , Qt::AlignVCenter | Qt::TextWordWrap
                      , index.data(Qt::DisplayRole).toString(),
                      &boundingRect);

    qDebug() << "boundingRect" << boundingRect;

    return QSize(1, boundingRect.height() + 4);

}
