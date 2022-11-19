#include <QtTest>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>
#include <QTime>
#include <QDate>
#include <QDebug>
#include <QtGui/QTextDocument>


#include "text/markdowntextdocument.h"
#include "skrdata.h"
#include "skrresult.h"
#include "importer.h"

class TreeHubCase : public QObject {
    Q_OBJECT

public:

    TreeHubCase();
    ~TreeHubCase();

public slots:

private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    //    void getAll();
    void getAllIndents();
    void getAllSortOrders();
    void getAllIds();
    void setTitle();
    void getTitle();
    void setIndent();
    void getIndent();
    void setTrashed();
    void restoring();
    void getTrashed();
    void setPrimaryContent();
    void getPrimaryContent();
    void setCreationDate();
    void getCreationDate();
    void setUpdateDate();
    void getUpdateDate();
    void getInternalTitle();

    void getParentId();

    void saveTree();

    void queue();
    void missingProjectError();

    void addTreeItem();
    void removeTreeItem();

    // properties
    void property();
    void property_replace();

    // label
    void getTreeLabel();
    void setTreeLabel();

    // tag
    void setTag();

    void cleanUpHtml();
    void sortAlphabetically();

    // cut /copy /paste
    void simpleCutPaste();
    void multipleCutPaste();
    void simpleCutPasteInTheSameFolder();
    void folderCutPaste();
    void simpleCopyPaste();
    void multipleCopyPaste();
    void simpleCopyPasteInTheSameFolder();

    //duplicate
    void duplicate();
    void duplicateWithChildren();

    // move
    void  getValidSortOrderAfterTree();
    void  moveIntoProjectFolder();
    void  moveIntoFolder();
    void moveBefore();
    void moveBeforeFirstItem();
    void moveAfterLastItem();

    void filterOutChildren();

    // save / restore

    void restore();

private:

    SKRData *m_data;
    QUrl m_testProjectPath;
    int m_currentProjectId;
};

TreeHubCase::TreeHubCase()
{}

TreeHubCase::~TreeHubCase()
{}

void TreeHubCase::initTestCase()
{
    m_data            = new SKRData(this);
    m_testProjectPath = "qrc:/testfiles/skribisto_test_project.skrib";
    Importer::init();
}

void TreeHubCase::cleanupTestCase()
{}

void TreeHubCase::init()
{


    SKRResult result(this);
    int projectId = Importer::importProject(m_testProjectPath, "skrib", QVariantMap(), result);

    QList<int> idList = skrdata->projectHub()->getProjectIdList();

    if (idList.isEmpty()) {
        qDebug() << "no project id";
        QVERIFY(true == false);
        return;
    }

    m_currentProjectId = idList.first();
}

void TreeHubCase::cleanup()
{
    QSignalSpy spy(skrdata->projectHub(), SIGNAL(projectClosed(int)));

    skrdata->projectHub()->closeAllProjects();
    QCOMPARE(spy.count(), 1);

    while (!spy.isEmpty()) {
        QList<QVariant> arguments = spy.takeFirst();

        // qDebug() << "project n°" << QString::number(arguments.at(0).toInt())
        // << " closed";
    }
}

// ------------------------------------------------------------------------------------


// void TreeHubCase::getAll()
// {
//    //    QList<QHash<QString, QVariant> > list =
// skrdata->treeHub()->getAll(1);
//    //    QVERIFY(list.length() > 0);
//    //    QList<QString> keyList = list.at(0).keys();
//    //    QVERIFY(keyList.length() > 0);

//    QVERIFY(true == false);

// }

// ------------------------------------------------------------------------------------


