#ifndef MARKDOWNTEXTDOCUMENT_H
#define MARKDOWNTEXTDOCUMENT_H

#include <QTextDocument>
#include "skribisto_common_global.h"

class SKRCOMMONEXPORT MarkdownTextDocument : public QTextDocument
{
public:
    enum CharFormat {
             Italic = 0x0,
             Bold = 0x1,
             Underline = 0x2,
             Strikethrough = 0x4
         };
         Q_DECLARE_FLAGS(CharFormats, CharFormat)


    explicit MarkdownTextDocument(QObject *parent = nullptr);

    QString toSkribistoMarkdown() const;
    void setSkribistoMarkdown(const QString &markdownText);

protected:
    // fromSkribistoMarkdown()
    QString cleanUpMarkdown(const QString &markdownText) const;
    QStringList createBlockListFromMarkdown(const QString &markdownText) const;
    QList<QTextBlock> createBlocks(const QStringList &blockStringList);
    void formatBlock(const QTextBlock &block);
    void formatCharsInBlock(const QTextBlock &block);

    // skribistoMarkdown()
    QString convertFromBlockToString(const QTextBlock &block) const;
    QStringList breakStrings(const QStringList &blockStringList) const;
};
Q_DECLARE_OPERATORS_FOR_FLAGS(MarkdownTextDocument::CharFormats)

#endif // MARKDOWNTEXTDOCUMENT_H
