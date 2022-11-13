#ifndef TAGCHOOSERDIALOG_H
#define TAGCHOOSERDIALOG_H

#include <QDialog>

namespace Ui {
class TagChooserDialog;
}

class TagChooserDialog : public QDialog
{
    Q_OBJECT

public:
    explicit TagChooserDialog(QWidget *parent, int projectId, int treeItemId);
    ~TagChooserDialog();

public slots:
    void reloadAvailableTags();
    void reloadItemTags();


private:
    Ui::TagChooserDialog *ui;
    int m_projectId, m_treeItemId;

};

#endif // TAGCHOOSERDIALOG_H
