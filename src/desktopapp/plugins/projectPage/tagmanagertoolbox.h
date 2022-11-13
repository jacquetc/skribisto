#ifndef TAGMANAGERTOOLBOX_H
#define TAGMANAGERTOOLBOX_H

#include <QWidget>
#include "toolbox.h"

namespace Ui {
class TagManagerToolbox;
}

class TagManagerToolbox : public Toolbox
{
    Q_OBJECT

public:
    explicit TagManagerToolbox(QWidget *parent, int projectId);
    ~TagManagerToolbox();

public slots:
    void reloadTags();

private:
    Ui::TagManagerToolbox *ui;
    int m_projectId;

    // Toolbox interface
public:
    QString title() const override
    {
        return tr("Tag manager");
    }
    QIcon icon() const override
    {
        return QIcon(":/icons/backup/tag.svg");
    }


    // Toolbox interface
public:
    void initialize() override;
};


#endif // TAGMANAGERTOOLBOX_H