void TreeHubCase::getAllIndents()
{
    QHash<TreeItemAddress, int> hash = skrdata->treeHub()->getAllIndents(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<TreeItemAddress> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getAllSortOrders()
{
    QHash<TreeItemAddress, int> hash = skrdata->treeHub()->getAllSortOrders(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<TreeItemAddress> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getAllIds()
{
    QList<TreeItemAddress> list = skrdata->treeHub()->getAllIds(m_currentProjectId);

    QVERIFY(list.size() > 0);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::setTitle()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(titleChanged(TreeItemAddress,QString)));

    skrdata->treeHub()->setTitle(TreeItemAddress(m_currentProjectId, 1), "new_title");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toString() == "new_title");

    //    QString value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getTitle()
{
    QString title = skrdata->treeHub()->getTitle(TreeItemAddress(m_currentProjectId, 4));

    QCOMPARE(title, QString("Sol"));
}

void TreeHubCase::setIndent()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(indentChanged(TreeItemAddress,int)));

    skrdata->treeHub()->setIndent(TreeItemAddress(m_currentProjectId, 1), 1);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toInt() == 1);

    //    QString value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getIndent()
{
    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 2));

    QCOMPARE(indent, 1);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::setTrashed()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(trashedChanged(TreeItemAddress,bool)));

    SKRResult result = skrdata->treeHub()->setTrashedWithChildren(TreeItemAddress(m_currentProjectId,
                                                                  5),
                                                                  true);

    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 5);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toBool() == true);
    bool value = skrdata->treeHub()->getTrashed(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(value, true);

    QDateTime date = skrdata->treeHub()->getTrashedDate(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(date.isValid(), true);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::restoring()
{
    setTrashed();

    // restoring
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(trashedChanged(TreeItemAddress,bool)));

    SKRResult result = skrdata->treeHub()->untrashOnlyOneTreeItem(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(result.isSuccess(), true);

    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toBool() == false);

    bool value = skrdata->treeHub()->getTrashed(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(value, false);

    QDateTime date = skrdata->treeHub()->getTrashedDate(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(date.isNull(), true);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getTrashed()
{
    bool value = skrdata->treeHub()->getTrashed(TreeItemAddress(m_currentProjectId, 2));

    QCOMPARE(value, false);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::setPrimaryContent()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(primaryContentChanged(TreeItemAddress,QString)));

    skrdata->treeHub()->setPrimaryContent(TreeItemAddress(m_currentProjectId, 1), "new_content");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toString() == "new_content");
    QString value = skrdata->treeHub()->getPrimaryContent(TreeItemAddress(m_currentProjectId, 1));

    QCOMPARE(value, QString("new_content"));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getPrimaryContent()
{
    QString value = skrdata->treeHub()->getPrimaryContent(TreeItemAddress(m_currentProjectId, 16));
    MarkdownTextDocument doc;

    doc.setSkribistoMarkdown(value);
    QCOMPARE(doc.toPlainText(), QString("second content test_project_dict_word badword"));

    // lorem ipsum :
    value = skrdata->treeHub()->getPrimaryContent(TreeItemAddress(m_currentProjectId, 14));
    QVERIFY(value.size() > 5000);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::setCreationDate()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(creationDateChanged(TreeItemAddress,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    skrdata->treeHub()->setCreationDate(TreeItemAddress(m_currentProjectId, 1), date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = skrdata->treeHub()->getCreationDate(TreeItemAddress(m_currentProjectId, 1));

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getCreationDate()
{
    QDateTime value = skrdata->treeHub()->getCreationDate(TreeItemAddress(m_currentProjectId, 2));

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 1, 1)));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::setUpdateDate()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(updateDateChanged(TreeItemAddress,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    skrdata->treeHub()->setUpdateDate(TreeItemAddress(m_currentProjectId, 1), date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(1).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = skrdata->treeHub()->getUpdateDate(TreeItemAddress(m_currentProjectId, 1));

    QVERIFY(value < QDateTime::currentDateTime());
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getUpdateDate()
{
    QDateTime value = skrdata->treeHub()->getUpdateDate(TreeItemAddress(m_currentProjectId, 2));

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 1, 1)));
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getInternalTitle()
{
    QList<TreeItemAddress> folders = skrdata->treeHub()->getIdsWithInternalTitle(m_currentProjectId, "note_folder");

    QCOMPARE(folders.at(0).itemId, 3);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::getParentId()
{
    TreeItemAddress parentId = skrdata->treeHub()->getParentId(TreeItemAddress(m_currentProjectId, 18));

    QCOMPARE(parentId.itemId, 6);

    TreeItemAddress nullParentId = skrdata->treeHub()->getParentId(TreeItemAddress(m_currentProjectId, 0));

    QCOMPARE(nullParentId.isValid(), false);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::saveTree()
{
    auto save = skrdata->treeHub()->saveTree(m_currentProjectId);
    QCOMPARE(save.count(), 25);

    skrdata->treeHub()->restoreTree(m_currentProjectId, save);
    QCOMPARE(skrdata->treeHub()->getAllIds(m_currentProjectId).count(), 25);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::queue()
{
    QString value = skrdata->treeHub()->getTitle(TreeItemAddress(m_currentProjectId, 4));

    QCOMPARE(value, QString("Sol"));
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(titleChanged(TreeItemAddress,QString)));

    skrdata->treeHub()->setTitle(TreeItemAddress(m_currentProjectId, 1), "new_title1");
    QVERIFY(spy.count() == 1);
    value = skrdata->treeHub()->getTitle(TreeItemAddress(m_currentProjectId, 1));
    QCOMPARE(value, QString("new_title1"));
    skrdata->treeHub()->setTitle(TreeItemAddress(m_currentProjectId, 1), "new_title2");
    value = skrdata->treeHub()->getTitle(TreeItemAddress(m_currentProjectId, 1));
    QCOMPARE(value, QString("new_title2"));
    skrdata->treeHub()->setTitle(TreeItemAddress(m_currentProjectId, 1), "new_title3");
    value = skrdata->treeHub()->getTitle(TreeItemAddress(m_currentProjectId, 1));
    QCOMPARE(value, QString("new_title3"));
    skrdata->treeHub()->setTitle(TreeItemAddress(m_currentProjectId, 1), "new_title4");
}

// ------------------------------------------------------------------------------------

void TreeHubCase::missingProjectError()
{
    //    QSignalSpy spy(skrdata->errorHub(), SIGNAL(errorSent()));
    //    skrdata->treeHub()->getTitle(9999, 1);
    //    QCOMPARE(spy.count(), 1);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::addTreeItem()
{
    SKRResult result = skrdata->treeHub()->addTreeItemBelow(TreeItemAddress(m_currentProjectId, 1), "TEXT");
    TreeItemAddress lastId       = skrdata->treeHub()->getLastAddedAddress();

    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId.itemId > 1);
    int sortOrder1 = skrdata->treeHub()->getSortOrder(TreeItemAddress(m_currentProjectId, 1));
    int sortOrder2 = skrdata->treeHub()->getSortOrder(TreeItemAddress(m_currentProjectId, lastId.itemId));

    QCOMPARE(sortOrder1 + 1000, sortOrder2);

    result = skrdata->treeHub()->addTreeItemAbove(TreeItemAddress(m_currentProjectId, 1), "TEXT");
    lastId = skrdata->treeHub()->getLastAddedAddress();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId.itemId > 1);
    sortOrder1 = skrdata->treeHub()->getSortOrder(TreeItemAddress(m_currentProjectId, 1));
    sortOrder2 = skrdata->treeHub()->getSortOrder(TreeItemAddress(m_currentProjectId, lastId.itemId));
    QCOMPARE(sortOrder1 - 1000, sortOrder2);

    result = skrdata->treeHub()->addChildTreeItem(TreeItemAddress(m_currentProjectId, 1), "TEXT");
    lastId = skrdata->treeHub()->getLastAddedAddress();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId.itemId > 1);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::removeTreeItem()
{
    SKRResult result = skrdata->treeHub()->removeTreeItem(TreeItemAddress(m_currentProjectId, 1));

    QVERIFY(result.isSuccess() == true);
}

// ------------------------------------------------------------------------------------

void TreeHubCase::property()
{
    QSignalSpy spy(skrdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    skrdata->treePropertyHub()->setProperty(TreeItemAddress(m_currentProjectId, 1), "test1", "value1");
    QVERIFY(spy.count() == 1);
    QString value =  skrdata->treePropertyHub()->getProperty(TreeItemAddress(m_currentProjectId,
                                                             1),
                                                             "test1");

    QCOMPARE(value, QString("value1"));
    QHash<int, bool> hash = skrdata->treePropertyHub()->getAllIsSystems(
        m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

void TreeHubCase::property_replace()
{
    QSignalSpy spy(skrdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    SKRResult result = skrdata->treePropertyHub()->setProperty(TreeItemAddress(m_currentProjectId, 1), "test1", "value1");

    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst();
    int id                    = arguments.at(1).toInt();

    skrdata->treePropertyHub()->setProperty(TreeItemAddress(m_currentProjectId, 1), "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
    skrdata->treePropertyHub()->setProperty(TreeItemAddress(m_currentProjectId, 1), "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
}

void TreeHubCase::getTreeLabel()
{
    QString value = skrdata->treePropertyHub()->getProperty(TreeItemAddress(m_currentProjectId,
                                                            5),
                                                            "label");

    QCOMPARE(value, "this is a label");
}

void TreeHubCase::setTreeLabel()
{
    skrdata->treePropertyHub()->setProperty(TreeItemAddress(m_currentProjectId, 1), "label", "new");
    QString value = skrdata->treePropertyHub()->getProperty(TreeItemAddress(m_currentProjectId,
                                                            1),
                                                            "label");

    QCOMPARE(value, "new");
}

void TreeHubCase::setTag()
{
    SKRResult result = skrdata->tagHub()->setTagRelationship(TreeItemAddress(m_currentProjectId, 2), 1);

    QCOMPARE(result.isSuccess(), true);
    result = skrdata->tagHub()->setTagRelationship(TreeItemAddress(m_currentProjectId, 2), 1);
    QCOMPARE(result.isSuccess(), true);
}

void TreeHubCase::cleanUpHtml() {
    QString html = "style=\" font-family:'Cantarell'; font-size:16pt; font-weight:400; font-style:normal;\"";

    html.remove(QRegularExpression(" font-family:.*?;"));

    qDebug() << html;
}

void TreeHubCase::sortAlphabetically() {
    SKRResult result = skrdata->treeHub()->sortAlphabetically(TreeItemAddress(m_currentProjectId, 0));

    QCOMPARE(result.isSuccess(), true);

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    QList<int> wantedIds;
    wantedIds << 0 << 3 << 21 << 22 << 24 << 7 << 8 << 9 << 10 << 11 <<1 << 23 << 2 << 4 << 5
              << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12;
    QCOMPARE(ids, wantedIds);
}

// -----------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------

void TreeHubCase::simpleCutPaste() {
    skrdata->treeHub()->cut(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14));
    skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 6));


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }

    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5 << 13 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 14 << 12 << 3 << 21 << 22;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));

    QCOMPARE(indent, 3);
}

void TreeHubCase::simpleCutPasteInTheSameFolder() {
    skrdata->treeHub()->cut(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14));
    skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 5));



    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5 << 13 << 15 << 16 << 14 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));

    QCOMPARE(indent, 3);
}

void TreeHubCase::multipleCutPaste() {
    skrdata->treeHub()->cut(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14)
                            << TreeItemAddress(m_currentProjectId, 15)
                            << TreeItemAddress(m_currentProjectId, 16));
    QCOMPARE(skrdata->treeHub()->isCutCopy(TreeItemAddress(m_currentProjectId, 14)), true);

    skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 6));
    QCOMPARE(skrdata->treeHub()->isCutCopy(TreeItemAddress(m_currentProjectId, 14)), false);



    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5 << 13 << 6 << 17 << 18 << 19 << 20 << 14 << 15 << 16 << 12 << 3 << 21 << 22;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));
    QCOMPARE(indent, 3);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 15));
    QCOMPARE(indent, 3);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 16));
    QCOMPARE(indent, 3);
}

