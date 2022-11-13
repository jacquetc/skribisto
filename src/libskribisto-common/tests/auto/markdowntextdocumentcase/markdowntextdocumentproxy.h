#ifndef MARKDOWNTEXTDOCUMENTPROXY_H
#define MARKDOWNTEXTDOCUMENTPROXY_H

#include "text/markdowntextdocument.h"

class MarkdownTextDocumentProxy : public MarkdownTextDocument {


public:

    MarkdownTextDocumentProxy(){

    }
    QString cleanUpMarkdown(const QString &markdownText) const
    {
        return MarkdownTextDocument::cleanUpMarkdown(markdownText);
    }
    QStringList createBlockListFromMarkdown(const QString &markdownText) const
    {
        return  MarkdownTextDocument::createBlockListFromMarkdown(markdownText);
    }
    QList<QTextBlock> createBlocks(const QStringList &blockStringList)
    {
        return  MarkdownTextDocument::createBlocks(blockStringList);

    }
    void formatBlock(const QTextBlock &block)
    {
        MarkdownTextDocument::formatBlock(block);

    }
    void formatCharsInBlock(const QTextBlock &block)
    {
        MarkdownTextDocument::formatCharsInBlock(block);
    }


};

#endif // MARKDOWNTEXTDOCUMENTPROXY_H
