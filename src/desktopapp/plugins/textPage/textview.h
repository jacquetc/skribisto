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
    QMetaObject::Connection m_saveConnection;
    QTimer *m_saveTimer;
    bool m_wasModified;

    void saveTextState();
    void connectSaveConnection();

protected:
    void mousePressEvent(QMouseEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;
};

#endif // TEXTVIEW_H