void TreeHubCase::folderCutPaste() {
    skrdata->treeHub()->cut(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 2));
    QCOMPARE(skrdata->treeHub()->isCutCopy(TreeItemAddress(m_currentProjectId, 2)),  true);

    SKRResult result = skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 3));
    QCOMPARE(skrdata->treeHub()->isCutCopy(TreeItemAddress(m_currentProjectId, 2)), false);

    QCOMPARE(result.isSuccess(),                                    true);


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    qDebug() << "found:" << ids;
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 3 << 21 << 22 << 2 << 4 << 5 << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));
    QCOMPARE(indent, 4);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 15));
    QCOMPARE(indent, 4);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 16));
    QCOMPARE(indent, 4);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 12));
    QCOMPARE(indent, 3);
}

void TreeHubCase::simpleCopyPaste() {
    skrdata->treeHub()->copy(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14));
    SKRResult result = skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 6));

    QCOMPARE(result.isSuccess(), true);
    QList<TreeItemAddress> newIdList = result.getData("treeItemAddressList",
                                          QVariant::fromValue(QList<TreeItemAddress>())).value<QList<TreeItemAddress>>();

    QCOMPARE(newIdList.isEmpty(), false);


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5 << 13 << 14 << 15
              << 16 << 6 << 17 << 18 << 19 << 20 << newIdList.first().itemId << 12 << 3 << 21 << 22;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));

    QCOMPARE(indent, 3);
}

