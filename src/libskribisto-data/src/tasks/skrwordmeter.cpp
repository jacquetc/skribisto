#include "skrwordmeter.h"

#include <QTextDocument>

SKRWordMeterWorker::SKRWordMeterWorker(QObject              *parent,
                                       const SKR::PaperType& paperType,
                                       int                   projectId,
                                       int                   paperId,
                                       const QString       & text,
                                       bool triggerProjectModifiedSignal) :
    QThread(parent), m_paperType(paperType), m_projectId(projectId), m_paperId(paperId), m_text(text),  m_triggerProjectModifiedSignal(triggerProjectModifiedSignal)
{}

void SKRWordMeterWorker::countWords()
{
    QTextDocument textDocument;

    textDocument.setMarkdown(m_text);
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

    emit wordCountCalculated(m_paperType, m_projectId, m_paperId, wordCount, m_triggerProjectModifiedSignal);
}

void SKRWordMeterWorker::countCharacters()
{
    QTextDocument textDocument;

    textDocument.setMarkdown(m_text);
    QString plainText = textDocument.toPlainText();

    int charCount = plainText.count();

    emit characterCountCalculated(m_paperType, m_projectId, m_paperId, charCount,  m_triggerProjectModifiedSignal);
}

void SKRWordMeterWorker::run()
{
    countWords();
    countCharacters();
    emit finished();
}

SKRWordMeter::SKRWordMeter(QObject *parent) : QObject(parent)
{}

void SKRWordMeter::countText(const SKR::PaperType& paperType,
                             int                   projectId,
                             int                   paperId,
                             const QString       & text,
                             bool                  sameThread,
                             bool triggerProjectModifiedSignal)
{
    SKRWordMeterWorker *worker = new SKRWordMeterWorker(this, paperType, projectId, paperId, text, triggerProjectModifiedSignal);

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
