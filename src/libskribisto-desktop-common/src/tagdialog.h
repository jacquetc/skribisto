#ifndef TAGDIALOG_H
#define TAGDIALOG_H

#include <QDialog>
#include "skribisto_desktop_common_global.h"

namespace Ui {
class TagDialog;
}

class SKRDESKTOPCOMMONEXPORT TagDialog : public QDialog
{
    Q_OBJECT

public:
    explicit TagDialog(QWidget *parent, int projectId, int tagId = -1, bool creating = false);
    ~TagDialog();

    int newTagId() const;

private:
    Ui::TagDialog *ui;
    bool m_creating;
    QString m_color, m_textColor;
    int m_projectId, m_tagId;
    int m_newTagId;

    void setColors(const QString &color, const QString &textColor);
};

#endif // TAGDIALOG_H