void TreeHubCase::simpleCopyPasteInTheSameFolder() {
    skrdata->treeHub()->copy(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14));
    SKRResult result = skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 5));

    QCOMPARE(result.isSuccess(), true);
    QList<TreeItemAddress> newIdList = result.getData("treeItemAddressList",
                                          QVariant::fromValue<QList<TreeItemAddress> >(QList<TreeItemAddress>())).value<QList<TreeItemAddress> >();

    QCOMPARE(newIdList.isEmpty(), false);


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
                 << 13 << 14 << 15 << 16 << newIdList.first().itemId << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22;

    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));

    QCOMPARE(indent, 3);
}

void TreeHubCase::duplicate()
{
    SKRResult result = skrdata->treeHub()->duplicateTreeItem(TreeItemAddress(m_currentProjectId, 5), false);
    QList<TreeItemAddress> newIdList = result.getData("treeItemAddressList",
                                          QVariant::fromValue<QList<TreeItemAddress> >(QList<TreeItemAddress>())).value<QList<TreeItemAddress> >();
    QCOMPARE(newIdList.isEmpty(), false);

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
              << 13 << 14 << 15 << 16 << newIdList.first().itemId << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22;

    QCOMPARE(ids, wantedIds);
}

void TreeHubCase::duplicateWithChildren()
{
    SKRResult result = skrdata->treeHub()->duplicateTreeItem(TreeItemAddress(m_currentProjectId, 5), true);
    QList<TreeItemAddress> newIdList = result.getData("treeItemAddressList",
                                          QVariant::fromValue(QList<TreeItemAddress>())).value<QList<TreeItemAddress>>();


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22

    QList<int> newIdIntList;
    for(const TreeItemAddress &address : newIdList){
        newIdIntList << address.itemId;
    }



    QList<int> wantedIds;
    qDebug() << "newIdList" << newIdIntList;
    qDebug() << "ids" << ids;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
              << 13 << 14 << 15 << 16 << newIdIntList << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22;

    QCOMPARE(ids, wantedIds);

}

