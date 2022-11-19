#include "skrstathub.h"
#include "skrdata.h"

SKRStatHub::SKRStatHub(QObject *parent) : QObject(parent)
{
    connect(skrdata->treeHub(), &SKRTreeHub::trashedChanged,
            this, &SKRStatHub::updateTreeItemCounts, Qt::QueuedConnection);

    connect(skrdata->treeHub(), &SKRTreeHub::treeItemRemoved,
            this, &SKRStatHub::removeTreeItemFromStat, Qt::QueuedConnection);

    connect(skrdata->treePropertyHub(), &SKRPropertyHub::propertyChanged, this, [this]( int projectId,
            int            propertyId,
            int            treeItemCode,
            const QString& name,
            const QString& value)
    {
        if (name == "printable")
            updateTreeItemCounts(TreeItemAddress(projectId, treeItemCode));

    }, Qt::QueuedConnection
    );
}

int SKRStatHub::getTreeItemTotalCount(SKRStatHub::StatType type, int project)
{
    int totalCount                                = 0;


    QHash<TreeItemAddress, QVariantHash>::const_iterator i = m_treeItemAddressWithDataHash.constBegin();
    while (i != m_treeItemAddressWithDataHash.constEnd()) {
        const TreeItemAddress &address = i.key();
        if(address.projectId == project){
            const QVariantHash &data = i.value();

            if (!data.value("isTrashed", false).toBool() && data.value("isPrintable", true).toBool()) {
                switch (type) {
                case SKRStatHub::Character:
                    totalCount += data.value("characterCount", 0).toInt();
                    break;

                case SKRStatHub::Word:
                    totalCount += data.value("wordCount", 0).toInt();
                    break;
                }
            }

        }

        ++i;
    }


    return totalCount;
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateWordStats(const TreeItemAddress &treeItemAddress,
                                 int  wordCount,
                                 bool triggerProjectModifiedSignal)
{


    SKRPropertyHub *propertyHub                  = skrdata->treePropertyHub();
    SKRTreeHub     *treeHub                      = skrdata->treeHub();


    // ------------- get trashed
    bool isTrashed = treeHub->getTrashed(treeItemAddress);
    bool isPrintable = propertyHub->getProperty(treeItemAddress, "printable", "true") == "true" ? true : false;

    // ------------- update word_count

    if (wordCount != -1) {
        propertyHub->setProperty(treeItemAddress,
                                 "word_count",
                                 QString::number(wordCount),
                                 true,
                                 triggerProjectModifiedSignal);
    }

    // ------------- update general stats




    QVariantHash data = m_treeItemAddressWithDataHash.value(treeItemAddress);

    if (wordCount != -1) {
        data.insert("wordCount", wordCount);
    }
    data.insert("isTrashed", isTrashed);
    data.insert("isPrintable", isPrintable);

    m_treeItemAddressWithDataHash.insert(treeItemAddress, data);

    int totalCount = getTreeItemTotalCount(SKRStatHub::Word, treeItemAddress.projectId);
    emit statsChanged(SKRStatHub::Word, treeItemAddress.projectId, totalCount);


    // -------------update parents' charCountWithChildren:

    // get all ancestors
    QList<TreeItemAddress> ancestors = treeHub->getAllAncestors(treeItemAddress);

    ancestors.prepend(treeItemAddress);

    // for each ancestor, get all children


    int timerCounter = 1;
    for (const TreeItemAddress &ancestorId : qAsConst(ancestors)) {

        QList<TreeItemAddress> children = treeHub->getAllChildren(ancestorId);


        // remove trashed
        QMutableListIterator<TreeItemAddress> i(children);

        while (i.hasNext()) {
            TreeItemAddress childId                   = i.next();

            const QVariantHash &data = m_treeItemAddressWithDataHash.value(childId, QVariantHash());
            if (data.value("isTrashed", false).toBool() || !data.value("isPrintable", true).toBool()) {
                i.remove();
            }
        }

        int totalChildrenCount = 0;

        for (const TreeItemAddress &childId : qAsConst(children)) {
            const QVariantHash &childData = m_treeItemAddressWithDataHash.value(childId, QVariantHash());

            totalChildrenCount += childData.value("wordCount", 0).toInt();
        }

        // add ancestor count
        const QVariantHash &ancestorData = m_treeItemAddressWithDataHash.value(ancestorId, QVariantHash());
        totalChildrenCount += ancestorData.value("wordCount", 0).toInt();

        // set property

        QTimer::singleShot(50 + timerCounter * 50, this, [=](){
            propertyHub->setProperty(ancestorId, "word_count_with_children",
                                 QString::number(totalChildrenCount), true, triggerProjectModifiedSignal);
        });
    }

}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateCharacterStats(const TreeItemAddress &treeItemAddress,
                                      int  characterCount,
                                      bool triggerProjectModifiedSignal)
{

    SKRPropertyHub *propertyHub                  = skrdata->treePropertyHub();
    SKRTreeHub     *treeHub                      = skrdata->treeHub();


    // ------------- get trashed
    bool isTrashed = treeHub->getTrashed(treeItemAddress);
    bool isPrintable = propertyHub->getProperty(treeItemAddress, "printable", "true") == "true" ? true : false;

    // ------------- update char_count

    if (characterCount != -1) {
        propertyHub->setProperty(treeItemAddress,
                                 "char_count",
                                 QString::number(characterCount),
                                 true,
                                 triggerProjectModifiedSignal);
    }

    // ------------- update general stats


    QVariantHash data = m_treeItemAddressWithDataHash.value(treeItemAddress);

    if (characterCount != -1) {
        data.insert("wordCount", characterCount);
    }
    data.insert("isTrashed", isTrashed);
    data.insert("isPrintable", isPrintable);

    m_treeItemAddressWithDataHash.insert(treeItemAddress, data);

    int totalCount = getTreeItemTotalCount(SKRStatHub::Word, treeItemAddress.projectId);
    emit statsChanged(SKRStatHub::Word, treeItemAddress.projectId, totalCount);

    // -------------update parents' charCountWithChildren:

    // get all ancerstors
    QList<TreeItemAddress> ancestors = treeHub->getAllAncestors(treeItemAddress);

    ancestors.prepend(treeItemAddress);

    // for each ancestor, get all children


    for (const TreeItemAddress &ancestorId : qAsConst(ancestors)) {
        QList<TreeItemAddress> children = treeHub->getAllChildren(ancestorId);

        // remove trashed
        QMutableListIterator<TreeItemAddress> i(children);

        while (i.hasNext()) {
            TreeItemAddress childId                   = i.next();

            const QVariantHash &data = m_treeItemAddressWithDataHash.value(childId, QVariantHash());
            if (data.value("isTrashed", false).toBool() || !data.value("isPrintable", true).toBool()) {
                i.remove();
            }
        }

        int totalChildrenCount = 0;

        for (const TreeItemAddress &childId : qAsConst(children)) {
            const QVariantHash &childData = m_treeItemAddressWithDataHash.value(childId, QVariantHash());

            totalChildrenCount += childData.value("characterCount", 0).toInt();
        }

        // add ancestor count
        const QVariantHash &ancestorData = m_treeItemAddressWithDataHash.value(ancestorId, QVariantHash());
        totalChildrenCount += ancestorData.value("characterCount", 0).toInt();

        // set property

        propertyHub->setProperty(ancestorId, "char_count_with_children",
                                 QString::number(totalChildrenCount), true, triggerProjectModifiedSignal);
    }
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateTreeItemCounts(const TreeItemAddress &treeItemAddress)
{
    updateCharacterStats(treeItemAddress);
    updateWordStats(treeItemAddress);
}

// ---------------------------------------------------------------------------------

void SKRStatHub::removeTreeItemFromStat(const TreeItemAddress &treeItemAddress)
{
    //  needed for cleaner calculation
    updateTreeItemCounts(treeItemAddress);

    // delete ref to treeItemId
    m_treeItemAddressWithDataHash.remove(treeItemAddress);
}
