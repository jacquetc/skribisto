#include "skrwordmeter.h"
#include "text/markdowntextdocument.h"

#include <QTextDocument>

SKRWordMeterWorker::SKRWordMeterWorker(QObject       *parent,
                                       const TreeItemAddress &treeItemAddress,
                                       const QString&& text,
                                       bool           triggerProjectModifiedSignal) :
    QThread(parent), m_treeItemAddress(treeItemAddress), m_text(text),
    m_triggerProjectModifiedSignal(triggerProjectModifiedSignal)
{}

void SKRWordMeterWorker::countWords()
{
    MarkdownTextDocument textDocument;

    textDocument.setSkribistoMarkdown(m_text);
    QString plainText = textDocument.toPlainText();

    plainText.replace(QRegularExpression("\\n+|\\t+"), " ");
    plainText = plainText.trimmed();
    int wordCount = plainText.count(" ");

    if (wordCount == 0) {
        if (plainText.isEmpty()) {
            wordCount = 0;
        }
        else {
            wordCount = 1;
        }
    }
    else {
        wordCount += 1;
    }

    emit wordCountCalculated(m_treeItemAddress, wordCount, m_triggerProjectModifiedSignal);
}

void SKRWordMeterWorker::countCharacters()
{
    MarkdownTextDocument textDocument;

    textDocument.setSkribistoMarkdown(m_text);
    QString plainText = textDocument.toPlainText();

    int charCount = plainText.size();

    emit characterCountCalculated(m_treeItemAddress, charCount,  m_triggerProjectModifiedSignal);
}

void SKRWordMeterWorker::run()
{
    countWords();
    countCharacters();
}

SKRWordMeter::SKRWordMeter(QObject *parent) : QObject(parent)
{}

void SKRWordMeter::countText(const TreeItemAddress &treeItemAddress,
                             const QString&& text,
                             bool           sameThread,
                             bool           triggerProjectModifiedSignal)
{
    SKRWordMeterWorker *worker = new SKRWordMeterWorker(this, treeItemAddress,
                                                        std::move(text),
                                                        triggerProjectModifiedSignal);

    if (sameThread) {
        connect(worker,
                &SKRWordMeterWorker::wordCountCalculated,
                this,
                &SKRWordMeter::wordCountCalculated,
                Qt::DirectConnection);
        connect(worker,
                &SKRWordMeterWorker::characterCountCalculated,
                this,
                &SKRWordMeter::characterCountCalculated,
                Qt::DirectConnection);
        worker->countWords();
        worker->countCharacters();
    }
    else {
        connect(worker,
                &SKRWordMeterWorker::finished,
                worker,
                &QObject::deleteLater,
                Qt::QueuedConnection);
        connect(worker,
                &SKRWordMeterWorker::wordCountCalculated,
                this,
                &SKRWordMeter::wordCountCalculated,
                Qt::QueuedConnection);
        connect(worker,
                &SKRWordMeterWorker::characterCountCalculated,
                this,
                &SKRWordMeter::characterCountCalculated,
                Qt::QueuedConnection);
        worker->start();
    }
}