void TreeHubCase::multipleCopyPaste() {
    skrdata->treeHub()->copy(QList<TreeItemAddress>() << TreeItemAddress(m_currentProjectId, 14)
                             << TreeItemAddress(m_currentProjectId,15)
                             << TreeItemAddress(m_currentProjectId,16));
    SKRResult result = skrdata->treeHub()->paste(TreeItemAddress(m_currentProjectId, 6));

    QCOMPARE(result.isSuccess(), true);
    QList<TreeItemAddress> newIdList = result.getData("treeItemAddressList",
                                          QVariant::fromValue(QList<TreeItemAddress>())).value<QList<TreeItemAddress>>();

    QCOMPARE(newIdList.count(), 3);


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
              << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << newIdList.first().itemId <<
                 newIdList.at(1).itemId << newIdList.at(2).itemId << 12 << 3 << 21 << 22;
    QCOMPARE(ids, wantedIds);


    int indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 14));
    QCOMPARE(indent, 3);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 15));
    QCOMPARE(indent, 3);
    indent = skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 16));
    QCOMPARE(indent, 3);
}

void TreeHubCase::moveIntoProjectFolder() {


    // move at the end of the tree :
    SKRResult result = skrdata->treeHub()->moveTreeItemAsChildOf(TreeItemAddress(m_currentProjectId, 4), TreeItemAddress(m_currentProjectId, 0));

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }

    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 5
              << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20  << 12 << 3 << 21 << 22 << 4 ;

    QCOMPARE(ids, wantedIds);


}

void TreeHubCase::moveIntoFolder() {


    // move at the end of the tree :
    SKRResult result = skrdata->treeHub()->moveTreeItemAsChildOf(TreeItemAddress(m_currentProjectId, 4), TreeItemAddress(m_currentProjectId, 6));

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 5
              << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 4 << 12 << 3 << 21 << 22 ;
    QCOMPARE(ids, wantedIds);


}


void TreeHubCase::moveBefore() {


    // move before :
    SKRResult result = skrdata->treeHub()->moveTreeItem(TreeItemAddress(m_currentProjectId, 4), TreeItemAddress(m_currentProjectId, 3), false);

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 5
              << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 4 << 3 << 21 << 22 ;
    QCOMPARE(ids, wantedIds);
    QCOMPARE(skrdata->treeHub()->getIndent(TreeItemAddress(m_currentProjectId, 4)), 1);


}

