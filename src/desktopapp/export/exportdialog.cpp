#include "exportdialog.h"
#include "export/bookexportwizard.h"
#include "export/selectionexportwizard.h"
#include "skrdata.h"
#include "thememanager.h"
#include "ui_exportdialog.h"

ExportDialog::ExportDialog(QWidget *parent, bool enablePrint) :
    QDialog(parent),
    ui(new Ui::ExportDialog),
    m_enablePrint(enablePrint)
{
    ui->setupUi(this);

    if(m_enablePrint){
        ui->groupBox->setTitle(tr("Choose the way of printing"));
        ui->bookDetailLabel->setText(tr("Print a book or a chapter using \"Section\" items"));
        ui->selectionDetailLabel->setText(tr("Select the items to print"));
    }

    connect(ui->bookButton, &QPushButton::clicked, this, [this](){
        BookExportWizard wizard(this, skrdata->projectHub()->getActiveProject(), m_enablePrint);
        this->close();
        wizard.exec();
    });


    connect(ui->selectionButton, &QPushButton::clicked, this, [this](){
        SelectionExportWizard wizard(this, skrdata->projectHub()->getActiveProject(), m_enablePrint);
        this->close();
        wizard.exec();
    });

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);

}

ExportDialog::~ExportDialog()
{
    delete ui;
}
