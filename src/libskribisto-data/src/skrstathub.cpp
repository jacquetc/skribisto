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

    //------------------------------

    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();

    connect(skrdata->projectHub() ,&PLMProjectHub::projectLoaded, this, [this](int projectId){
        const QList<TreeItemAddress> &addresses = skrdata->treeHub()->getAllIds(projectId);

        for(const TreeItemAddress &address : addresses){
            bool isTrashed = skrdata->treeHub()->getTrashed(address);
            bool isPrintable = skrdata->treePropertyHub()->getProperty(address, "printable", "true") == "true" ? true : false;
            QVariantHash data;
            data.insert("is_trashed", isTrashed);
            data.insert("is_printable", isPrintable);
            data.insert("trigger_project_modified_signal", false);

            m_treeItemAddressWithDataHash.insert(address, data);
        }

        for(int i = addresses.count() - 1; i >= 0 ; i--){
            const TreeItemAddress &address = addresses.at(i);
            const QString &pageType = skrdata->treeHub()->getType(address);
            for(auto *plugin : m_pageInterfacePluginList){
                if(plugin->pageType() == pageType){
                    plugin->updateCharAndWordCount(address, true, false);
                    break;
                }
            }


        }

    });

    connect(skrdata->projectHub() ,&PLMProjectHub::projectToBeClosed, this, [this](int projectId){
        m_setPropertyTimer->stop();
        m_countPropertiesToSet.clear();


    });


    m_setPropertyTimer = new QTimer(this);
    m_setPropertyTimer->setInterval(1000);

    connect(m_setPropertyTimer, &QTimer::timeout, this, [this](){

        QStringList propertyNames;
        propertyNames << "char_count" << "char_count_with_children"  << "word_count"  << "word_count_with_children" ;

        QSet<TreeItemAddress>::const_iterator i =  m_countPropertiesToSet.constBegin();
        while (i != m_countPropertiesToSet.constEnd()) {
            const QVariantHash &data = m_treeItemAddressWithDataHash.value(*i);

            for(const QString &propertyName : propertyNames){
                if(data.contains(propertyName)){
                    //qDebug() << i.key() << propertyName << QString::number(data.value(propertyName).toInt());
                    skrdata->treePropertyHub()->setProperty(*i, propertyName,
                                             QString::number(data.value(propertyName).toInt()), true
                                                            , !data.value("trigger_project_modified_signal").toBool());
                }

            }

        i++;
        }
        m_countPropertiesToSet.clear();
    });

}

SKRStatHub::~SKRStatHub()
{
    m_setPropertyTimer->stop();

}

