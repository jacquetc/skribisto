#include "mainwindow.h"
#include "./ui_mainwindow.h"
#include "author/author_controller.h"
#include "system/system_controller.h"

#include <QMessageBox>
#include <QProgressDialog>

using namespace Presenter::Author;
using namespace Presenter::System;

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // disable all:
    ui->saveSystemPushButton->setEnabled(false);
    // ui->addAsyncPushButton->setEnabled(false);
    ui->removePushButton->setEnabled(false);

    // error handling :

    connect(ThreadedUndoRedoSystem::instance(), &ThreadedUndoRedoSystem::errorSent, this, [=](Error error) {
        QMessageBox *box = new QMessageBox(this);
        switch (error.status())
        {
        case Error::Status::Warning:
            box->setIcon(QMessageBox::Icon::Warning);
            box->setText(tr("Warning: %1").arg(error.message()));
            box->setDetailedText(error.className() + "\n" + error.code() + "\n" + error.message() + "\n" +
                                 error.data());
            break;
        case Error::Status::Critical:
            box->setIcon(QMessageBox::Icon::Critical);
            box->setText(tr("Critical: %1").arg(error.message()));
            box->setDetailedText(error.className() + "\n" + error.code() + "\n" + error.message() + "\n" +
                                 error.data());
            break;
        case Error::Status::Fatal:
            box->setIcon(QMessageBox::Icon::NoIcon);
            box->setText(tr("Fatal: %1").arg(error.message()));
            box->setDetailedText(error.className() + "\n" + error.code() + "\n" + error.message() + "\n" +
                                 error.data());
            break;
        default:
            break;
        }
        box->setModal(true);
        box->show();
    });

    // system :

    auto systemController = SystemController::instance();
    QProgressDialog *progressDialog = new QProgressDialog(this);
    progressDialog->setWindowModality(Qt::WindowModal);
    progressDialog->setMinimumDuration(1000);
    progressDialog->setCancelButton(nullptr);
    progressDialog->reset();

    connect(systemController, &SystemController::loadSystemProgressFinished, this, [progressDialog]() {
        progressDialog->reset();
        progressDialog->close();
    });

    connect(systemController, &SystemController::loadSystemProgressRangeChanged, this,
            [progressDialog](int minimum, int maximum) { progressDialog->setRange(minimum, maximum); });

    connect(systemController, &SystemController::loadSystemProgressTextChanged, this,
            [progressDialog](const QString &progressText) { progressDialog->setLabelText(progressText); });

    connect(systemController, &SystemController::loadSystemProgressValueChanged, this,
            [progressDialog](int progressValue) { progressDialog->setValue(progressValue); });

    connect(ui->loadSystemPushButton, &QPushButton::clicked, this, [=]() {
        SystemController::closeSystem();

        Contracts::DTO::System::LoadSystemDTO dto;
        dto.setFileName(QUrl("qrc:/test_clean.skrib"));

        systemController->loadSystem(dto);
    });

    connect(systemController, &SystemController::systemLoaded, this, [this]() {
        ui->addAsyncPushButton->setEnabled(true);
        ui->removePushButton->setEnabled(true);

        connect(
            AuthorController::instance(), &AuthorController::getAllReplied, this,
            [this](QList<AuthorDTO> result) {
                for (const auto &author : result)
                {
                    auto *item = new QListWidgetItem(ui->listWidget);
                    item->setText(author.name());
                    item->setData(Qt::UserRole, author.id());
                }
            },
            Qt::SingleShotConnection);

        AuthorController::instance()->getAll();
    });

    // undo redo buttons:

    ui->redoToolButton->setDefaultAction(
        Presenter::UndoRedo::ThreadedUndoRedoSystem::instance()->createRedoAction(this));
    ui->undoToolButton->setDefaultAction(
        Presenter::UndoRedo::ThreadedUndoRedoSystem::instance()->createUndoAction(this));

    // add / remove buttons:

    auto authorController = AuthorController::instance();

    connect(ui->addAsyncPushButton, &QPushButton::clicked, this, [=]() {
        Contracts::DTO::Author::CreateAuthorDTO dto("test");

        authorController->create(dto);
    });

    connect(authorController, &AuthorController::authorCreated, this, [this](AuthorDTO result) {
        auto *item = new QListWidgetItem(ui->listWidget);
        item->setText(result.name());
        item->setData(Qt::UserRole, result.id());
    });

    connect(ui->removePushButton, &QPushButton::clicked, this, [this]() {
        QListWidgetItem *item = ui->listWidget->item(0);
        if (!item)
        {
            return;
        }
        auto authorController = AuthorController::instance();

        authorController->remove(item->data(Qt::UserRole).toInt());
    });

    connect(authorController, &AuthorController::authorRemoved, this, [this](Contracts::DTO::Author::AuthorDTO result) {
        for (int i = 0; i < ui->listWidget->count(); i++)
        {
            if (ui->listWidget->item(i)->data(Qt::UserRole).toInt() == result.id())
            {
                ui->listWidget->takeItem(i);
                break;
            }
        }
    });
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::changeEvent(QEvent *e)
{
    QWidget::changeEvent(e);
    switch (e->type())
    {
    case QEvent::LanguageChange:
        ui->retranslateUi(this);
        break;
    default:
        break;
    }
}
