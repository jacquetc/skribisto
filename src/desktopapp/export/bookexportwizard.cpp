#include "bookexportwizard.h"
#include "skrdata.h"
#include "thememanager.h"
#include "ui_bookexportwizard.h"
#include "projectcommands.h"


#include <QButtonGroup>
#include <QFileDialog>
#include <QPrintDialog>
#include <QPrintPreviewDialog>
#include <QPrinter>
#include <QPrinterInfo>
#include <QDesktopServices>

BookExportWizard::BookExportWizard(QWidget *parent, int projectId, bool enablePrint) :
    QWizard(parent),
    ui(new Ui::BookExportWizard),
      m_enablePrint(enablePrint),
      m_projectId(projectId),
      m_selectedBookId(-1),
      m_selectedChapterId(-1)
{
    ui->setupUi(this);

    QList<PageTypeIconInterface *> pluginList =
            skrpluginhub->pluginsByType<PageTypeIconInterface>();
    for(auto *plugin : pluginList){
        m_typeWithPlugin.insert(plugin->pageType(), plugin);
    }


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

            if(whichButton != WizardButton::CustomButton1){
                return;
            }

            m_parameters.insert("font_size", ui->fontSizeSpinBox->value());
            m_parameters.insert("font_family", ui->fontComboBox->currentFont().family());
            m_parameters.insert("text_block_indent", ui->paragraphFirstLineIndentSpinBox->value());
            m_parameters.insert("text_block_top_margin", ui->paragraphTopMarginSpinBox->value());
            m_parameters.insert("text_space_between_line", ui->spaceBetweenLinesSpinBox->value());

            QList<int> outputItems = determineOutputItems();

            QPrinterInfo printerInfo;
            QPrinter     printer(printerInfo);

            QPrintPreviewDialog previewDialog(&printer);

            QTextDocument *finalDocument = projectCommands->getPrintTextDocument(m_projectId, m_parameters, outputItems);

            this->connect(&previewDialog, &QPrintPreviewDialog::paintRequested, this, [finalDocument](QPrinter *printer) {
                finalDocument->print(printer);
            });

            previewDialog.exec();


        });

    }
    else{
        this->setButtonText(WizardButton::FinishButton, tr("Export"));
    }

    // fill book list



    for(int treeItemId : skrdata->treeHub()->getAllIds(m_projectId)){
        if(skrdata->treeHub()->getType(m_projectId, treeItemId) == "SECTION"){
            if(skrdata->treePropertyHub()->getProperty(m_projectId, treeItemId, "section_type", "separator") == "book-beginning"){
                if(!skrdata->treeHub()->getTrashed(m_projectId, treeItemId)){


                    QString title = skrdata->treeHub()->getTitle(m_projectId, treeItemId);
                    QIcon icon;
                    auto *plugin = m_typeWithPlugin.value(skrdata->treeHub()->getType(m_projectId, treeItemId), nullptr);
                        if(plugin){
                            icon = QIcon(plugin->pageTypeIconUrl(m_projectId, treeItemId));
                        }




                    QListWidgetItem *bookItem = new QListWidgetItem(icon, title, ui->bookListWidget);
                    bookItem->setData(Qt::UserRole, treeItemId);
                }
            }
        }
    }

    connect(ui->bookListWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item){
        m_selectedBookId = item->data(Qt::UserRole).toInt();
        this->fillChapterListWidget(item->data(Qt::UserRole).toInt());
    });
    connect(ui->bookListWidget, &QListWidget::itemClicked, this, [this](QListWidgetItem *item){
        m_selectedBookId = item->data(Qt::UserRole).toInt();
        this->fillChapterListWidget(item->data(Qt::UserRole).toInt());
    });
    connect(ui->bookListWidget, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *item){
        m_selectedBookId = item->data(Qt::UserRole).toInt();
        this->fillChapterListWidget(item->data(Qt::UserRole).toInt());
    });


    if(ui->bookListWidget->count() > 0){
        ui->bookListWidget->setCurrentRow(0);
        this->fillChapterListWidget(ui->bookListWidget->currentItem()->data(Qt::UserRole).toInt());
    }


    // chapter list:


    connect(ui->chapterListWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item){
        m_selectedChapterId = item->data(Qt::UserRole).toInt();
    });
    connect(ui->chapterListWidget, &QListWidget::itemClicked, this, [this](QListWidgetItem *item){
        m_selectedChapterId = item->data(Qt::UserRole).toInt();
    });
    connect(ui->chapterListWidget, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *item){
        m_selectedChapterId = item->data(Qt::UserRole).toInt();
    });

    if(ui->chapterListWidget->count() > 0){
        ui->chapterListWidget->setCurrentRow(0);
    }


    setupOptionPage();

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);

}

