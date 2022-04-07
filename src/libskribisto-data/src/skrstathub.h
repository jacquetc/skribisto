#ifndef SKRSTATHUB_H
#define SKRSTATHUB_H

#include <QObject>
#include "skr.h"
#include "skribisto_data_global.h"

class EXPORT SKRStatHub : public QObject {
    Q_OBJECT

public:

    enum StatType {
        Character,
        Word
    };
    Q_ENUM(StatType)

    explicit SKRStatHub(QObject *parent = nullptr);

    Q_INVOKABLE int getTreeItemTotalCount(SKRStatHub::StatType type,
                                          int                  project);

public slots:

    void updateWordStats(int  projectId,
                         int  treeItemId,
                         int  wordCount                    = -1,
                         bool triggerProjectModifiedSignal = true);
    void updateCharacterStats(int  projectId,
                              int  treeItemId,
                              int  characterCount               = -1,
                              bool triggerProjectModifiedSignal = true);

private slots:

    void updateTreeItemCounts(int  projectId,
                            int  treeItemId);
    void removeTreeItemFromStat(int projectId,
                                int treeItemId);

signals:

    void statsChanged(SKRStatHub::StatType statType,
                      int                  projectId,
                      int                  count);

private:

    QHash<int, QHash<int, QHash<QString, int> > >m_treeItemHashByProjectHash;
};

#endif // SKRSTATHUB_H
