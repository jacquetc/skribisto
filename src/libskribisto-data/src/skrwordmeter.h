#ifndef SKRWORDMETER_H
#define SKRWORDMETER_H

#include <QObject>
#include "skr.h"
#include "skribisto_data_global.h"


class SKRWordMeterWorker : public QThread {
    Q_OBJECT

public:

    SKRWordMeterWorker(QObject       *parent,
                       int            projectId,
                       int            treeItemId,
                       const QString& text,
                       bool           triggerProjectModifiedSignal);

public slots:

    void countWords();
    void countCharacters();

signals:

    void wordCountCalculated(int  projectId,
                             int  treeItemId,
                             int  wordCount,
                             bool triggerProjectModifiedSignal);
    void characterCountCalculated(int  projectId,
                                  int  treeItemId,
                                  int  charCount,
                                  bool triggerProjectModifiedSignal);
    void finished();

private:

    void run() override;

    int m_projectId;
    int m_treeItemId;
    QString m_text;
    bool m_triggerProjectModifiedSignal;
};

class EXPORT SKRWordMeter : public QObject {
    Q_OBJECT

public:

    explicit SKRWordMeter(QObject *parent = nullptr);
    void countText(int            projectId,
                   int            treeItemId,
                   const QString& text,
                   bool           sameThread,
                   bool           triggerProjectModifiedSignal = true);

signals:

    void wordCountCalculated(int  projectId,
                             int  treeItemId,
                             int  wordCount,
                             bool triggerProjectModifiedSignal);
    void characterCountCalculated(int  projectId,
                                  int  treeItemId,
                                  int  charCount,
                                  bool triggerProjectModifiedSignal);

private:

    QThread m_workerThread;
};

#endif // SKRWORDMETER_H
