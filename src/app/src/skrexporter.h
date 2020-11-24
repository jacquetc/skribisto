#ifndef SKREXPORTER_H
#define SKREXPORTER_H

#include <QObject>
#include <QUrl>
#include <QTextDocument>
#include <QTextCharFormat>
#include "skr.h"

class SKRExporter : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool projectId MEMBER m_projectId READ projectId WRITE setProjectId NOTIFY projectIdChanged)
    Q_PROPERTY(bool printEnabled MEMBER m_printEnabled READ printEnabled WRITE setPrintEnabled NOTIFY printEnabledChanged)
    Q_PROPERTY(SKRExporter::OutputType outputType MEMBER m_outputType READ outputType WRITE setOutputType NOTIFY outputTypeChanged)
    Q_PROPERTY(QUrl outputUrl MEMBER m_outputUrl READ outputUrl WRITE setOutputUrl NOTIFY outputUrlChanged)
    Q_PROPERTY(QList<int> sheetIdList MEMBER m_sheetIdList READ sheetIdList WRITE setSheetIdList NOTIFY sheetIdListChanged)
    Q_PROPERTY(QList<int> noteIdList MEMBER m_noteIdList READ noteIdList WRITE setNoteIdList NOTIFY noteIdListChanged)
    Q_PROPERTY(bool includeSynopsis MEMBER m_includeSynopsis READ includeSynopsis WRITE setIncludeSynopsis NOTIFY includeSynopsisChanged)
    Q_PROPERTY(int indentWithTitle MEMBER m_indentWithTitle READ indentWithTitle WRITE setIndentWithTitle NOTIFY indentWithTitleChanged)
    Q_PROPERTY(QString fontFamily MEMBER m_fontFamily READ fontFamily WRITE setFontFamily NOTIFY fontFamilyChanged)
    Q_PROPERTY(int fontPointSize MEMBER m_fontPointSize READ fontPointSize WRITE setFontPointSize NOTIFY fontPointSizeChanged)
    Q_PROPERTY(bool tagsEnabled MEMBER m_tagsEnabled READ tagsEnabled WRITE setTagsEnabled NOTIFY tagsEnabledChanged)
    Q_PROPERTY(bool numbered MEMBER m_numbered READ numbered WRITE setNumbered NOTIFY numberedChanged)
    Q_PROPERTY(bool quick MEMBER m_quick READ quick WRITE setQuick NOTIFY quickChanged)
    Q_PROPERTY(int textTopMargin MEMBER m_textTopMargin READ textTopMargin WRITE setTextTopMargin NOTIFY textTopMarginChanged)
    Q_PROPERTY(int textIndent MEMBER m_textIndent READ textIndent WRITE setTextIndent NOTIFY textIndentChanged)


public:

    enum OutputType {
        Odt,
        Txt,
        Html,
        Md,
        Pdf,
        Printer,
        PrinterPreview
    };
    Q_ENUM(OutputType)


    explicit SKRExporter(QObject *parent = nullptr);
    Q_INVOKABLE void run();


    bool printEnabled() const;
    void setPrintEnabled(bool printEnabled);

    SKRExporter::OutputType outputType() const;
    void setOutputType(const SKRExporter::OutputType &outputType);

    QList<int> sheetIdList() const;
    void setSheetIdList(const QList<int> &sheetIdList);

    QList<int> noteIdList() const;
    void setNoteIdList(const QList<int> &noteIdList);



    bool includeSynopsis() const;
    void setIncludeSynopsis(bool includeSynopsis);

    QUrl outputUrl() const;
    void setOutputUrl(const QUrl &outputUrl);

    int projectId() const;
    void setProjectId(int projectId);

    int indentWithTitle() const;
    void setIndentWithTitle(int indentWithTitle);

    QString fontFamily() const;
    void setFontFamily(const QString &fontFamily);

    int fontPointSize() const;
    void setFontPointSize(int fontPointSize);

    bool tagsEnabled() const;
    void setTagsEnabled(bool tagsEnabled);

    bool numbered() const;
    void setNumbered(bool numbered);

    bool quick() const;
    void setQuick(bool quick);

    int textTopMargin() const;
    void setTextTopMargin(int textTopMargin);

    int textIndent() const;
    void setTextIndent(int textIndent);

signals:
    void projectIdChanged(int projectId);
    void printEnabledChanged(bool value);
    void outputTypeChanged(OutputType value);
    void progressChanged(int value);
    void progressMaximumChanged(int value);
    void outputUrlChanged(QUrl url);
    void sheetIdListChanged(QList<int> list);
    void noteIdListChanged(QList<int> list);
    void includeSynopsisChanged(bool value);
    void indentWithTitleChanged(int indent);
    void fontFamilyChanged(QString fontFamily);
    void fontPointSizeChanged(int pointSize);
    void tagsEnabledChanged(bool value);
    void numberedChanged(bool value);
    void quickChanged(bool value);
    void textTopMarginChanged(int value);
    void textIndentChanged(int value);

private:
    void createContent(QTextDocument *textDocument, SKR::ItemType paperType, int projectId, int paperId, QList<int> *numbers);

private:
    int m_projectId;
    bool m_printEnabled;
    OutputType m_outputType;
    QUrl m_outputUrl;
    QList<int> m_sheetIdList, m_noteIdList;
    int m_indentWithTitle;
    bool m_includeSynopsis;
    QString m_fontFamily;
    int m_fontPointSize;
    QTextCharFormat m_charFormat;
    QTextBlockFormat m_blockFormat;
    bool m_tagsEnabled;
    bool m_numbered;
    bool m_quick;
    int m_textTopMargin;
    int m_textIndent;

};

#endif // SKREXPORTER_H