int SKRStatHub::getProjectTotalCount(SKRStatHub::StatType type, int project)
{
    int totalCount                                = 0;


    QHash<TreeItemAddress, QVariantHash>::const_iterator i = m_treeItemAddressWithDataHash.constBegin();
    while (i != m_treeItemAddressWithDataHash.constEnd()) {
        const TreeItemAddress &address = i.key();
        if(address.projectId == project){
            const QVariantHash &data = i.value();

            if (!data.value("is_trashed", false).toBool() && data.value("is_printable", true).toBool()) {
                switch (type) {
                case SKRStatHub::Character:
                    totalCount += data.value("char_count", 0).toInt();
                    break;

                case SKRStatHub::Word:
                    totalCount += data.value("word_count", 0).toInt();
                    break;
                }
            }

        }

        ++i;
    }


    return totalCount;
}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateStats(const TreeItemAddress &treeItemAddress, SKRStatHub::StatType type,
                                      int  count,
                                      bool triggerProjectModifiedSignal)
{

    QString countPropertyName;
    QString countWithChildrenPropertyName;
    switch (type) {
    case SKRStatHub::StatType::Character:
     countPropertyName = "char_count";
     countWithChildrenPropertyName = "char_count_with_children";
        break;
    case SKRStatHub::StatType::Word:
     countPropertyName = "word_count";
     countWithChildrenPropertyName = "word_count_with_children";
        break;
    default:
        break;
    }




    SKRPropertyHub *propertyHub                  = skrdata->treePropertyHub();
    SKRTreeHub     *treeHub                      = skrdata->treeHub();




    // ------------- get trashed
    bool isTrashed = treeHub->getTrashed(treeItemAddress);
    bool isPrintable = propertyHub->getProperty(treeItemAddress, "printable", "true") == "true" ? true : false;

    // ------------- update count

    if (count != -1) {

        QTimer::singleShot(50, this, [=](){

        propertyHub->setProperty(treeItemAddress,
                                 countPropertyName,
                                 QString::number(count),
                                 true,
                                 triggerProjectModifiedSignal);
        });
    }


    // ------------- update cache

    QVariantHash data = m_treeItemAddressWithDataHash.value(treeItemAddress);

    if (count != -1) {
        data.insert(countPropertyName, count);
    }
    data.insert("is_trashed", isTrashed);
    data.insert("is_printable", isPrintable);
    data.insert("trigger_project_modified_signal", triggerProjectModifiedSignal);

    m_treeItemAddressWithDataHash.insert(treeItemAddress, data);

    // ------------- update general stats
    int totalCount = getProjectTotalCount(type, treeItemAddress.projectId);
    emit statsChanged(type, treeItemAddress.projectId, totalCount);

    // -------------update parents', charCountWithChildren:





    // get all ancerstors
    QList<TreeItemAddress> ancestors = treeHub->getAllAncestors(treeItemAddress);

    // for each ancestor, get all children


    ancestors.prepend(treeItemAddress);

    for (const TreeItemAddress &ancestorId : qAsConst(ancestors)) {
        const QList<TreeItemAddress> &children = treeHub->getAllDirectChildren(ancestorId);

        int totalChildrenCount = 0;

        for (const TreeItemAddress &childId : qAsConst(children)) {

            // add chilId if none
            if(!m_treeItemAddressWithDataHash.contains(childId)){

                bool isTrashed = treeHub->getTrashed(childId);
                bool isPrintable = propertyHub->getProperty(childId, "printable", "true") == "true" ? true : false;
                QVariantHash data;
                data.insert("is_trashed", isTrashed);
                data.insert("is_printable", isPrintable);
                data.insert("trigger_project_modified_signal", triggerProjectModifiedSignal);

                m_treeItemAddressWithDataHash.insert(childId, data);
            }
            // use chilId
            const QVariantHash &childData = m_treeItemAddressWithDataHash.value(childId, QVariantHash());

            if(!childData.value("is_trashed", false).toBool() || childData.value("is_printable", true).toBool()){
                if(childData.value(countWithChildrenPropertyName, 0).toInt() > 0){
                    totalChildrenCount += childData.value(countWithChildrenPropertyName, 0).toInt();
                }
                else{
                    totalChildrenCount += childData.value(countPropertyName, 0).toInt();
                }
            }

        }

        // add ancestor count (and target's too)
        QVariantHash ancestorData = m_treeItemAddressWithDataHash.value(ancestorId, QVariantHash());
        totalChildrenCount += ancestorData.value(countPropertyName, 0).toInt();
        ancestorData.insert("trigger_project_modified_signal", triggerProjectModifiedSignal);
        ancestorData.insert(countWithChildrenPropertyName, totalChildrenCount);
        m_treeItemAddressWithDataHash.insert(ancestorId, ancestorData);
        // set property
        m_countPropertiesToSet.insert(ancestorId);
    }



    if(m_setPropertyTimer->isActive()){
        m_setPropertyTimer->stop();
    }
    m_setPropertyTimer->start();

}

// ---------------------------------------------------------------------------------

void SKRStatHub::updateTreeItemCounts(const TreeItemAddress &treeItemAddress)
{
    updateStats(treeItemAddress, SKRStatHub::Character);
    updateStats(treeItemAddress, SKRStatHub::Word);
}

// ---------------------------------------------------------------------------------

void SKRStatHub::removeTreeItemFromStat(const TreeItemAddress &treeItemAddress)
{
    //  needed for cleaner calculation
    updateTreeItemCounts(treeItemAddress);

    // delete ref to treeItemId
    m_treeItemAddressWithDataHash.remove(treeItemAddress);
}
