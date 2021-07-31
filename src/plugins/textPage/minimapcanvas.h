#ifndef MINIMAPCANVAS_H
#define MINIMAPCANVAS_H

#include <QQuickPaintedItem>
#include <QQuickTextDocument>
#include <QTextCursor>

class MinimapCanvas : public QQuickPaintedItem {
    Q_OBJECT
    Q_PROPERTY(
        QQuickTextDocument *
        textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

    //    Q_PROPERTY(QString fontFamily READ fontFamily WRITE setFontFamily
    // NOTIFY formatChanged)
    //    Q_PROPERTY(qreal fontSize READ fontSize WRITE setFontSize NOTIFY
    // formatChanged)
    //    Q_PROPERTY(
    //        qreal topMargin READ topMargin WRITE setTopMargin NOTIFY
    // formatChanged)
    //    Q_PROPERTY(
    //        qreal textIndent READ textIndent WRITE setTextIndent NOTIFY
    // formatChanged)

public:

    MinimapCanvas(QQuickItem *parent = nullptr);

    QQuickTextDocument* textDocument() const;
    void                setTextDocument(QQuickTextDocument *textDocument);

    //    QString             fontFamily() const;
    //    void                setFontFamily(const QString& fontFamily);

    //    qreal               fontSize() const;
    //    void                setFontSize(qreal fontSize);

    //    qreal               topMargin() const;
    //    void                setTopMargin(qreal topMargin);
    //    qreal               textIndent() const;
    //    void                setTextIndent(qreal textIndent);

    void paint(QPainter *painter);

signals:

    void textDocumentChanged();
    void formatChanged();

private:

    QQuickTextDocument *m_textDoc;
    QTextCursor m_textCursor;
};

#endif // MINIMAPCANVAS_H
