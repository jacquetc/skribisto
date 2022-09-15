#include "themewizard.h"
#include "thememanager.h"
#include "ui_themewizard.h"

#include <QColorDialog>

ThemeWizard::ThemeWizard(QWidget *parent) :
    QWizard(parent),
    ui(new Ui::ThemeWizard)
{
    ui->setupUi(this);

    ui->themeTreeWidget->setHeaderLabel(tr("Themes"));

    ui->typeComboBox->addItem(tr("Light"), "light");
    ui->typeComboBox->addItem(tr("Dark"), "dark");



    resetThemeList();
    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);

    connect(ui->themeTreeWidget, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem *current, QTreeWidgetItem *previous){
        if(!current->data(0, Qt::UserRole).toString().isEmpty()){
            m_selectedTheme = current->text(0);
            m_selectedLocation = current->data(0, Qt::UserRole).toString();

            ui->themeNameLabel->setText(m_selectedTheme);

            bool isEditable = themeManager->themeWithEditableHash().value(m_selectedTheme, false);


            ui->modifyRadioButton->setEnabled(isEditable);
            ui->modifyRadioButton->setChecked(isEditable);
            ui->duplicateAndModifyRadioButton->setChecked(!isEditable);

            if(ui->duplicateAndModifyRadioButton->isChecked()){
                ui->nameLineEdit->setText(m_selectedTheme + " " + tr("(Copy)"));
            }
            else{
                ui->nameLineEdit->setText(m_selectedTheme);
            }

            if(themeManager->themeType(m_selectedTheme) == ThemeManager::Light){
                ui->typeComboBox->setCurrentIndex(0);
            }
            if(themeManager->themeType(m_selectedTheme) == ThemeManager::Dark){
                ui->typeComboBox->setCurrentIndex(1);
            }


            QMap<QString, QString> colorMap = themeManager->getColorMap(m_selectedTheme);
            this->setColorTableColors(colorMap);

            ui->themeExampleWidget->setPalette(themeManager->toPalette(colorMap));
        }

    });

    connect(ui->colorTableWidget, &QTableWidget::itemDoubleClicked, this, [this](QTableWidgetItem *item){
        QBrush brush = item->background();
        QColor newColor = QColorDialog::getColor(brush.color(), this, tr("Select Color"), QColorDialog::ShowAlphaChannel | QColorDialog::DontUseNativeDialog);

        item->setBackground(QBrush(QColor(newColor)));

        auto colorMap = getColorMapFromTable();
        ui->themeExampleWidget->setPalette(themeManager->toPalette(colorMap));
    });

}

ThemeWizard::~ThemeWizard()
{
    delete ui;
}

void ThemeWizard::resetThemeList()
{
    ui->themeTreeWidget->clear();
    themeManager->reloadThemes();

    QTreeWidgetItem *lightItem = new QTreeWidgetItem(ui->themeTreeWidget, QTreeWidgetItem::UserType);
    lightItem->setText(0, tr("Light"));
    lightItem->setData(0, Qt::DecorationRole, QIcon(":/icons/backup/color-picker-white.svg"));
    lightItem->setFlags(Qt::ItemIsEnabled);

    QMap<QString, QString> lightMap = themeManager->lightThemeWithLocationMap();

    QMapIterator<QString, QString> iter(lightMap);
    while(iter.hasNext()){
        iter.next();

        QTreeWidgetItem *item = new QTreeWidgetItem(lightItem, QTreeWidgetItem::Type);
        item->setText(0, iter.key());
        item->setData(0 , Qt::UserRole, iter.value());

    }
    ui->themeTreeWidget->expandItem(lightItem);


    QTreeWidgetItem *darkItem = new QTreeWidgetItem(ui->themeTreeWidget, QTreeWidgetItem::UserType);
    darkItem->setText(0, tr("Dark"));
    darkItem->setData(0, Qt::DecorationRole, QIcon(":/icons/backup/color-picker-black.svg"));
    darkItem->setFlags(Qt::ItemIsEnabled);


    QMap<QString, QString> darkMap = themeManager->darkThemeWithLocationMap();

    QMapIterator<QString, QString> darkIter(darkMap);
    while(darkIter.hasNext()){
        darkIter.next();

        QTreeWidgetItem *item = new QTreeWidgetItem(darkItem, QTreeWidgetItem::Type);
        item->setText(0, darkIter.key());
        item->setData(0 , Qt::UserRole, darkIter.value());

    }
    ui->themeTreeWidget->expandItem(darkItem);
}

void ThemeWizard::setColorTableColors(QMap<QString, QString> colorMap)
{
    ui->colorTableWidget->clear();
    ui->colorTableWidget->setRowCount(colorMap.count());
    ui->colorTableWidget->setColumnCount(2);

    QMapIterator<QString, QString> iter(colorMap);

    int row = 0;
    while(iter.hasNext()){
        iter.next();

        QTableWidgetItem *textItem = new QTableWidgetItem();
        textItem->setText(this->getHumanReadableName(iter.key()));
        textItem->setData(Qt::UserRole, iter.key());
        textItem->setFlags(Qt::ItemIsEnabled);
        ui->colorTableWidget->setItem(row, 0, textItem);

        QTableWidgetItem *colorItem = new QTableWidgetItem();
        colorItem->setData(Qt::UserRole, iter.key());
        colorItem->setData(Qt::BackgroundRole, QBrush(QColor(iter.value())));
        colorItem->setFlags(Qt::ItemIsEnabled);
        ui->colorTableWidget->setItem(row, 1, colorItem);

        row++;

    }
}

QString ThemeWizard::getHumanReadableName(const QString &colorPropertyName) const
{
    if(colorPropertyName == "window"){
        return tr("Window background");
    }
    if(colorPropertyName == "windowText"){
        return tr("Window text");
    }
    if(colorPropertyName == "base"){
        return tr("Base background");
    }
    if(colorPropertyName == "text"){
        return tr("Base text");
    }
    if(colorPropertyName == "brightText"){
        return tr("Bright text");
    }
    if(colorPropertyName == "highlight"){
        return tr("Highlight");
    }
    if(colorPropertyName == "highlightedText"){
        return tr("Highlighted text");
    }
    if(colorPropertyName == "button"){
        return tr("Button background");
    }
    if(colorPropertyName == "buttonText"){
        return tr("Button text");
    }
    if(colorPropertyName == "light"){
        return tr("Light hue");
    }
    if(colorPropertyName == "mid"){
        return tr("Midlight hue");
    }
    if(colorPropertyName == "dark"){
        return tr("Dark hue");
    }
    if(colorPropertyName == "placeholderText"){
        return tr("Placeholder text");
    }
    if(colorPropertyName == "toolTipText"){
        return tr("Tip text");
    }
    if(colorPropertyName == "toolTipBase"){
        return tr("Tip background");
    }
    if(colorPropertyName == "shadow"){
        return tr("Shadow");
    }

    return colorPropertyName;

}

//----------------------------------------------------

QMap<QString, QString> ThemeWizard::getColorMapFromTable() const
{

    QMap<QString, QString> colorMap;

    for(int i = 0; i < ui->colorTableWidget->rowCount(); i++){

        QTableWidgetItem *item = ui->colorTableWidget->item(i, 1);
        colorMap.insert(item->data(Qt::UserRole).toString(), item->background().color().name(QColor::HexArgb));
    }

    return colorMap;

}

//----------------------------------------------------
//----------------------------------------------------
//----------------------------------------------------

