#include "skrwordmeter.h"

#include <QTextDocument>

SKRWordMeterWorker::SKRWordMeterWorker(QObject *parent, const SKR::PaperType &paperType, int projectId, int paperId, const QString &text) :
    QThread(parent), m_paperType(paperType), m_projectId(projectId), m_paperId(paperId), m_text(text)
{

}

void SKRWordMeterWorker::countWords()
{
    QTextDocument textDocument;
    textDocument.setMarkdown(m_text);
    QString plainText = textDocument.toPlainText();

    plainText.replace(QRegularExpression("\\n*|\\t"), " ");

    int wordCount = plainText.count(" ");

    emit wordCountCalculated(m_paperType, m_projectId, m_paperId, wordCount);
}

void SKRWordMeterWorker::countCharacters()
{
    QTextDocument textDocument;
    textDocument.setMarkdown(m_text);
    QString plainText = textDocument.toPlainText();

    int charCount = plainText.count();

    emit characterCountCalculated(m_paperType, m_projectId, m_paperId, charCount);
}

void SKRWordMeterWorker::run()
{

    countWords();
    countCharacters();
}

SKRWordMeter::SKRWordMeter(QObject *parent) : QObject(parent)
{

}

void SKRWordMeter::countText(const SKR::PaperType &paperType, int projectId, int paperId, const QString &text, bool sameThread)
{


    SKRWordMeterWorker *worker = new SKRWordMeterWorker(this, paperType, projectId, paperId, text);

    if(sameThread){
        connect(worker, &SKRWordMeterWorker::wordCountCalculated, this, &SKRWordMeter::wordCountCalculated, Qt::DirectConnection);
        connect(worker, &SKRWordMeterWorker::characterCountCalculated, this, &SKRWordMeter::characterCountCalculated, Qt::DirectConnection);
        worker->countWords();
        worker->countCharacters();
    }
    else {
        connect(worker, &SKRWordMeterWorker::finished, worker, &QObject::deleteLater, Qt::QueuedConnection);
        connect(worker, &SKRWordMeterWorker::wordCountCalculated, this, &SKRWordMeter::wordCountCalculated, Qt::QueuedConnection);
        connect(worker, &SKRWordMeterWorker::characterCountCalculated, this, &SKRWordMeter::characterCountCalculated, Qt::QueuedConnection);
        worker->start();
    }


}
