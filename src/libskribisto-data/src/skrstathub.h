#ifndef SKRSTATHUB_H
#define SKRSTATHUB_H

#include <QObject>
#include "interfaces/pageinterface.h"
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
    ~SKRStatHub();

    Q_INVOKABLE int getProjectTotalCount(SKRStatHub::StatType type,
                                          int                  project);

public slots:

    void updateStats(const TreeItemAddress &treeItemAddress, SKRStatHub::StatType type,
                              int  count               = -1,
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
    QList<PageInterface *> m_pageInterfacePluginList;
    QTimer *m_setPropertyTimer;
    QSet<TreeItemAddress> m_countPropertiesToSet;
};

#endif // SKRSTATHUB_H
