#include "skrexporter.h"
#include <QTextCursor>
#include <QTextDocumentFragment>
#include <QTextDocumentWriter>
#include <QGuiApplication>
#include <QPdfWriter>
#include <QPagedPaintDevice>
#include <QTextBlock>

#ifdef SKR_PRINT_SUPPORT
# include <QPrinter>
# include <QPrintPreviewDialog>
# include <QPrintDialog>
# include <QPrinterInfo>
#endif // SKR_PRINT_SUPPORT

#include "plmdata.h"
#include "skr.h"

SKRExporter::SKRExporter(QObject *parent) : QObject(parent), m_projectId(-2), m_printEnabled(false),
    m_outputType(OutputType::Odt), m_indentWithTitle(0), m_includeSynopsis(false), m_tagsEnabled(false), m_numbered(false),
    m_quick(false)
{
    m_fontFamily    = qGuiApp->font().family().replace(", ","");
    m_fontPointSize = qGuiApp->font().pointSize();
    m_textTopMargin = 2;
    m_textIndent = 2;
}

// --------------------------------------------------------------------------------------

void SKRExporter::run()
{
    if (m_projectId == -2 || (m_sheetIdList.isEmpty() && m_noteIdList.isEmpty())) {
        return;
    }

    emit progressMaximumChanged(m_sheetIdList.count() + m_noteIdList.count());



    m_charFormat.setFontFamily(m_fontFamily);
    m_charFormat.setFontPointSize(m_fontPointSize);
    m_blockFormat.setTopMargin(m_textTopMargin);
    m_blockFormat.setTextIndent(m_textIndent);


    QTextDocument *textDocument = new QTextDocument(this);
    QString projectTitle = plmdata->projectHub()->getProjectName(m_projectId);

    QTextCursor textCursor(textDocument);
    if (!m_quick) {
        // project title :

        textCursor.movePosition(QTextCursor::Start);


        QTextDocument titleDoc;

        titleDoc.setHtml(QString("%1").arg(projectTitle));

        textCursor.insertFragment(QTextDocumentFragment(&titleDoc));

        QTextBlockFormat titleBlockFormat;

        titleBlockFormat.setAlignment(Qt::AlignHCenter);
        titleBlockFormat.setHeadingLevel(1);
        textCursor.setBlockFormat(titleBlockFormat);
        textCursor.insertBlock(m_blockFormat, m_charFormat);
    }
    QList<int> sheetNumbers;

    for (int sheetId : m_sheetIdList) {
        this->createContent(textDocument, SKR::Sheet, m_projectId, sheetId, &sheetNumbers);
    }

    if (!m_noteIdList.isEmpty()) {
        if (!m_quick) {

            textCursor.movePosition(QTextCursor::End);
            QTextBlockFormat lineBreakBlockFormat;
            lineBreakBlockFormat.setPageBreakPolicy(QTextFormat::PageBreak_AlwaysBefore);
            textCursor.insertBlock(lineBreakBlockFormat);
            // notes :
            QTextDocument noteDoc;

            noteDoc.setPlainText(QString("%1").arg(tr("Notes")));

            textCursor.movePosition(QTextCursor::End);
            textCursor.insertFragment(QTextDocumentFragment(&noteDoc));


            QTextBlockFormat noteBlockFormat;

            noteBlockFormat.setAlignment(Qt::AlignHCenter);
            noteBlockFormat.setHeadingLevel(1);
            textCursor.mergeBlockFormat(noteBlockFormat);
            textCursor.insertBlock(m_blockFormat, m_charFormat);
        }

        QList<int> noteNumbers;

        for (int noteId : m_noteIdList) {
            this->createContent(textDocument, SKR::Note, m_projectId, noteId, &noteNumbers);
        }
    }

    // set size and bold with headers for ODT, PDF, Print

    for (int i = 0; i < textDocument->blockCount(); i++) {
        QTextBlock textBlock = textDocument->findBlockByNumber(i);

        int headingLevel = textBlock.blockFormat().headingLevel();

        if (headingLevel > 0) {
            QTextCursor cursor(textDocument);
            cursor.setPosition(textBlock.position());
            cursor.movePosition(QTextCursor::EndOfBlock, QTextCursor::KeepAnchor);

            QTextCharFormat textCharFormat;
            textCharFormat.setFontWeight(QFont::Bold);
            textCharFormat.setFontPointSize(m_fontPointSize + m_fontPointSize / headingLevel);
            cursor.mergeCharFormat(textCharFormat);
        }
    }


    // writing


    QByteArray format;

    switch (m_outputType) {
    case OutputType::Odt:
        format = "odf";
        break;

    case OutputType::Html:
        format = "HTML";
        break;

    case OutputType::Txt:
        format = "plaintext";
        break;

    case OutputType::Md:
        format = "markdown";
        break;

    default:
        format = "";
        break;
    }

    QString fileName = m_outputUrl.toLocalFile();

    if ((m_outputType != OutputType::Printer) && (m_outputType != OutputType::Pdf)) {
        QTextDocumentWriter writer(fileName, format);
        bool ok = writer.write(textDocument);
    }


    //    // PDF :

    if (m_outputType == OutputType::Pdf) {
        QPdfWriter pdfPrinter(fileName);
        pdfPrinter.setPdfVersion(QPagedPaintDevice::PdfVersion_1_6);
        pdfPrinter.setTitle(projectTitle);

        textDocument->print(&pdfPrinter);
    }

#ifdef SKR_PRINT_SUPPORT

    // Print preview:

    if (m_outputType == OutputType::PrinterPreview) {
        QPrinterInfo printerInfo;
        QPrinter     printer(printerInfo);

        QPrintPreviewDialog previewDialog(&printer);

        this->connect(&previewDialog, &QPrintPreviewDialog::paintRequested, [textDocument](QPrinter *printer) {
            textDocument->print(printer);
        });

        previewDialog.exec();
    }


    // Print :

    if (m_outputType == OutputType::Printer) {
        QPrinterInfo printerInfo;
        QPrinter     printer(printerInfo);

        QPrintDialog printDialog(&printer);

        if (printDialog.exec() == QDialog::Accepted) {
            textDocument->print(&printer);
        }
    }


#endif // SKR_PRINT_SUPPORT
}

