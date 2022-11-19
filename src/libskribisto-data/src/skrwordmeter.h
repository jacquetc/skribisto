#ifndef SKRWORDMETER_H
#define SKRWORDMETER_H

#include <QObject>
#include "skr.h"
#include "skribisto_data_global.h"
#include "treeitemaddress.h"


class SKRWordMeterWorker : public QThread {
    Q_OBJECT

public:

    SKRWordMeterWorker(QObject       *parent,
                       const TreeItemAddress &treeItemAddress,
                       const QString&& text,
                       bool           triggerProjectModifiedSignal);

public slots:

    void countWords();
    void countCharacters();

signals:

    void wordCountCalculated(const TreeItemAddress &treeItemAddress,
                             int  wordCount,
                             bool triggerProjectModifiedSignal);
    void characterCountCalculated(const TreeItemAddress &treeItemAddress,
                                  int  charCount,
                                  bool triggerProjectModifiedSignal);

private:

    void run() override;
    TreeItemAddress m_treeItemAddress;
    QString m_text;
    bool m_triggerProjectModifiedSignal;
};

class EXPORT SKRWordMeter : public QObject {
    Q_OBJECT

public:

    explicit SKRWordMeter(QObject *parent = nullptr);
    void countText(const TreeItemAddress &treeItemAddress,
                   const QString&& text,
                   bool           sameThread,
                   bool           triggerProjectModifiedSignal = true);

signals:

    void wordCountCalculated(const TreeItemAddress &treeItemAddress,
                             int  wordCount,
                             bool triggerProjectModifiedSignal);
    void characterCountCalculated(const TreeItemAddress &treeItemAddress,
                                  int  charCount,
                                  bool triggerProjectModifiedSignal);

private:

    QThread m_workerThread;
};

#endif // SKRWORDMETER_H
