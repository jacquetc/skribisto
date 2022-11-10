#include "textpagesettings.h"
#include "desktopapplication.h"
#include "ui_textpagesettings.h"

#include <QSettings>

TextPageSettings::TextPageSettings(QWidget *parent) :
    SettingsSubPanel(parent),
    centralWidgetUi(new Ui::TextPageSettings)
{
    QWidget *widget = new QWidget;
    centralWidgetUi->setupUi(widget);
    setCentralWidget(widget);

    reset();

    connect(centralWidgetUi->fontPointSizeSpinBox, &QSpinBox::valueChanged, this, &TextPageSettings::updateSampleTextFormat);
    connect(centralWidgetUi->paragraphTopMarginSpinBox, &QSpinBox::valueChanged, this, &TextPageSettings::updateSampleTextFormat);
    connect(centralWidgetUi->firstLineIndentSpinBox, &QSpinBox::valueChanged, this, &TextPageSettings::updateSampleTextFormat);
    connect(centralWidgetUi->fontFamilyComboBox, &QFontComboBox::currentFontChanged, this, &TextPageSettings::updateSampleTextFormat);

}

//--------------------------------------

TextPageSettings::~TextPageSettings()
{
    delete centralWidgetUi;
}

//--------------------------------------

void TextPageSettings::updateSampleTextFormat()
{


    QTextCharFormat charFormat;
    charFormat.setFontPointSize(centralWidgetUi->fontPointSizeSpinBox->value());
    charFormat.setFontFamilies(QStringList() << centralWidgetUi->fontFamilyComboBox->currentFont().family());

    QTextBlockFormat blockFormat;
    blockFormat.setTopMargin(centralWidgetUi->paragraphTopMarginSpinBox->value());
    blockFormat.setTextIndent(centralWidgetUi->firstLineIndentSpinBox->value());

   QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
   textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
   textCursor.mergeBlockFormat(blockFormat);
   textCursor.mergeCharFormat(charFormat);
}

//--------------------------------------


void TextPageSettings::accept()
{
    QHash<QString, QVariant> newValuesHash;

    QSettings settings;

    QString textFontFamily = centralWidgetUi->fontFamilyComboBox->currentFont().family();
    settings.setValue("textPage/textFontFamily", textFontFamily);
    newValuesHash.insert("textPage/textFontFamily", textFontFamily);

    int textFontPointSize =  centralWidgetUi->fontPointSizeSpinBox->value();
    settings.setValue("textPage/textFontPointSize", textFontPointSize);
    newValuesHash.insert("textPage/textFontPointSize", textFontPointSize);

    int paragraphTopMargin = centralWidgetUi->paragraphTopMarginSpinBox->value();
    settings.setValue("textPage/paragraphTopMargin", paragraphTopMargin);
    newValuesHash.insert("textPage/paragraphTopMargin", paragraphTopMargin);

    int paragraphFirstLineIndent = centralWidgetUi->firstLineIndentSpinBox->value();
    settings.setValue("textPage/paragraphFirstLineIndent", paragraphFirstLineIndent);
    newValuesHash.insert("textPage/paragraphFirstLineIndent", paragraphFirstLineIndent);


    QHash<QString, QVariant> changedValuesHash;
    QHash<QString, QVariant>::const_iterator i = newValuesHash.constBegin();
    while (i != newValuesHash.constEnd()) {

        if(m_defaultValuesHash.value(i.key()) != i.value()){
            changedValuesHash.insert(i.key(), i.value());
        }
        ++i;
    }


    if(!changedValuesHash.isEmpty()){
        emit static_cast<DesktopApplication *>(qApp)->settingsChanged(changedValuesHash);
    }
}

//--------------------------------------

void TextPageSettings::reset()
{
    m_defaultValuesHash.clear();

    QSettings settings;

    QString textFontFamily = settings.value("textPage/textFontFamily", QApplication::font().family()).toString();
    QFont font;
    font.setFamily(textFontFamily);
    centralWidgetUi->fontFamilyComboBox->setCurrentFont(font);
    m_defaultValuesHash.insert("textPage/textFontFamily", textFontFamily);


    int textFontPointSize = settings.value("textPage/textFontPointSize",  QApplication::font().pointSize()).toInt();
    centralWidgetUi->fontPointSizeSpinBox->setValue(textFontPointSize);
    m_defaultValuesHash.insert("textPage/textFontPointSize", textFontPointSize);

    int paragraphTopMargin = settings.value("textPage/paragraphTopMargin",  12).toInt();
    centralWidgetUi->paragraphTopMarginSpinBox->setValue(paragraphTopMargin);
    m_defaultValuesHash.insert("textPage/paragraphTopMargin", paragraphTopMargin);

    int paragraphFirstLineIndent = settings.value("textPage/paragraphFirstLineIndent",  12).toInt();
    centralWidgetUi->firstLineIndentSpinBox->setValue(paragraphFirstLineIndent);
    m_defaultValuesHash.insert("textPage/paragraphFirstLineIndent", paragraphFirstLineIndent);

    updateSampleTextFormat();
}