// --------------------------------------------------------------------------------------

void SKRExporter::createContent(QTextDocument *textDocument, SKR::ItemType paperType, int projectId,
                                int paperId, QList<int> *numbers) {
    PLMPaperHub *paperHub;

    if (paperType == SKR::Sheet) {
        paperHub = plmdata->sheetHub();
    }
    else if (paperType == SKR::Note) {
        paperHub = plmdata->noteHub();
    }
    else {
        return;
    }

    QTextCursor textCursor(textDocument);

    textCursor.movePosition(QTextCursor::End);


    int indent = paperHub->getIndent(projectId, paperId);


    if ((m_indentWithTitle > indent) || (paperType == SKR::Note)) {
        // number :

        // increment :
        while (indent + 1 >= numbers->count()) {
            numbers->append(0);
        }


        // reset following numbers
        numbers->replace(indent, numbers->at(indent) + 1);

        for (int v = indent + 1; v < numbers->count(); v++) {
            numbers->replace(v, 0);
        }

        // create string :
        QStringList numberStringList;

        for (int i = 0; i <= indent; i++) {
            numberStringList.append(QString::number(numbers->at(i)));
        }
        QString numberString = numberStringList.join(".");

        // title :

        QString title =  paperHub->getTitle(projectId, paperId);

        if (paperType == SKR::Note) {
            if (paperId == plmdata->noteHub()->getSynopsisFolderId(projectId)) {
                title = tr("Outlines");
            }
        }

        QTextDocument titleDoc;

        if (m_numbered) {
            titleDoc.setPlainText(QString("%1. %2").arg(numberString).arg(title));
        }
        else {
            titleDoc.setPlainText(QString("%1").arg(title));
        }

        QTextBlockFormat titleBlockFormat;
        titleBlockFormat.setAlignment(Qt::AlignHCenter);
        titleBlockFormat.setHeadingLevel(indent + 2);
        textCursor.insertBlock(titleBlockFormat, m_charFormat);
        textCursor.insertFragment(QTextDocumentFragment(&titleDoc));
        textCursor.insertBlock(m_blockFormat, m_charFormat);
        textCursor.insertBlock(m_blockFormat, m_charFormat);
    }

    // tags :


    if (m_tagsEnabled) {
        QTextDocument tagsDoc;
        QList<int>    tagsIdList = plmdata->tagHub()->getTagsFromItemId(projectId, paperType, paperId);


        if (!tagsIdList.isEmpty()) {
            QStringList tagsStringList;

            for (int tagId : tagsIdList) {
                tagsStringList.append(plmdata->tagHub()->getTagName(projectId, tagId));
            }
            tagsStringList.sort(Qt::CaseInsensitive);

            QString tagsMd = tagsStringList.join(" ; ");

            tagsDoc.setMarkdown(tagsMd);
            QTextCursor cursor(&tagsDoc);
            cursor.select(QTextCursor::Document);
            cursor.mergeCharFormat(m_charFormat);
            cursor.mergeBlockFormat(m_blockFormat);
            cursor.movePosition(QTextCursor::Start);

            QTextBlockFormat blockFormat;
            blockFormat.setAlignment(Qt::AlignLeft);
            QTextCharFormat textCharFormat;
            textCharFormat.setFontItalic(true);
            cursor.insertBlock(blockFormat, textCharFormat);
            cursor.insertText(tr("Tags:"), textCharFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);

            textCursor.movePosition(QTextCursor::End);
            textCursor.insertFragment(QTextDocumentFragment(&tagsDoc));
            textCursor.insertBlock(m_blockFormat, m_charFormat);
        }
    }

    bool synopsisIsEmpty = true;

    if ((paperType == SKR::Sheet) && m_includeSynopsis) {
        QTextDocument synopsisDoc;
        int synopsisId     = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);
        QString synopsisMd = plmdata->noteHub()->getContent(projectId, synopsisId);
        synopsisIsEmpty = synopsisMd.isEmpty();

        if (!synopsisIsEmpty) {
            synopsisDoc.setMarkdown(synopsisMd);
            QTextCursor cursor(&synopsisDoc);

            cursor.select(QTextCursor::Document);
            cursor.mergeCharFormat(m_charFormat);
            cursor.mergeBlockFormat(m_blockFormat);
            cursor.movePosition(QTextCursor::Start);

            QTextBlockFormat blockFormat;
            blockFormat.setAlignment(Qt::AlignLeft);
            QTextCharFormat textCharFormat;
            textCharFormat.setFontItalic(true);
            cursor.insertBlock(blockFormat, textCharFormat);
            cursor.insertText(tr("Outline:"), textCharFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);

            textCursor.movePosition(QTextCursor::End);
            textCursor.insertFragment(QTextDocumentFragment(&synopsisDoc));
            textCursor.insertBlock(m_blockFormat, m_charFormat);
        }
    }

    // content :
    QTextDocument contentDoc;
    QString contentMd = paperHub->getContent(projectId, paperId);

    if (!contentMd.isEmpty()) {
        contentDoc.setMarkdown(contentMd);
        QTextCursor cursor(&contentDoc);
        cursor.select(QTextCursor::Document);
        qDebug() << "cursor.selectedText()" << cursor.selectedText().count();
        cursor.mergeBlockFormat(m_blockFormat);
        cursor.mergeCharFormat(m_charFormat);

        if ((paperType == SKR::Sheet) && m_includeSynopsis && !synopsisIsEmpty) {


            cursor.movePosition(QTextCursor::Start);
            QTextBlockFormat blockFormat;
            blockFormat.setAlignment(Qt::AlignLeft);
            QTextCharFormat textCharFormat;
            textCharFormat.setFontItalic(true);
            cursor.insertBlock(blockFormat, textCharFormat);
            cursor.insertText(tr("Text:"), textCharFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);
            cursor.insertBlock(m_blockFormat, m_charFormat);
        }


        textCursor.movePosition(QTextCursor::End);
        textCursor.insertFragment(QTextDocumentFragment(&contentDoc));
        textCursor.insertBlock(m_blockFormat, m_charFormat);
    }

    emit progressChanged(1);
}

