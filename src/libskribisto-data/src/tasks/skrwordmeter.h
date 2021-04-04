#ifndef SKRWORDMETER_H
#define SKRWORDMETER_H

#include <QObject>
#include "skr.h"


class SKRWordMeterWorker : public QThread {
    Q_OBJECT

public:

    SKRWordMeterWorker(QObject             *parent,
                       const SKR::ItemType& paperType,
                       int                  projectId,
                       int                  paperId,
                       const QString      & text,
                       bool                 triggerProjectModifiedSignal);

public slots:

    void countWords();
    void countCharacters();

signals:

    void wordCountCalculated(SKR::ItemType paperType,
                             int           projectId,
                             int           paperId,
                             int           wordCount,
                             bool          triggerProjectModifiedSignal);
    void characterCountCalculated(SKR::ItemType paperType,
                                  int           projectId,
                                  int           paperId,
                                  int           charCount,
                                  bool          triggerProjectModifiedSignal);
    void finished();

private:

    void run() override;

    SKR::ItemType m_paperType;
    int m_projectId;
    int m_paperId;
    QString m_text;
    bool m_triggerProjectModifiedSignal;
};

class SKRWordMeter : public QObject {
    Q_OBJECT

public:

    explicit SKRWordMeter(QObject *parent = nullptr);
    void countText(const SKR::ItemType& paperType,
                   int                  projectId,
                   int                  paperId,
                   const QString      & text,
                   bool                 sameThread,
                   bool                 triggerProjectModifiedSignal = true);

signals:

    void wordCountCalculated(SKR::ItemType paperType,
                             int           projectId,
                             int           paperId,
                             int           wordCount,
                             bool          triggerProjectModifiedSignal);
    void characterCountCalculated(SKR::ItemType paperType,
                                  int           projectId,
                                  int           paperId,
                                  int           charCount,
                                  bool          triggerProjectModifiedSignal);

private:

    QThread m_workerThread;
};

#endif // SKRWORDMETER_H
