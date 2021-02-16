#include <QtTest>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>
#include <QTime>
#include <QDate>
#include <QDebug>


#include "plmdata.h"
#include "skrresult.h"

class WriteCase : public QObject {
    Q_OBJECT

public:

    WriteCase();
    ~WriteCase();

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


private:

    PLMData *m_data;
    QUrl m_testProjectPath;
    int m_currentProjectId;
};

WriteCase::WriteCase()
{}

WriteCase::~WriteCase()
{}

void WriteCase::initTestCase()
{
    m_data            = new PLMData(this);
    m_testProjectPath = "qrc:/testfiles/skribisto_test_project.skrib";
}

void WriteCase::cleanupTestCase()
{}

void WriteCase::init()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectLoaded(int)));

    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<int> idList = plmdata->projectHub()->getProjectIdList();

    if (idList.isEmpty()) {
        qDebug() << "no project id";
        QVERIFY(true == false);
        return;
    }

    m_currentProjectId = idList.first();
}

void WriteCase::cleanup()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectClosed(int)));

    plmdata->projectHub()->closeAllProjects();
    QCOMPARE(spy.count(), 1);

    while (!spy.isEmpty()) {
        QList<QVariant> arguments = spy.takeFirst();

        // qDebug() << "project n°" << QString::number(arguments.at(0).toInt())
        // << " closed";
    }
}

// ------------------------------------------------------------------------------------


// void WriteCase::getAll()
// {
//    //    QList<QHash<QString, QVariant> > list =
// plmdata->treeHub()->getAll(1);
//    //    QVERIFY(list.length() > 0);
//    //    QList<QString> keyList = list.at(0).keys();
//    //    QVERIFY(keyList.length() > 0);

//    QVERIFY(true == false);

// }

// ------------------------------------------------------------------------------------


void WriteCase::getAllIndents()
{
    QHash<int, int> hash = plmdata->treeHub()->getAllIndents(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::getAllSortOrders()
{
    QHash<int, int> hash = plmdata->treeHub()->getAllSortOrders(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::getAllIds()
{
    QList<int> list = plmdata->treeHub()->getAllIds(m_currentProjectId);

    QVERIFY(list.size() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::setTitle()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(titleChanged(int,int,QString)));

    plmdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_title");

    //    QString value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getTitle()
{
    QString title = plmdata->treeHub()->getTitle(m_currentProjectId, 1);

    QCOMPARE(title, QString("First title"));
}

void WriteCase::setIndent()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(indentChanged(int,int,int)));

    plmdata->treeHub()->setIndent(m_currentProjectId, 1, 1);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toInt() == 1);

    //    QString value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getIndent()
{
    int indent = plmdata->treeHub()->getIndent(m_currentProjectId, 1);

    QCOMPARE(indent, 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::setTrashed()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(trashedChanged(int,int,bool)));

    SKRResult result = plmdata->treeHub()->setTrashedWithChildren(m_currentProjectId,
                                                                 55,
                                                                 true);

    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 2);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toBool() == true);
    bool value = plmdata->treeHub()->getTrashed(m_currentProjectId, 55);

    QCOMPARE(value, true);

    QDateTime date = plmdata->treeHub()->getTrashedDate(m_currentProjectId, 55);

    QCOMPARE(date.isValid(), true);
}

// ------------------------------------------------------------------------------------

void WriteCase::restoring()
{
    setTrashed();

    // restoring
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(trashedChanged(int,int,bool)));

    SKRResult result = plmdata->treeHub()->untrashOnlyOneTreeItem(m_currentProjectId, 2);

    QCOMPARE(result.isSuccess(), true);

    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toBool() == false);

    bool value = plmdata->treeHub()->getTrashed(m_currentProjectId, 2);

    QCOMPARE(value, false);

    QDateTime date = plmdata->treeHub()->getTrashedDate(m_currentProjectId, 2);

    QCOMPARE(date.isNull(), true);
}

// ------------------------------------------------------------------------------------

void WriteCase::getTrashed()
{
    bool value = plmdata->treeHub()->getTrashed(m_currentProjectId, 1);

    QCOMPARE(value, false);
}

// ------------------------------------------------------------------------------------

