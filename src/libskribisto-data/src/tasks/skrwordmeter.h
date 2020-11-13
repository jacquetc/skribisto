#ifndef SKRWORDMETER_H
#define SKRWORDMETER_H

#include <QObject>
#include "skr.h"



class SKRWordMeterWorker : public QThread
{
    Q_OBJECT

public:

    SKRWordMeterWorker(QObject *parent, const SKR::PaperType &paperType, int projectId, int paperId, const QString &text);

public slots:
    void countWords();
    void countCharacters();

        signals:
void wordCountCalculated(SKR::PaperType paperType, int projectId, int paperId, int wordCount);
void characterCountCalculated(SKR::PaperType paperType, int projectId, int paperId, int charCount);
void finished();

private:
void run() override;

SKR::PaperType m_paperType;
int m_projectId;
int m_paperId;
QString m_text;
};

class SKRWordMeter : public QObject
{
    Q_OBJECT
public:
    explicit SKRWordMeter(QObject *parent = nullptr);
    void countText(const SKR::PaperType &paperType, int projectId, int paperId, const QString &text, bool sameThread);

signals:
    void wordCountCalculated(SKR::PaperType paperType, int projectId, int paperId, int wordCount);
    void characterCountCalculated(SKR::PaperType paperType, int projectId, int paperId, int charCount);

private:
    QThread m_workerThread;

};

#endif // SKRWORDMETER_H
