#include "skrstathub.h"
#include "skrdata.h"

SKRStatHub::SKRStatHub(QObject *parent) : QObject(parent)
{
    connect(skrdata->treeHub(), &SKRTreeHub::trashedChanged,
            this, &SKRStatHub::updateTreeItemCounts);

    connect(skrdata->treeHub(), &SKRTreeHub::treeItemRemoved,
            this, &SKRStatHub::removeTreeItemFromStat);

    connect(skrdata->treePropertyHub(), &SKRPropertyHub::propertyChanged, this, [this]( int projectId,
            int            propertyId,
            int            treeItemCode,
            const QString& name,
            const QString& value)
    {
        if (name == "printable")
            updateTreeItemCounts(projectId, treeItemCode);

    }
    );
}

int SKRStatHub::getTreeItemTotalCount(SKRStatHub::StatType type, int project)
{
    int totalCount                                = 0;
    QHash<int, QHash<QString, int> > treeItemHash = m_treeItemHashByProjectHash.value(project,
                                                                                      QHash<int,
                                                                                      QHash<QString, int> >());

    QHash<int, QHash<QString, int> >::const_iterator i = treeItemHash.constBegin();

    while (i != treeItemHash.constEnd()) {
        QHash<QString, int> hash = i.value();

        if (!hash.value("isTrashed", 0) && hash.value("isPrintable", 1)) {
            switch (type) {
            case SKRStatHub::Character:
                totalCount += hash.value("characterCount", 0);
                break;

            case SKRStatHub::Word:
                totalCount += hash.value("wordCount", 0);
                break;
            }
        }
        ++i;
    }

    return totalCount;
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateWordStats(int  projectId,
                                 int  treeItemId,
                                 int  wordCount,
                                 bool triggerProjectModifiedSignal)
{
    QHash<int, QHash<QString, int> > projectHash = m_treeItemHashByProjectHash.value(projectId);
    SKRPropertyHub *propertyHub                  = skrdata->treePropertyHub();
    SKRTreeHub     *treeHub                      = skrdata->treeHub();


    // ------------- get trashed
    bool isTrashed = treeHub->getTrashed(projectId, treeItemId);
    bool isPrintable = propertyHub->getProperty(projectId, treeItemId, "printable", "true") == "true" ? true : false;

    // ------------- update word_count

    if (wordCount != -1) {
        propertyHub->setProperty(projectId,
                                 treeItemId,
                                 "word_count",
                                 QString::number(wordCount),
                                 true,
                                 triggerProjectModifiedSignal);
    }

    // ------------- update general stats

    QHash<QString, int> treeItemHash = projectHash.value(treeItemId);

    if (wordCount != -1) {
        treeItemHash.insert("wordCount", wordCount);
    }
    treeItemHash.insert("isTrashed", isTrashed);
    treeItemHash.insert("isPrintable", isPrintable);
    projectHash.insert(treeItemId, treeItemHash);

    int totalCount = 0;

    m_treeItemHashByProjectHash.insert(projectId, projectHash);
    totalCount = getTreeItemTotalCount(SKRStatHub::Word, projectId);

    emit statsChanged(SKRStatHub::Word, projectId, totalCount);


    // -------------update parents' charCountWithChildren:

    // get all ancestors
    QList<int> ancestors = treeHub->getAllAncestors(projectId, treeItemId);

    ancestors.prepend(treeItemId);

    // for each ancestor, get all children

    for (int ancestorId : qAsConst(ancestors)) {
        QList<int> children = treeHub->getAllChildren(projectId, ancestorId);

        // remove trashed
        QMutableListIterator<int> i(children);

        while (i.hasNext()) {
            int childId                   = i.next();
            QHash<QString, int> childHash = projectHash.value(childId, QHash<QString, int>());

            if (childHash.value("isTrashed", 0) || !childHash.value("isPrintable", 1)) {
                i.remove();
            }
        }

        int totalChildrenCount = 0;

        for (int childId : qAsConst(children)) {
            QHash<QString, int> childHash = projectHash.value(childId, QHash<QString, int>());

            totalChildrenCount += childHash.value("wordCount", 0);
        }

        // add ancestor count
        QHash<QString, int> ancestorHash = projectHash.value(ancestorId, QHash<QString, int>());
        totalChildrenCount += ancestorHash.value("wordCount", 0);

        // set property

        propertyHub->setProperty(projectId, ancestorId, "word_count_with_children",
                                 QString::number(totalChildrenCount), true, triggerProjectModifiedSignal);
    }
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateCharacterStats(int  projectId,
                                      int  treeItemId,
                                      int  characterCount,
                                      bool triggerProjectModifiedSignal)
{
    QHash<int, QHash<QString, int> > projectHash = m_treeItemHashByProjectHash.value(projectId);
    SKRPropertyHub *propertyHub                  = skrdata->treePropertyHub();
    SKRTreeHub     *treeHub                      = skrdata->treeHub();

    // ------------- get trashed
    bool isTrashed = treeHub->getTrashed(projectId, treeItemId);
    bool isPrintable = propertyHub->getProperty(projectId, treeItemId, "printable", "true") == "true" ? true : false;

    // ------------- update char_count

    if (characterCount != -1) {
        propertyHub->setProperty(projectId,
                                 treeItemId,
                                 "char_count",
                                 QString::number(characterCount),
                                 true,
                                 triggerProjectModifiedSignal);
    }

    // ------------- update general stats

    QHash<QString, int> treeItemHash = projectHash.value(treeItemId);

    if (characterCount != -1) {
        treeItemHash.insert("characterCount", characterCount);
    }

    treeItemHash.insert("isTrashed", isTrashed);
    treeItemHash.insert("isPrintable", isPrintable);
    projectHash.insert(treeItemId, treeItemHash);

    int totalCount = 0;

    m_treeItemHashByProjectHash.insert(projectId, projectHash);
    totalCount = getTreeItemTotalCount(SKRStatHub::Character, projectId);

    emit statsChanged(SKRStatHub::Character, projectId, totalCount);

    // -------------update parents' charCountWithChildren:

    // get all ancerstors
    QList<int> ancestors = treeHub->getAllAncestors(projectId, treeItemId);

    ancestors.prepend(treeItemId);

    // for each ancestor, get all children

    for (int ancestorId : qAsConst(ancestors)) {
        QList<int> children = treeHub->getAllChildren(projectId, ancestorId);

        // remove trashed
        QMutableListIterator<int> i(children);

        while (i.hasNext()) {
            int childId                   = i.next();
            QHash<QString, int> childHash = projectHash.value(childId, QHash<QString, int>());

            if (childHash.value("isTrashed", 0) || !childHash.value("isPrintable", 1)) {
                i.remove();
            }
        }

        int totalChildrenCount = 0;

        for (int childId : qAsConst(children)) {
            QHash<QString, int> childHash = projectHash.value(childId, QHash<QString, int>());

            totalChildrenCount += childHash.value("characterCount", 0);
        }

        // add ancestor count
        QHash<QString, int> ancestorHash = projectHash.value(ancestorId, QHash<QString, int>());
        totalChildrenCount += ancestorHash.value("characterCount", 0);

        // set property

        propertyHub->setProperty(projectId, ancestorId, "char_count_with_children",
                                 QString::number(totalChildrenCount), true, triggerProjectModifiedSignal);
    }
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateTreeItemCounts(int projectId, int treeItemId)
{
    updateCharacterStats(projectId, treeItemId);
    updateWordStats(projectId, treeItemId);
}

// ---------------------------------------------------------------------------------

void SKRStatHub::removeTreeItemFromStat(int projectId, int treeItemId)
{
    //  needed for cleaner calculation
    updateTreeItemCounts(projectId, treeItemId);

    // delete ref to treeItemId
    QHash<int, QHash<QString, int> > projectHash = m_treeItemHashByProjectHash.value(projectId);

    projectHash.remove(treeItemId);
    m_treeItemHashByProjectHash.insert(projectId, projectHash);
}