void WriteCase::setPrimaryContent()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(primaryContentChanged(int,int,QString)));

    plmdata->treeHub()->setPrimaryContent(m_currentProjectId, 1, "new_content");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_content");
    QString value = plmdata->treeHub()->getPrimaryContent(m_currentProjectId, 1);

    QCOMPARE(value, QString("new_content"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getPrimaryContent()
{
    QString value = plmdata->treeHub()->getPrimaryContent(m_currentProjectId, 1);

    QCOMPARE(value, QString("fir**st** *content* test_project_dict_word badword\n\n"));

    // lorem ipsum :
    value = plmdata->treeHub()->getPrimaryContent(m_currentProjectId, 8);
    QVERIFY(value.size() > 5000);
}

// ------------------------------------------------------------------------------------

void WriteCase::setCreationDate()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(creationDateChanged(int,int,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    plmdata->treeHub()->setCreationDate(m_currentProjectId, 1, date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = plmdata->treeHub()->getCreationDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
}

// ------------------------------------------------------------------------------------

void WriteCase::getCreationDate()
{
    QDateTime value = plmdata->treeHub()->getCreationDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2000, 1, 1), QTime(0, 0, 0)));
}

// ------------------------------------------------------------------------------------

void WriteCase::setUpdateDate()
{
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(updateDateChanged(int,int,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    plmdata->treeHub()->setUpdateDate(m_currentProjectId, 1, date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = plmdata->treeHub()->getUpdateDate(m_currentProjectId, 1);

    QVERIFY(value < QDateTime::currentDateTime());
}

// ------------------------------------------------------------------------------------

void WriteCase::getUpdateDate()
{
    QDateTime value = plmdata->treeHub()->getUpdateDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 1, 1)));
}


// ------------------------------------------------------------------------------------

void WriteCase::queue()
{
    QString value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);

    QCOMPARE(value, QString("First title"));
    QSignalSpy spy(plmdata->treeHub(), SIGNAL(titleChanged(int,int,QString)));

    plmdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title1");
    QVERIFY(spy.count() == 1);
    value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title1"));
    plmdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title2");
    value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title2"));
    plmdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title3");
    value = plmdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title3"));
    plmdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title4");
}

// ------------------------------------------------------------------------------------

void WriteCase::missingProjectError()
{
    //    QSignalSpy spy(plmdata->errorHub(), SIGNAL(errorSent()));
    //    plmdata->treeHub()->getTitle(9999, 1);
    //    QCOMPARE(spy.count(), 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::addTreeItem()
{
    SKRResult result = plmdata->treeHub()->addTreeItemBelow(m_currentProjectId, 1, "TEXT");
    int lastId     = plmdata->treeHub()->getLastAddedId();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);
    int sortOrder1 = plmdata->treeHub()->getSortOrder(m_currentProjectId, 1);
    int sortOrder2 = plmdata->treeHub()->getSortOrder(m_currentProjectId, lastId);
    QVERIFY(sortOrder1 + 1000 == sortOrder2);

    result  = plmdata->treeHub()->addTreeItemAbove(m_currentProjectId, 1, "TEXT");
    lastId = plmdata->treeHub()->getLastAddedId();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);    
    sortOrder1 = plmdata->treeHub()->getSortOrder(m_currentProjectId, 1);
    sortOrder2 = plmdata->treeHub()->getSortOrder(m_currentProjectId, lastId);
    QVERIFY(sortOrder1 - 1000 == sortOrder2);

    result  = plmdata->treeHub()->addChildTreeItem(m_currentProjectId, 1, "TEXT");
    lastId = plmdata->treeHub()->getLastAddedId();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::removeTreeItem()
{
    SKRResult result = plmdata->treeHub()->removeTreeItem(m_currentProjectId, 1);

    QVERIFY(result.isSuccess() == true);
}

// ------------------------------------------------------------------------------------

void WriteCase::property()
{
    QSignalSpy spy(plmdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    plmdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    QVERIFY(spy.count() == 1);
    QString value =  plmdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                              1,
                                                              "test0");

    QCOMPARE(value, QString("value0"));
    QHash<int, bool> hash = plmdata->treePropertyHub()->getAllIsSystems(
        m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

void WriteCase::property_replace()
{
    QSignalSpy spy(plmdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    SKRResult result = plmdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst();
    int id                    = arguments.at(1).toInt();

    plmdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
    plmdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
}

void WriteCase::getTreeLabel()
{
    QString value = plmdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                             1,
                                                             "label");

    QCOMPARE(value, "this is a label");
}

void WriteCase::setTreeLabel()
{
    plmdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "label", "new");
    QString value = plmdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                             1,
                                                             "label");

    QCOMPARE(value, "new");
}

void WriteCase::setTag()
{
    SKRResult result = plmdata->tagHub()->setTagRelationship(m_currentProjectId, 2, 1);

    QCOMPARE(result.isSuccess(), true);
    result = plmdata->tagHub()->setTagRelationship(m_currentProjectId, 2, 1);
    QCOMPARE(result.isSuccess(), true);

}

QTEST_GUILESS_MAIN(WriteCase)

#include "tst_writecase.moc"
