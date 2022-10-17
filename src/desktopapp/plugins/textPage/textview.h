#ifndef TEXTVIEW_H
#define TEXTVIEW_H

#include <QWidget>
#include "view.h"
#include "toolboxes/outlinetoolbox.h"

namespace Ui {
class TextView;
}

class TextView : public View
{
    Q_OBJECT

public:
    explicit TextView(QWidget *parent = nullptr);
    ~TextView();
    QList<Toolbox *> toolboxes();

public slots:
    void saveContent();

protected:
    void initialize();

private:
    Ui::TextView *centralWidgetUi;
    OutlineToolbox *m_outlineToolbox;
    bool m_isSecondaryContent;

    // QWidget interface
protected:
    void mousePressEvent(QMouseEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;
};

#endif // TEXTVIEW_H
