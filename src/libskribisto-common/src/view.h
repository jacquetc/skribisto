#ifndef VIEW_H
#define VIEW_H

#include "toolbox.h"

#include <QList>
#include <QToolBar>
#include <QVBoxLayout>
#include <QWidget>
#include "skribisto_common_global.h"

namespace Ui {
class View;
}

class SKRCOMMONEXPORT View : public QWidget
{
    Q_OBJECT

public:
    explicit View(const QString &type, QWidget *parent = nullptr);
    ~View();
    virtual QList<Toolbox*> toolboxes() { return  QList<Toolbox*>();}

    virtual void setIdentifiersAndInitialize(int projectId = -1, int treeItemId = -1);

    const QString &type() const;

    int projectId() const;

    int treeItemId() const;


    const QVariantMap &parameters() const;
    void setParameters(const QVariantMap &newParameters);

protected:
    virtual void initialize() = 0;


    void setCentralWidget(QWidget *widget);
    void setToolBar(QToolBar *toolBar);

    virtual QVariantMap addOtherViewParametersBeforeSplit() {
        return QVariantMap();
    }
//    void paintEvent(QPaintEvent *event);


signals:

private slots:
    void on_closeToolButton_clicked();

    void init();
private:
    Ui::View *ui;
    QString m_type;
    int m_projectId;
    int m_treeItemId;
    QVariantMap m_parameters;
};

#endif // VIEW_H