#include "tagdialog.h"
#include "skrdata.h"
#include "tagcommands.h"
#include "ui_tagdialog.h"

TagDialog::TagDialog(QWidget *parent, int projectId, int tagId, bool creating) :
    QDialog(parent),
    m_creating(creating),
    m_projectId(projectId),
    m_tagId(tagId),
    m_newTagId(-1),
    ui(new Ui::TagDialog)
{
    ui->setupUi(this);


    this->setWindowTitle(m_creating ? tr("Tag creation") : tr("Tag modification"));


    QMap<QString, QString> colorMap = skrdata->tagHub()->getTagPresetColors();

    int column = 0;
    int row = 0;
    int index = 0;

    QMap<QString, QString>::const_iterator i = colorMap.constBegin();
    while (i != colorMap.constEnd()) {
        auto *item = new QTableWidgetItem("");
        item->setBackground(QBrush(QColor(i.key())));
        item->setForeground(QBrush(QColor(i.value())));
        item->setTextAlignment(Qt::AlignCenter);

        ui->tableWidget->setItem(row, column, item);

        row = index / ui->tableWidget->rowCount();
        column = index % ui->tableWidget->rowCount();
        index += 1;
        ++i;
    }

    connect(ui->tableWidget, &QTableWidget::itemActivated, this, [this](QTableWidgetItem *item){
        setColors(item->background().color().name(), item->foreground().color().name());
    });

    connect(ui->tableWidget, &QTableWidget::itemClicked, this, [this](QTableWidgetItem *item){
        setColors(item->background().color().name(), item->foreground().color().name());
    });



    connect(ui->buttonBox, &QDialogButtonBox::accepted, this, [this](){
        if(m_creating){
            m_newTagId = tagCommands->addTag(m_projectId, ui->tagNameLineEdit->text(), m_color, m_textColor);
        }
        else {
            tagCommands->modifyTag(m_projectId, m_tagId, ui->tagNameLineEdit->text(), m_color, m_textColor);

        }

        this->close();

    });

    connect(ui->buttonBox, &QDialogButtonBox::rejected, this, [this](){
        this->close();

    });


    if(creating){
        QPair<QString, QString> colorPair = skrdata->tagHub()->getTagRandomColors();
        setColors(colorPair.first, colorPair.second);

    }
    else {
        ui->tagNameLineEdit->setText(skrdata->tagHub()->getTagName(m_projectId, m_tagId));
        setColors(skrdata->tagHub()->getTagColor(m_projectId, m_tagId), skrdata->tagHub()->getTagTextColor(m_projectId, m_tagId));
    }
}

TagDialog::~TagDialog()
{
    delete ui;
}

int TagDialog::newTagId() const
{
    return m_newTagId;
}

//----------------------------------------------------------------------

void TagDialog::setColors(const QString &color, const QString &textColor)
{

    m_color = color;
    ui->colorFrame->setStyleSheet(QString("QFrame { background-color: %1; }").arg(m_color));
    m_textColor = textColor;
    ui->textColorFrame->setStyleSheet(QString("QFrame { background-color: %1; }").arg(m_textColor));
}
