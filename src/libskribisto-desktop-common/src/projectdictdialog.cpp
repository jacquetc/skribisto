#include "projectdictdialog.h"
#include "ui_projectdictdialog.h"

#include <QInputDialog>
#include <QString>
#include <dictcommands.h>

ProjectDictDialog::ProjectDictDialog(QWidget *parent, int projectId)
    : QDialog(parent), ui(new Ui::ProjectDictDialog), m_projectId(projectId)
{
    ui->setupUi(this);

    populateWordList();
    connect(ui->searchLineEdit, &QLineEdit::textChanged, this, &ProjectDictDialog::populateWordList);

    connect(ui->deleteButton, &QPushButton::clicked, this, [this]() {
        if (ui->wordListWidget->currentItem() == nullptr)
        {
            return;
        }

        QString word = ui->wordListWidget->currentItem()->text();
        m_newWordList.removeOne(word);
        m_deletedWordList.append(word);
        m_deletedWordList.removeDuplicates();
        populateWordList();
    });
    connect(ui->newButton, &QPushButton::clicked, this, [this]() {
        bool ok;
        QString word = QInputDialog::getText(this, tr("New word"), tr("Add a new word to the user dictionary"),
                                             QLineEdit::EchoMode::Normal, "", &ok);

        if (!ok || word.isEmpty() || word.contains(" ") || m_originalWords.contains(word) ||
            m_newWordList.contains(word))
        {
            return;
        }
        m_newWordList.append(word);
        m_newWordList.removeDuplicates();
        populateWordList();
    });

    connect(ui->buttonBox, &QDialogButtonBox::clicked, this, [this](QAbstractButton *button) {
        if (ui->buttonBox->buttonRole(button) == QDialogButtonBox::ResetRole)
        {

            m_newWordList.clear();
            m_deletedWordList.clear();
            populateWordList();
        }
        if (ui->buttonBox->buttonRole(button) == QDialogButtonBox::AcceptRole)
        {
            for (const QString &word : m_deletedWordList)
            {
                dictCommands->deleteWordFromProjectDict(m_projectId, word);
            }

            for (const QString &word : m_newWordList)
            {
                dictCommands->addWordToProjectDict(m_projectId, word);
            }

            this->close();
        }
        if (ui->buttonBox->buttonRole(button) == QDialogButtonBox::RejectRole)
        {

            this->close();
        }
    });
}

ProjectDictDialog::~ProjectDictDialog()
{
    delete ui;
}

void ProjectDictDialog::populateWordList(const QString &filter)
{
    ui->wordListWidget->clear();
    m_originalWords = dictCommands->getProjectDictList(m_projectId);

    if (filter.isEmpty())
    {
        for (const QString &word : m_originalWords)
        {
            if (m_deletedWordList.contains(word))
            {
                continue;
            }
            new QListWidgetItem(word, ui->wordListWidget);
        }
    }
    else
    {
        for (const QString &word : m_originalWords.filter(filter, Qt::CaseInsensitive))
        {
            if (m_deletedWordList.contains(word))
            {
                continue;
            }
            new QListWidgetItem(word, ui->wordListWidget);
        }
    }

    for (const QString &newWord : m_newWordList)
    {
        new QListWidgetItem(newWord, ui->wordListWidget);
    }
}