BookExportWizard::~BookExportWizard()
{
    delete ui;
}

void BookExportWizard::done(int result)
{
    QList<int> outputItems = determineOutputItems();

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
                QTextDocument *finalDocument = projectCommands->getPrintTextDocument(m_projectId, m_parameters, outputItems);
                finalDocument->print(&printer);
            }
        }
        else {
            QUrl url = QUrl::fromLocalFile(ui->destinationLineEdit->text());
            SKRResult result = projectCommands->exportProject(m_projectId, url, m_currentFormat, m_parameters, outputItems);

            QDesktopServices::openUrl(url);
        }

        break;
    default:
        break;
    }


}

//----------------------------------------------

void BookExportWizard::fillChapterListWidget(int bookItemId)
{
    ui->chapterListWidget->clear();
    m_chaptersWithContents.clear();
    m_allItemsOfBook.clear();
    m_allItemsOfBook.append(m_selectedBookId);

    // list all ids for this book
    QList<int> chapterContent;

    bool insideBook = false;
    for(int treeItemId : skrdata->treeHub()->getAllIds(m_projectId)){
        if(treeItemId == bookItemId){
            insideBook = true;
            continue;
        }

        QString type = skrdata->treeHub()->getType(m_projectId, treeItemId);
        QString sectionType = skrdata->treePropertyHub()->getProperty(m_projectId, treeItemId, "section_type", "separator");
        bool trashed = skrdata->treeHub()->getTrashed(m_projectId, treeItemId);
        if(trashed){
            continue;
        }

        if(insideBook){
            m_allItemsOfBook.append(treeItemId);
        }
        else{
            continue;
        }

        if(type == "SECTION" && sectionType == "chapter"){
            if(!chapterContent.isEmpty()){
                m_chaptersWithContents << chapterContent;
            }
            chapterContent.clear();
        }

        if(type == "FOLDER"){
            continue;
        }

        if(type == "SECTION" && (sectionType == "book-end" || sectionType == "book-beginning")){
            m_chaptersWithContents << chapterContent;
            break;
        }

        chapterContent << treeItemId;

    }
    QListWidgetItem *bookItem = new QListWidgetItem(tr("All"), ui->chapterListWidget);
    bookItem->setData(Qt::UserRole, -1);


    for(const QList<int> &chapterContentList : m_chaptersWithContents){
            int chapterSectionId = chapterContentList.first();

            if(chapterSectionId == m_selectedBookId){
                continue;
            }


            QString title = skrdata->treeHub()->getTitle(m_projectId, chapterSectionId);
            QIcon icon;
            auto *plugin = m_typeWithPlugin.value(skrdata->treeHub()->getType(m_projectId, chapterSectionId), nullptr);
                if(plugin){
                    icon = QIcon(plugin->pageTypeIconUrl(m_projectId, chapterSectionId));
                }




            QListWidgetItem *bookItem = new QListWidgetItem(icon, title, ui->chapterListWidget);
            bookItem->setData(Qt::UserRole, chapterSectionId);

    }
}

//----------------------------------------------

QList<int> BookExportWizard::determineOutputItems()
{

    if(m_selectedChapterId > -1){
        for(QList<int> contentList : m_chaptersWithContents){
            int chapterId = contentList.first();

            if(chapterId == m_selectedChapterId){
                return contentList;
            }
        }
    }

    // else return book items


    return m_allItemsOfBook;
}

//----------------------------------------------

void BookExportWizard::setupOptionPage()
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