int SKRExporter::textIndent() const
{
    return m_textIndent;
}

void SKRExporter::setTextIndent(int textIndent)
{
    m_textIndent = textIndent;
    emit textIndentChanged(textIndent);
}

int SKRExporter::textTopMargin() const
{
    return m_textTopMargin;
}

void SKRExporter::setTextTopMargin(int textTopMargin)
{
    m_textTopMargin = textTopMargin;
    emit textTopMarginChanged(textTopMargin);
}

bool SKRExporter::quick() const
{
    return m_quick;
}

void SKRExporter::setQuick(bool quick)
{
    m_quick = quick;
    emit quickChanged(quick);
}

bool SKRExporter::numbered() const
{
    return m_numbered;
}

void SKRExporter::setNumbered(bool numbered)
{
    m_numbered = numbered;
}

bool SKRExporter::tagsEnabled() const
{
    return m_tagsEnabled;
}

void SKRExporter::setTagsEnabled(bool tagsEnabled)
{
    m_tagsEnabled = tagsEnabled;
    emit tagsEnabledChanged(tagsEnabled);
}

int SKRExporter::fontPointSize() const
{
    return m_fontPointSize;
}

void SKRExporter::setFontPointSize(int fontPointSize)
{
    m_fontPointSize = fontPointSize;
    emit fontPointSizeChanged(fontPointSize);
}

