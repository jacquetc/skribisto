#include "projectview.h"
#include "projectcommands.h"
#include "projectdictdialog.h"
#include "skrdata.h"
#include "tagmanagertoolbox.h"
#include "text/spellchecker.h"
#include "toolboxes/outlinetoolbox.h"
#include "ui_projectview.h"

ProjectView::ProjectView(QWidget *parent) : View("PROJECT", parent), centralWidgetUi(new Ui::ProjectView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);
    this->setFocusProxy(centralWidgetUi->projectNameLineEdit);

    connect(centralWidgetUi->projectNameLineEdit, &QLineEdit::editingFinished, this, [this]() {
        if (skrdata->projectHub()->getProjectName(this->projectId()) != centralWidgetUi->projectNameLineEdit->text())
        {
            projectCommands->setProjectName(this->projectId(), centralWidgetUi->projectNameLineEdit->text());
        }
    });
    connect(centralWidgetUi->authorLineEdit, &QLineEdit::editingFinished, this, [this]() {
        if (skrdata->projectHub()->getAuthor(this->projectId()) != centralWidgetUi->authorLineEdit->text())
        {
            projectCommands->setAuthor(this->projectId(), centralWidgetUi->authorLineEdit->text());
        }
    });
    connect(centralWidgetUi->dictionaryComboBox, &QComboBox::activated, this, [this](int index) {
        if (skrdata->projectHub()->getLangCode(this->projectId()) !=
            centralWidgetUi->dictionaryComboBox->currentData().toString())
        {
            projectCommands->setLanguageCode(this->projectId(),
                                             centralWidgetUi->dictionaryComboBox->currentData().toString());
        }
    });

    connect(centralWidgetUi->userDictPushButton, &QPushButton::clicked, this, [this]() {
        ProjectDictDialog dialog(this, this->projectId());
        dialog.exec();
    });
}

ProjectView::~ProjectView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> ProjectView::toolboxes()
{
    QList<Toolbox *> toolboxes;

    toolboxes.append(new TagManagerToolbox(nullptr, this->projectId()));

    OutlineToolbox *outlineToolbox = new OutlineToolbox;
    toolboxes.append(outlineToolbox);
    connect(this, &View::initialized, outlineToolbox, &Toolbox::setIdentifiersAndInitialize);
    outlineToolbox->setIdentifiersAndInitialize(this->treeItemAddress());

    return toolboxes;
}

void ProjectView::initialize()
{

    centralWidgetUi->projectNameLineEdit->setText(skrdata->projectHub()->getProjectName(this->projectId()));
    centralWidgetUi->authorLineEdit->setText(skrdata->projectHub()->getAuthor(this->projectId()));

    // dictionaries:

    QMap<QString, QString> dictAndPathMap = SpellChecker::dictAndPathMap();
    QMap<QString, QString>::const_iterator i = dictAndPathMap.constBegin();
    while (i != dictAndPathMap.constEnd())
    {
        QLocale locale(i.key());
        centralWidgetUi->dictionaryComboBox->addItem(QString("%1 (%2)").arg(locale.nativeLanguageName(), i.key()),
                                                     i.key());
        ++i;
    }

    centralWidgetUi->dictionaryComboBox->setCurrentIndex(
        centralWidgetUi->dictionaryComboBox->findData(skrdata->projectHub()->getLangCode(this->projectId())));

    // history
    emit this->addToHistoryCalled(this, QVariantMap());
}
