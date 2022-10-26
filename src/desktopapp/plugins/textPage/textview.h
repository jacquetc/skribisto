#ifndef TEXTVIEW_H
#define TEXTVIEW_H

#include <QWidget>
#include "view.h"

namespace Ui {
class TextView;
}

class TextView : public View
{
    Q_OBJECT

public:
    explicit TextView(QWidget *parent = nullptr);
    ~TextView();
    QList<Toolbox *> toolboxes() override;

public slots:
    void saveContent();

protected:
    void initialize() override;

private:
    Ui::TextView *centralWidgetUi;
    bool m_isSecondaryContent;

    // QWidget interface
    void saveTextState();
protected:
    void mousePressEvent(QMouseEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;
};

#endif // TEXTVIEW_H
