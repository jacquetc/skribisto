#ifndef TAGCHOOSERDIALOG_H
#define TAGCHOOSERDIALOG_H

#include "treeitemaddress.h"
#include <QDialog>

namespace Ui {
class TagChooserDialog;
}

class TagChooserDialog : public QDialog
{
    Q_OBJECT

public:
    explicit TagChooserDialog(QWidget *parent, const TreeItemAddress &treeItemAddress);
    ~TagChooserDialog();

public slots:
    void reloadAvailableTags();
    void reloadItemTags();


private:
    Ui::TagChooserDialog *ui;
    TreeItemAddress m_treeItemAddress;
    int m_projectId;

};

#endif // TAGCHOOSERDIALOG_H
