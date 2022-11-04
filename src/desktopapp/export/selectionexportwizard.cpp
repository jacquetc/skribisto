#include "selectionexportwizard.h"
#include "projectcommands.h"
#include "treemodels/projecttreeselectproxymodel.h"
#include "ui_selectionexportwizard.h"
#include "skrdata.h"

#include <QButtonGroup>
#include <QFileDialog>
#include <QPrintDialog>
#include <QPrintPreviewDialog>
#include <QPrinter>
#include <QPrinterInfo>
#include <QDesktopServices>

SelectionExportWizard::SelectionExportWizard(QWidget *parent, int projectId, bool enablePrint) :
    QWizard(parent),
    ui(new Ui::SelectionExportWizard),
    m_enablePrint(enablePrint),
    m_projectId(projectId)
{
    ui->setupUi(this);

    ProjectTreeSelectProxyModel *selectProxyModel = new ProjectTreeSelectProxyModel(this, m_projectId);

    ui->treeView->setModel(selectProxyModel);
    ui->treeView->expandAll();

    if(m_enablePrint){
        this->setButtonText(WizardButton::FinishButton, tr("Print"));
        this->setOption(QWizard::HaveCustomButton1, true);
        this->setButtonText(WizardButton::CustomButton1, tr("Preview"));
        this->button(WizardButton::CustomButton1)->hide();

        ui->extensionGroupBox->hide();
        ui->destinationWidget->hide();

        connect(this, &QWizard::currentIdChanged, this,  [this](int pageId){
            this->button(WizardButton::CustomButton1)->setVisible(pageId == 1);

        });
        // preview:
        connect(this, &QWizard::customButtonClicked, this,  [this](int whichButton){

            m_parameters.insert("font_size", ui->fontSizeSpinBox->value());
            m_parameters.insert("font_family", ui->fontComboBox->currentFont().family());
            m_parameters.insert("text_block_indent", ui->paragraphFirstLineIndentSpinBox->value());
            m_parameters.insert("text_block_top_margin", ui->paragraphTopMarginSpinBox->value());
            m_parameters.insert("text_space_between_line", ui->spaceBetweenLinesSpinBox->value());

            if(whichButton != WizardButton::CustomButton1){
                return;
            }
            ProjectTreeSelectProxyModel *selectProxyModel = static_cast<ProjectTreeSelectProxyModel *>(ui->treeView->model());
            QList<int> checkedIdList =selectProxyModel->getCheckedIdsList();

            QPrinterInfo printerInfo;
            QPrinter     printer(printerInfo);

            QPrintPreviewDialog previewDialog(&printer);

            QTextDocument *finalDocument = projectCommands->getPrintTextDocument(m_projectId, m_parameters, checkedIdList);

            this->connect(&previewDialog, &QPrintPreviewDialog::paintRequested, this, [finalDocument](QPrinter *printer) {
                finalDocument->print(printer);
            });

            previewDialog.exec();


        });

    }
    else{
        this->setButtonText(WizardButton::FinishButton, tr("Export"));
    }

    setupOptionPage();
}

SelectionExportWizard::~SelectionExportWizard()
{
    delete ui;
}

void SelectionExportWizard::done(int result)
{
    ProjectTreeSelectProxyModel *selectProxyModel = static_cast<ProjectTreeSelectProxyModel *>(ui->treeView->model());
    QList<int> checkedIdList =selectProxyModel->getCheckedIdsList();

    switch (result) {
    case 0:
        this->close();

        break;
    case 1:

        m_parameters.insert("font_size", ui->fontSizeSpinBox->value());
        m_parameters.insert("font_family", ui->fontComboBox->currentFont().family());
        m_parameters.insert("text_block_indent", ui->paragraphFirstLineIndentSpinBox->value());
        m_parameters.insert("text_block_top_margin", ui->paragraphTopMarginSpinBox->value());
        m_parameters.insert("text_space_between_line", ui->spaceBetweenLinesSpinBox->value());

        if(m_enablePrint){
            QPrinterInfo printerInfo;
            QPrinter     printer(printerInfo);

            QPrintDialog printDialog(&printer);

            if (printDialog.exec() == QDialog::Accepted) {
                QTextDocument *finalDocument = projectCommands->getPrintTextDocument(m_projectId, m_parameters, checkedIdList);
                finalDocument->print(&printer);
            }
        }
        else {
            QUrl url = QUrl::fromLocalFile(ui->destinationLineEdit->text());
            SKRResult result = projectCommands->exportProject(m_projectId, url, m_currentFormat, m_parameters, checkedIdList);

            QDesktopServices::openUrl(url);
        }
        break;
    default:
        break;
    }




}

//----------------------------------------------

void SelectionExportWizard::setupOptionPage()
{


    QString validatorText = skrdata->projectHub()->getProjectName(m_projectId);
    validatorText.remove("*");
    validatorText.remove(QRegularExpression("[<>:\"/\\\\\\|\\?]."));

    QString validProjectName = validatorText;

    // paths
    QString startPath = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);
    m_destinationPath = startPath;

    ui->destinationLineEdit->setText(startPath);

    QObject::connect(ui->selectDestinationButton, &QPushButton::clicked, this, [this, startPath, validProjectName](){
        m_destinationPath = QFileDialog::getExistingDirectory(this, QString(), startPath);
        ui->destinationLineEdit->setText(QDir::toNativeSeparators(m_destinationPath + "/" + validProjectName + "." + m_currentFormat));


    });

    // formats:

    QButtonGroup *formatActionGroup = new QButtonGroup(this);

    QList<QPair<QString,QString>> extensionPairList = projectCommands->getExportExtensions();
    QStringList extensionHumanNameList = projectCommands->getExportExtensionHumanNames();

    for(int i = 0; i < extensionPairList.count() ; i++){
        QPair<QString,QString> extensionPair = extensionPairList.at(i);
        QString extensionHumanName = extensionHumanNameList.at(i);


        QPushButton *button = new QPushButton(this);
        button->setMaximumWidth(200);
        button->setCheckable(true);
        button->setText(extensionPair.second);
        button->setProperty("extension", extensionPair.second);
        button->setProperty("extension_short_name", extensionPair.first);
        button->setProperty("extension_human_name", extensionHumanName);
        button->setToolTip(extensionHumanName);

        formatActionGroup->addButton(button);
        ui->formatButtonsLayout->addWidget(button);

    }

    QObject::connect(formatActionGroup, &QButtonGroup::buttonToggled, this, [this, validProjectName](QAbstractButton *button, bool checked){
        if(checked){
            m_currentFormat = button->property("extension").toString();
            ui->destinationLineEdit->setText(QDir::toNativeSeparators(m_destinationPath + "/" + validProjectName + "." + m_currentFormat));
        }

    });

    if(!extensionPairList.isEmpty()){
        formatActionGroup->buttons().first()->setChecked(true);
    }
}
