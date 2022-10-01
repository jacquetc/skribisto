#ifndef OUTLINETOOLBOX_H
#define OUTLINETOOLBOX_H

#include <QWidget>
#include "toolbox.h"
#include "skribisto_common_global.h"

namespace Ui {
class OutlineToolbox;
}

class SKRCOMMONEXPORT OutlineToolbox : public Toolbox
{
    Q_OBJECT

public:
    explicit OutlineToolbox(QWidget *parent = nullptr);
    ~OutlineToolbox();

    // Toolbox interface
public:
    QString title() const {
        return tr("Outline");
    }
    QIcon icon() const {
        return QIcon(":/icons/backup/story-editor.svg");
    }

private:
    Ui::OutlineToolbox *ui;

};

#endif // OUTLINETOOLBOX_H