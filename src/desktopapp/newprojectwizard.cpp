#include "newprojectwizard.h"
#include "interfaces/newprojecttemplateinterface.h"
#include "projectcommands.h"
#include "skrdata.h"
#include "ui_newprojectwizard.h"

#include <QActionGroup>
#include <QButtonGroup>
#include <QFileDialog>

NewProjectWizard::NewProjectWizard(QWidget *parent) :
    QWizard(parent),
    ui(new Ui::NewProjectWizard),
    m_fileNameValidated(true)
{
    ui->setupUi(this);

    setupInfoPage();
    setupFormatPage();
    setupTemplatePage();

}

NewProjectWizard::~NewProjectWizard()
{
    delete ui;
}

const QString &NewProjectWizard::currentFormat() const
{
    return m_currentFormat;
}

void NewProjectWizard::setCurrentFormat(const QString &newCurrentFormat)
{
    if (m_currentFormat == newCurrentFormat)
        return;
    m_currentFormat = newCurrentFormat;
    emit currentFormatChanged();

    ui->formatDetailLabel->setText(m_formatWithPlugin.value(newCurrentFormat)->formatDetailText());
}

//----------------------------------------------------

void NewProjectWizard::setupInfoPage()
{
    QTimer::singleShot(0, this, [this](){
         this->button(QWizard::NextButton)->setEnabled(false);
    });


    QObject::connect(ui->projectNameLineEdit, &QLineEdit::textChanged, this, [this](const QString &text){


//        QRegularExpressionValidator validator(QRegularExpression(""));
//
//        int validatorPos = 0;
//        validator.validate(validatorText, validatorPos);


        this->button(QWizard::NextButton)->setEnabled(!text.isEmpty());

        m_projectName = text;

        QString validatorText = text;
        validatorText.remove("*");
        validatorText.remove(QRegularExpression("[<>:\"/\\\\\\|\\?]."));

        m_validProjectName = validatorText;

        determineFileName();
    });


    // languages :



}

//----------------------------------------------------

void NewProjectWizard::setupFormatPage()
{
    // paths
    QString startPath = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);

    ui->pathLineEdit->setText(startPath);

    QObject::connect(ui->selectPathButton, &QPushButton::clicked, this, [this, startPath](){
        QString path = QFileDialog::getExistingDirectory(this, QString(), startPath);
        ui->pathLineEdit->setText(path);

        determineFileName();
    });


    // formats

    QButtonGroup *formatActionGroup = new QButtonGroup(this);

    QList<NewProjectFormatInterface *> pluginList =
            skrpluginhub->pluginsByType<NewProjectFormatInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](NewProjectFormatInterface *plugin1, NewProjectFormatInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );


    for(auto *plugin : pluginList){
        m_formatWithPlugin.insert(plugin->extension(), plugin);

        QPushButton *button = new QPushButton(this);
        button->setMaximumWidth(200);
        button->setCheckable(true);
        button->setText(plugin->buttonText());
        button->setIcon(QIcon(plugin->buttonIcon()));
        button->setProperty("extension", plugin->extension());

        formatActionGroup->addButton(button);
        ui->formatButtonsLayout->addWidget(button);

    }

    QObject::connect(formatActionGroup, &QButtonGroup::buttonToggled, this, [this](QAbstractButton *button, bool checked){
        if(checked){
            setCurrentFormat(button->property("extension").toString());
        }

    });

    formatActionGroup->buttons().first()->setChecked(true);



}

//-----------------------------------------------


void NewProjectWizard::setupTemplatePage()
{

    QList<NewProjectTemplateInterface *> pluginList =
            skrpluginhub->pluginsByType<NewProjectTemplateInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](NewProjectTemplateInterface *plugin1, NewProjectTemplateInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );


    QListWidgetItem *noneItem = new QListWidgetItem(tr("None"), ui->templateListWidget);
    noneItem->setData(Qt::UserRole, "none");
    noneItem->setData(Qt::UserRole + 1, tr("Create an empty project."));

    for(auto *plugin : pluginList){
        for(int i = 0; i < plugin->templateHumanNames().count() ; i++){
            QString title = plugin->templateHumanNames().at(i);
            QListWidgetItem *item = new QListWidgetItem(title, ui->templateListWidget);
            item->setData(Qt::UserRole, plugin->templateNames().at(i));
            item->setData(Qt::UserRole + 1, plugin->templateDetailText().at(i));
        }
    }

    QObject::connect(ui->templateListWidget, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *current, QListWidgetItem *previous){
        m_selectedTemplateInternalName = current->data(Qt::UserRole).toString();
        ui->templateDetailLabel->setText(current->data(Qt::UserRole + 1).toString());
    });

    ui->templateListWidget->setCurrentItem(noneItem);

}

//-----------------------------------------------

void NewProjectWizard::determineFileName()
{

    QString finalFileName = m_formatWithPlugin.value(m_currentFormat)->finalFileName(ui->pathLineEdit->text(), m_validProjectName);
    ui->createdFileNameLabel->setText(finalFileName);

    setNewProjectUrl(QUrl::fromLocalFile(finalFileName));
}

//-----------------------------------------------

const QUrl &NewProjectWizard::newProjectUrl() const
{
    return m_newProjectUrl;
}

void NewProjectWizard::setNewProjectUrl(const QUrl &newNewProjectUrl)
{
    if (m_newProjectUrl == newNewProjectUrl)
        return;
    m_newProjectUrl = newNewProjectUrl;




    emit newProjectUrlChanged();
}

//----------------------------------------------------


void NewProjectWizard::accept()
{


    int newProjectId = projectCommands->createNewEmptyProject();

    projectCommands->setProjectName(newProjectId, m_projectName);
    projectCommands->setAuthor(newProjectId, ui->authorLineEdit->text());
    projectCommands->setLanguageCode(newProjectId, ui->languageComboBox->currentData().toString());


    // template:


    QList<NewProjectTemplateInterface *> pluginList =
            skrpluginhub->pluginsByType<NewProjectTemplateInterface>();


    for(auto *plugin : pluginList){
        if(plugin->templateNames().contains(m_selectedTemplateInternalName)){
            plugin->applyTemplate(newProjectId, m_selectedTemplateInternalName);
            break;
        }
    }

    // finally
    projectCommands->saveAs(newProjectId, m_newProjectUrl, m_formatWithPlugin.value(m_currentFormat)->extension());



    QWizard::accept();
}


void NewProjectWizard::reject()
{
    QWizard::reject();
}
