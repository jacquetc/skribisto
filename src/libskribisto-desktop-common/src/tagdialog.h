#ifndef TAGDIALOG_H
#define TAGDIALOG_H

#include <QDialog>

namespace Ui {
class TagDialog;
}

class TagDialog : public QDialog
{
    Q_OBJECT

public:
    explicit TagDialog(QWidget *parent, int projectId, int tagId = -1, bool creating = false);
    ~TagDialog();

private:
    Ui::TagDialog *ui;
    bool m_creating;
    QString m_color, m_textColor;
    int m_projectId, m_tagId;

    void setColors(const QString &color, const QString &textColor);
};

#endif // TAGDIALOG_H
