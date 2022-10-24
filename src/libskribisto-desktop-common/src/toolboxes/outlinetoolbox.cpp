#include "outlinetoolbox.h"
#include "skrdata.h"
#include "text/textbridge.h"
#include "ui_outlinetoolbox.h"

#include <QTimer>


OutlineToolbox::OutlineToolbox(QWidget *parent) :
    Toolbox(parent),
    ui(new Ui::OutlineToolbox)
{
    ui->setupUi(this);
}

OutlineToolbox::~OutlineToolbox()
{
    delete ui;
}

void OutlineToolbox::initialize()
{

    QTextDocument *document = new QTextDocument(this);


   document->setMarkdown(skrdata->treeHub()->getSecondaryContent(this->projectId(), this->treeItemId()));


    ui->textEdit->setDocument(document);

    QString uniqueDocumentReference = QString("%1_%2_%3").arg(QString::number(this->projectId()), QString::number(this->treeItemId()), "secondary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                ui->textEdit->uuid(),
                document);


    QTimer *saveTimer = new QTimer(this);
    saveTimer->setSingleShot(true);
    saveTimer->setInterval(100);


    connect(ui->textEdit, &QTextEdit::textChanged, this, [saveTimer](){
        if(saveTimer->isActive()){
            saveTimer->stop();
        }
        saveTimer->start();
    });
    connect(saveTimer, &QTimer::timeout, this, &OutlineToolbox::saveContent);
}


void OutlineToolbox::saveContent()
{
        skrdata->treeHub()->setSecondaryContent(this->projectId(), this->treeItemId(), ui->textEdit->toMarkdown());

}

