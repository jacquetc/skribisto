#include "welcomedialog.h"
#include "projectcommands.h"
#include "skrdata.h"
#include "ui_welcomedialog.h"

#include <NewProjectWizard.h>
#include <QFileDialog>
#include <QFileInfo>
#include <QSettings>

WelcomeDialog::WelcomeDialog(QWidget *parent) : QWidget(parent), ui(new Ui::WelcomeDialog)
{
    ui->setupUi(this);
    this->populateRecentProjects();
    QSettings settings;

    connect(ui->projectListWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item) {
        projectCommands->loadProject(item->data(Qt::UserRole).toUrl());

        this->close();
    });

    connect(ui->openButton, &QPushButton::clicked, this, [this]() {
        QUrl openFileNameUrl = QFileDialog::getOpenFileUrl(this, "Open a project", QUrl(),
                                                           WelcomeDialog::tr("Skribisto project (*.skrib)"));

        if (!openFileNameUrl.isEmpty())
        {
            projectCommands->loadProject(openFileNameUrl);

            this->close();
        }
    });
    connect(ui->newButton, &QPushButton::clicked, this, [this]() {
        NewProjectWizard newWizard(this);
        newWizard.exec();
        this->close();
    });

    ui->showAtStartupCheckBox->setChecked(settings.value("main/showWelcome", true).toBool());

    connect(ui->showAtStartupCheckBox, &QCheckBox::clicked, this, [this](bool checked) {
        QSettings settings;
        settings.setValue("main/showWelcome", checked);
    });

    ui->alwaysOpenLastProjectCheckBox->setChecked(
        settings.value("main/alwaysOpenLastProjectAtStartup", false).toBool());

    ui->showAtStartupCheckBox->setEnabled(!ui->alwaysOpenLastProjectCheckBox->isChecked());

    connect(ui->alwaysOpenLastProjectCheckBox, &QCheckBox::clicked, this, [this](bool checked) {
        QSettings settings;
        settings.setValue("main/alwaysOpenLastProjectAtStartup", checked);

        ui->showAtStartupCheckBox->setEnabled(!checked);
    });
}

WelcomeDialog::~WelcomeDialog()
{
    delete ui;
}

void WelcomeDialog::populateRecentProjects()
{
    ui->projectListWidget->clear();

    QSettings settings;

    settings.beginGroup("welcome");
    int size = settings.beginReadArray("recentProjects");
    QList<QObject *> projectList;

    for (int i = 0; i < size; ++i)
    {
        settings.setArrayIndex(i);

        QUrl fileName = settings.value("fileNameUrl").toUrl();

        // exists ?
        QFileInfo info(fileName.toLocalFile());
        if (!info.exists() || !info.isWritable())
        {
            continue;
        }

        QObject *project = new QObject(this);
        project->setProperty("title", settings.value("title").toString());
        project->setProperty("fileName", fileName);
        projectList.append(project);

        // is project opened ?

        for (int projectId : skrdata->projectHub()->getProjectIdList())
        {
            QString projectName = skrdata->projectHub()->getProjectName(projectId);
            QUrl projectPath = skrdata->projectHub()->getPath(projectId);

            if ((projectName == project->property("title").toString()) &&
                (projectPath == project->property("fileName").toUrl()))
            {

                projectList.prepend(projectList.takeLast());
            }
        }
    }

    settings.endArray();
    settings.endGroup();

    for (QObject *project : projectList)
    {

        QListWidgetItem *item = new QListWidgetItem(project->property("title").toString());
        item->setData(Qt::UserRole, project->property("fileName").toUrl());
        item->setIcon(QIcon(":/icons/backup/address-book-new.svg"));
        ui->projectListWidget->addItem(item);
    }
}
