#pragma once


#include <QDialog>

namespace Ui {
class ExportDialog;
}

class ExportDialog : public QDialog
{
    Q_OBJECT

public:
    explicit ExportDialog(QWidget *parent, bool enablePrint);
    ~ExportDialog();

private:
    Ui::ExportDialog *ui;
    bool m_enablePrint;
};


