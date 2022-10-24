#ifndef SIZEHANDLE_H
#define SIZEHANDLE_H

#include <QMouseEvent>
#include <QWidget>

class SizeHandle : public QWidget
{
    Q_OBJECT
public:
    explicit SizeHandle(QWidget *parent = nullptr);

signals:


    // QWidget interface
protected:
    void mouseMoveEvent(QMouseEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;

signals:
    void moved(int deltaX);

private:
    QPoint m_oldPoint;


    // QWidget interface
protected:
    void enterEvent(QEnterEvent *event) override;
    void leaveEvent(QEvent *event) override;
};

#endif // SIZEHANDLE_H