QString SKRExporter::fontFamily() const
{
    return m_fontFamily;
}

void SKRExporter::setFontFamily(const QString& fontFamily)
{
    m_fontFamily = fontFamily;
    emit fontFamilyChanged(fontFamily);
}

int SKRExporter::indentWithTitle() const
{
    return m_indentWithTitle;
}

void SKRExporter::setIndentWithTitle(int indentWithTitle)
{
    m_indentWithTitle = indentWithTitle;
    emit indentWithTitleChanged(indentWithTitle);
}

int SKRExporter::projectId() const
{
    return m_projectId;
}

void SKRExporter::setProjectId(int projectId)
{
    m_projectId = projectId;
    emit projectIdChanged(projectId);
}

// --------------------------------------------------------------------------------------

bool SKRExporter::printEnabled() const
{
    return m_printEnabled;
}

void SKRExporter::setPrintEnabled(bool printEnabled)
{
    m_printEnabled = printEnabled;
    emit printEnabledChanged(printEnabled);
}

SKRExporter::OutputType SKRExporter::outputType() const
{
    return m_outputType;
}

void SKRExporter::setOutputType(const SKRExporter::OutputType& outputType)
{
    m_outputType = outputType;
    emit outputTypeChanged(outputType);
}

QList<int>SKRExporter::sheetIdList() const
{
    return m_sheetIdList;
}

void SKRExporter::setSheetIdList(const QList<int>& sheetIdList)
{
    m_sheetIdList = sheetIdList;
    emit sheetIdListChanged(sheetIdList);
}

QList<int>SKRExporter::noteIdList() const
{
    return m_noteIdList;
}

void SKRExporter::setNoteIdList(const QList<int>& noteIdList)
{
    m_noteIdList = noteIdList;
    emit noteIdListChanged(noteIdList);
}

bool SKRExporter::includeSynopsis() const
{
    return m_includeSynopsis;
}

void SKRExporter::setIncludeSynopsis(bool includeSynopsis)
{
    m_includeSynopsis = includeSynopsis;
    emit includeSynopsisChanged(includeSynopsis);
}

QUrl SKRExporter::outputUrl() const
{
    return m_outputUrl;
}

void SKRExporter::setOutputUrl(const QUrl& outputUrl)
{
    m_outputUrl = outputUrl;
    emit outputUrlChanged(outputUrl);
}
