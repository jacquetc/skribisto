#include "themewizard.h"
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
            ui->infoLabel->setText(themeManager->themeInfo(m_selectedTheme));

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

            QPalette palette = themeManager->toPalette(colorMap);
            // side example tab :
            QPalette sidePalette = themeManager->createSidePalette(palette);
            ui->sideThemeExampleWidget->setPalette(sidePalette);
            ui->comboBoxExample->setPalette(sidePalette);
            QTimer::singleShot(0, this, [this](){
                ui->comboBoxExample->shakePalette();
            });

            // middle example tab :
            QPalette middlePalette = themeManager->createMiddlePalette(palette);
            ui->middleThemeExampleWidget->setPalette(middlePalette);
            ui->comboBoxExample_2->setPalette(middlePalette);
            QTimer::singleShot(0, this, [this](){
                ui->comboBoxExample_2->shakePalette();
            });
        }

    });

    connect(ui->colorTableWidget, &QTableWidget::itemDoubleClicked, this, [this](QTableWidgetItem *item){
        QBrush brush = item->background();
        QColor newColor = QColorDialog::getColor(brush.color(), this, tr("Select Color"), QColorDialog::ShowAlphaChannel | QColorDialog::DontUseNativeDialog);

        if(newColor.isValid()){
            ui->colorTableWidget->item(item->row(), 1)->setBackground(QBrush(QColor(newColor)));

            auto colorMap = getColorMapFromTable();
            ui->sideThemeExampleWidget->setPalette(themeManager->toPalette(colorMap));
            ui->comboBoxExample->setPalette(themeManager->toPalette(colorMap));
            QTimer::singleShot(0, this, [this](){ ui->comboBoxExample->shakePalette();});

        }
    });

    connect(ui->nameLineEdit, &QLineEdit::textChanged, this, [this](const QString &themeName){

         bool isEditableAndExists = themeManager->themeWithEditableHash().value(themeName, false);

        if(isEditableAndExists){
            this->button(QWizard::WizardButton::FinishButton)->setEnabled(true);

            if(themeManager->lightThemeWithLocationMap().contains(themeName)){
                m_outputFileName = themeManager->lightThemeWithLocationMap().value(themeName);
            }
            else if(themeManager->darkThemeWithLocationMap().contains(themeName)){
                m_outputFileName = themeManager->darkThemeWithLocationMap().value(themeName);
            }

            ui->outputInfoLabel->setText(tr("Replacing: %1").arg(m_outputFileName));
        } //if new name :
        else if (!themeManager->themeWithEditableHash().contains(themeName)){

            this->button(QWizard::WizardButton::FinishButton)->setEnabled(true);
            QString validatorText = themeName;
            validatorText.remove("*");
            validatorText.remove(QRegularExpression("[<>:\"/\\\\\\|\\?]."));

            m_outputFileName = themeManager->getWritablePathForTheme() + "/" + validatorText + ".json";
            ui->outputInfoLabel->setText(tr("Creating: %1").arg(m_outputFileName));
        }
        else{
            this->button(QWizard::WizardButton::FinishButton)->setEnabled(false);
            ui->outputInfoLabel->setText(tr("Error: The name of this theme already exists!"));
        }


    });

    connect(ui->typeComboBox, &QComboBox::activated, this, [this](int index){
        if(index == 0){
            m_outputThemeType = ThemeManager::Light;
        }
        else if(index == 1){
            m_outputThemeType = ThemeManager::Dark;
        }

    });
    m_outputThemeType = ThemeManager::Light;
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



void ThemeWizard::done(int result)
{
    if(result == QDialog::Accepted){
        themeManager->saveTheme(ui->nameLineEdit->text(), m_outputFileName, m_outputThemeType, getColorMapFromTable());
    }

    QWizard::done(result);
}
