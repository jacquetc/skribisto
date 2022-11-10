#include "outlinetoolbox.h"
#include "text/markdowntextdocument.h"
#include "skrdata.h"
#include "text/textbridge.h"
#include "ui_outlinetoolbox.h"
#include "skrusersettings.h"
#include "projecttreecommands.h"

#include <QTimer>


OutlineToolbox::OutlineToolbox(QWidget *parent) :
    Toolbox(parent),
    ui(new Ui::OutlineToolbox), m_wasModified(false)
{
    ui->setupUi(this);
}

OutlineToolbox::~OutlineToolbox()
{
    if(m_wasModified){
        saveContent();
    }

    saveTextState();
    delete ui;
}

void OutlineToolbox::initialize()
{

    MarkdownTextDocument *document = new MarkdownTextDocument(this);


   document->setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(this->projectId(), this->treeItemId()));


    ui->textEdit->setDocument(document);

    QString uniqueDocumentReference = QString("%1_%2_%3").arg(QString::number(this->projectId()), QString::number(this->treeItemId()), "secondary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                ui->textEdit->uuid(),
                document);



    // create save timer

    m_saveTimer = new QTimer(this);
    m_saveTimer->setSingleShot(true);
    m_saveTimer->setInterval(200);
    connect(m_saveTimer, &QTimer::timeout, this, &OutlineToolbox::saveContent);
    QTimer::singleShot(0, this, [this](){ connectSaveConnection();});


    QSettings settings;

    // restore font size:

    int textFontPointSize = settings.value("outlineToolbox/textFontPointSize", qApp->font().pointSize()).toInt();

    QFont font = ui->textEdit->font();
    font.setPointSize(textFontPointSize);
    ui->textEdit->setFont(font);


    // restore cursor position

    int cursorPosition = SKRUserSettings::getFromProjectSettingHash(this->projectId(), "outlineTextCursorPosition", QString::number(this->treeItemId()), 0).toInt();

    QTextCursor cursor(ui->textEdit->document());
    cursor.setPosition(cursorPosition);
    ui->textEdit->setTextCursor(cursor);
    ui->textEdit->ensureCursorVisible();
}


void OutlineToolbox::saveContent()
{
    QString markdown = static_cast<MarkdownTextDocument *>(ui->textEdit->document())->toSkribistoMarkdown();

    projectTreeCommands->setContent(this->projectId(), this->treeItemId(), markdown, true);

    m_wasModified = false;
}


void OutlineToolbox::saveTextState()
{
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "outlineTextCursorPosition", QString::number(this->treeItemId()),
                                                                     ui->textEdit->textCursor().position());

}


void OutlineToolbox::connectSaveConnection()
{

  m_saveConnection =
      connect(ui->textEdit, &QTextEdit::textChanged, this, [this]() {
        m_wasModified = true;

        if(m_saveTimer->isActive()){
          m_saveTimer->stop();
        }
        m_saveTimer->start();
      });
}


void OutlineToolbox::wheelEvent(QWheelEvent *event)
{
    if(event->modifiers() == Qt::ControlModifier) {

        QPoint numPixels = event->pixelDelta();
        QPoint numDegrees = event->angleDelta() / 8;



        QObject::disconnect(m_saveConnection);

        if (!numPixels.isNull()) {
            if(numPixels.y() > 0) {
                ui->textEdit->zoomOut();
            }
            else{
                ui->textEdit->zoomIn();
            }
        } else if (!numDegrees.isNull()) {
            QPoint numSteps = numDegrees / 15;
            if(numSteps.y() > 0) {
                ui->textEdit->zoomOut();
            }
            else{
                ui->textEdit->zoomIn();
            }
        }

        if(ui->textEdit->font().pointSize() < 8){

            QFont font = ui->textEdit->font();
            font.setPointSize(8);
            ui->textEdit->setFont(font);

            event->accept();
        }

        connectSaveConnection();

        QSettings settings;
        settings.setValue("outlineToolbox/textFontPointSize", ui->textEdit->font().pointSize());

        event->accept();
    }
    else {
            ui->textEdit->wheelEvent(event);
    }
}