void TreeHubCase::moveBeforeFirstItem() {


    // move before :
    SKRResult result = skrdata->treeHub()->moveTreeItem(TreeItemAddress(m_currentProjectId, 4), TreeItemAddress(m_currentProjectId, 13), false);

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 5
               << 4 << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22 ;
    QCOMPARE(ids, wantedIds);

}
void TreeHubCase::moveAfterLastItem() {


    // move after :
    SKRResult result = skrdata->treeHub()->moveTreeItem(TreeItemAddress(m_currentProjectId, 4), TreeItemAddress(m_currentProjectId, 16), true);

    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22
    QList<int> wantedIds;
    wantedIds << 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 5
              << 13 << 14 << 15 << 16 << 4 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22 ;
    QCOMPARE(ids, wantedIds);

}

void TreeHubCase::getValidSortOrderAfterTree() {

    // after the last item of the project tree :
     int validSortOrder = skrdata->treeHub()->getValidSortOrderAfterTree(TreeItemAddress(m_currentProjectId, 0));
     QCOMPARE(validSortOrder, 24001);

     validSortOrder = skrdata->treeHub()->getValidSortOrderAfterTree(TreeItemAddress(m_currentProjectId, 6));
     QCOMPARE(validSortOrder, 20001);

}


void TreeHubCase::filterOutChildren() {


    // original:
    // 0 << 24 << 7 << 8 << 9 << 10 << 11 << 1 << 23 << 2 << 4 << 5
    // << 13 << 14 << 15 << 16 << 6 << 17 << 18 << 19 << 20 << 12 << 3 << 21 << 22

    // no filtering needed
    QList<TreeItemAddress> selection;
    selection << TreeItemAddress(m_currentProjectId, 17) << TreeItemAddress(m_currentProjectId, 18);

    QList<TreeItemAddress> filteredOutSelection = skrdata->treeHub()->filterOutChildren(selection);
    QList<int> filteredOutSelectionIds;
    for(const TreeItemAddress &address : filteredOutSelection){
        filteredOutSelectionIds << address.itemId;
    }

    QList<int> wantedSelection;
    wantedSelection <<  17 << 18;
    QCOMPARE(filteredOutSelectionIds, wantedSelection);

    // child / parent filtering
    selection.clear();
    wantedSelection.clear();
    filteredOutSelectionIds.clear();
    selection << TreeItemAddress(m_currentProjectId, 6) << TreeItemAddress(m_currentProjectId, 17);

    filteredOutSelection = skrdata->treeHub()->filterOutChildren(selection);

    for(const TreeItemAddress &address : filteredOutSelection){
        filteredOutSelectionIds << address.itemId;
    }

    wantedSelection <<  6;
    QCOMPARE(filteredOutSelectionIds, wantedSelection);

    // multiple parent filtering
    selection.clear();
    wantedSelection.clear();
    filteredOutSelectionIds.clear();
    selection << TreeItemAddress(m_currentProjectId, 2)
              << TreeItemAddress(m_currentProjectId, 4)
              << TreeItemAddress(m_currentProjectId, 5)
              << TreeItemAddress(m_currentProjectId, 13)
              << TreeItemAddress(m_currentProjectId, 14)
              << TreeItemAddress(m_currentProjectId, 15)
              << TreeItemAddress(m_currentProjectId, 16)
              << TreeItemAddress(m_currentProjectId, 6)
              << TreeItemAddress(m_currentProjectId, 17)
              << TreeItemAddress(m_currentProjectId, 18)
              << TreeItemAddress(m_currentProjectId, 19)
              << TreeItemAddress(m_currentProjectId, 20)
              << TreeItemAddress(m_currentProjectId, 12)
              << TreeItemAddress(m_currentProjectId, 3)
              << TreeItemAddress(m_currentProjectId, 21)
              << TreeItemAddress(m_currentProjectId, 22);


    filteredOutSelection = skrdata->treeHub()->filterOutChildren(selection);

    for(const TreeItemAddress &address : filteredOutSelection){
        filteredOutSelectionIds << address.itemId;
    }
    wantedSelection << 2 << 3;
    QCOMPARE(filteredOutSelectionIds, wantedSelection);
}

//-------------------------------------------------

void TreeHubCase::restore()
{
    skrdata->treeHub()->restoreTree(m_currentProjectId, skrdata->treeHub()->saveTree(m_currentProjectId));


    QList<int> ids;
    for(const TreeItemAddress &address : skrdata->treeHub()->getAllIds(m_currentProjectId)){
        ids << address.itemId;
    }
    qDebug() << "ids" << ids;

    QCOMPARE(ids.first(), 0);

}


QTEST_MAIN(TreeHubCase)

#include "tst_treehubcase.moc"
