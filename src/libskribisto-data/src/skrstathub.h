#ifndef SKRSTATHUB_H
#define SKRSTATHUB_H

#include <QObject>
#include "skr.h"
#include "skribisto_data_global.h"
#include "treeitemaddress.h"

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

    void updateWordStats(const TreeItemAddress &treeItemAddress,
                         int  wordCount                    = -1,
                         bool triggerProjectModifiedSignal = true);
    void updateCharacterStats(const TreeItemAddress &treeItemAddress,
                              int  characterCount               = -1,
                              bool triggerProjectModifiedSignal = true);

private slots:

    void updateTreeItemCounts(const TreeItemAddress &treeItemAddress);
    void removeTreeItemFromStat(const TreeItemAddress &treeItemAddress);

signals:

    void statsChanged(SKRStatHub::StatType statType,
                      int                  projectId,
                      int                  count);

private:

    QHash<TreeItemAddress, QVariantHash >m_treeItemAddressWithDataHash;
};

#endif // SKRSTATHUB_H
