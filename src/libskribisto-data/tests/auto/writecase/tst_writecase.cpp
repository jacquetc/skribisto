#include <QtTest>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>
#include <QTime>
#include <QDate>
#include <QDebug>

#include <QtGui/QTextDocument>


#include "skrdata.h"
#include "skrresult.h"
#include "models/skrtreelistmodel.h"

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

    void cleanUpHtml();
    void sortAlphabetically();

private:

    SKRData *m_data;
    QUrl m_testProjectPath;
    int m_currentProjectId;
};

WriteCase::WriteCase()
{}

WriteCase::~WriteCase()
{}

void WriteCase::initTestCase()
{
    m_data            = new SKRData(this);
    m_testProjectPath = "qrc:/testfiles/skribisto_test_project.skrib";
}

void WriteCase::cleanupTestCase()
{}

void WriteCase::init()
{
    QSignalSpy spy(skrdata->projectHub(), SIGNAL(projectLoaded(int)));

    skrdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<int> idList = skrdata->projectHub()->getProjectIdList();

    if (idList.isEmpty()) {
        qDebug() << "no project id";
        QVERIFY(true == false);
        return;
    }

    m_currentProjectId = idList.first();
}

void WriteCase::cleanup()
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


// void WriteCase::getAll()
// {
//    //    QList<QHash<QString, QVariant> > list =
// skrdata->treeHub()->getAll(1);
//    //    QVERIFY(list.length() > 0);
//    //    QList<QString> keyList = list.at(0).keys();
//    //    QVERIFY(keyList.length() > 0);

//    QVERIFY(true == false);

// }

// ------------------------------------------------------------------------------------


void WriteCase::getAllIndents()
{
    QHash<int, int> hash = skrdata->treeHub()->getAllIndents(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::getAllSortOrders()
{
    QHash<int, int> hash = skrdata->treeHub()->getAllSortOrders(m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::getAllIds()
{
    QList<int> list = skrdata->treeHub()->getAllIds(m_currentProjectId);

    QVERIFY(list.size() > 0);
}

// ------------------------------------------------------------------------------------

void WriteCase::setTitle()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(titleChanged(int,int,QString)));

    skrdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_title");

    //    QString value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getTitle()
{
    QString title = skrdata->treeHub()->getTitle(m_currentProjectId, 1);

    QCOMPARE(title, QString("First title"));
}

void WriteCase::setIndent()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(indentChanged(int,int,int)));

    skrdata->treeHub()->setIndent(m_currentProjectId, 1, 1);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toInt() == 1);

    //    QString value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getIndent()
{
    int indent = skrdata->treeHub()->getIndent(m_currentProjectId, 1);

    QCOMPARE(indent, 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::setTrashed()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(trashedChanged(int,int,bool)));

    SKRResult result = skrdata->treeHub()->setTrashedWithChildren(m_currentProjectId,
                                                                  55,
                                                                  true);

    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 3);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toBool() == true);
    bool value = skrdata->treeHub()->getTrashed(m_currentProjectId, 55);

    QCOMPARE(value, true);

    QDateTime date = skrdata->treeHub()->getTrashedDate(m_currentProjectId, 55);

    QCOMPARE(date.isValid(), true);
}

// ------------------------------------------------------------------------------------

void WriteCase::restoring()
{
    setTrashed();

    // restoring
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(trashedChanged(int,int,bool)));

    SKRResult result = skrdata->treeHub()->untrashOnlyOneTreeItem(m_currentProjectId, 2);

    QCOMPARE(result.isSuccess(), true);

    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toBool() == false);

    bool value = skrdata->treeHub()->getTrashed(m_currentProjectId, 2);

    QCOMPARE(value, false);

    QDateTime date = skrdata->treeHub()->getTrashedDate(m_currentProjectId, 2);

    QCOMPARE(date.isNull(), true);
}

// ------------------------------------------------------------------------------------

void WriteCase::getTrashed()
{
    bool value = skrdata->treeHub()->getTrashed(m_currentProjectId, 1);

    QCOMPARE(value, false);
}

// ------------------------------------------------------------------------------------

void WriteCase::setPrimaryContent()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(primaryContentChanged(int,int,QString)));

    skrdata->treeHub()->setPrimaryContent(m_currentProjectId, 1, "new_content");
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_content");
    QString value = skrdata->treeHub()->getPrimaryContent(m_currentProjectId, 1);

    QCOMPARE(value, QString("new_content"));
}

// ------------------------------------------------------------------------------------

void WriteCase::getPrimaryContent()
{
    QString value = skrdata->treeHub()->getPrimaryContent(m_currentProjectId, 1);
    QTextDocument doc;

    doc.setHtml(value);
    QCOMPARE(doc.toPlainText(), QString("first content test_project_dict_word badword"));

    // lorem ipsum :
    value = skrdata->treeHub()->getPrimaryContent(m_currentProjectId, 8);
    QVERIFY(value.size() > 5000);
}

// ------------------------------------------------------------------------------------

void WriteCase::setCreationDate()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(creationDateChanged(int,int,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    skrdata->treeHub()->setCreationDate(m_currentProjectId, 1, date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = skrdata->treeHub()->getCreationDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
}

// ------------------------------------------------------------------------------------

void WriteCase::getCreationDate()
{
    QDateTime value = skrdata->treeHub()->getCreationDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2000, 1, 1), QTime(0, 0, 0)));
}

// ------------------------------------------------------------------------------------

void WriteCase::setUpdateDate()
{
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(updateDateChanged(int,int,QDateTime)));
    QDateTime  date(QDate(2010, 1, 1), QTime(1, 0, 0));

    skrdata->treeHub()->setUpdateDate(m_currentProjectId, 1, date);
    QCOMPARE(spy.count(), 1);

    // make sure the signal was emitted exactly one time
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010, 1, 1), QTime(1, 0, 0)));
    QDateTime value = skrdata->treeHub()->getUpdateDate(m_currentProjectId, 1);

    QVERIFY(value < QDateTime::currentDateTime());
}

// ------------------------------------------------------------------------------------

void WriteCase::getUpdateDate()
{
    QDateTime value = skrdata->treeHub()->getUpdateDate(m_currentProjectId, 1);

    QCOMPARE(value, QDateTime(QDate(2010, 1, 1), QTime(1, 1, 1)));
}

// ------------------------------------------------------------------------------------

void WriteCase::queue()
{
    QString value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);

    QCOMPARE(value, QString("First title"));
    QSignalSpy spy(skrdata->treeHub(), SIGNAL(titleChanged(int,int,QString)));

    skrdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title1");
    QVERIFY(spy.count() == 1);
    value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title1"));
    skrdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title2");
    value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title2"));
    skrdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title3");
    value = skrdata->treeHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title3"));
    skrdata->treeHub()->setTitle(m_currentProjectId, 1, "new_title4");
}

// ------------------------------------------------------------------------------------

void WriteCase::missingProjectError()
{
    //    QSignalSpy spy(skrdata->errorHub(), SIGNAL(errorSent()));
    //    skrdata->treeHub()->getTitle(9999, 1);
    //    QCOMPARE(spy.count(), 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::addTreeItem()
{
    SKRResult result = skrdata->treeHub()->addTreeItemBelow(m_currentProjectId, 1, "TEXT");
    int lastId       = skrdata->treeHub()->getLastAddedId();

    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);
    int sortOrder1 = skrdata->treeHub()->getSortOrder(m_currentProjectId, 1);
    int sortOrder2 = skrdata->treeHub()->getSortOrder(m_currentProjectId, lastId);

    QVERIFY(sortOrder1 + 1000 == sortOrder2);

    result = skrdata->treeHub()->addTreeItemAbove(m_currentProjectId, 1, "TEXT");
    lastId = skrdata->treeHub()->getLastAddedId();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);
    sortOrder1 = skrdata->treeHub()->getSortOrder(m_currentProjectId, 1);
    sortOrder2 = skrdata->treeHub()->getSortOrder(m_currentProjectId, lastId);
    QVERIFY(sortOrder1 - 1000 == sortOrder2);

    result = skrdata->treeHub()->addChildTreeItem(m_currentProjectId, 1, "TEXT");
    lastId = skrdata->treeHub()->getLastAddedId();
    QVERIFY(result.isSuccess() == true);
    QVERIFY(lastId > 1);
}

// ------------------------------------------------------------------------------------

void WriteCase::removeTreeItem()
{
    SKRResult result = skrdata->treeHub()->removeTreeItem(m_currentProjectId, 1);

    QVERIFY(result.isSuccess() == true);
}

// ------------------------------------------------------------------------------------

void WriteCase::property()
{
    QSignalSpy spy(skrdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    skrdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    QVERIFY(spy.count() == 1);
    QString value =  skrdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                             1,
                                                             "test0");

    QCOMPARE(value, QString("value0"));
    QHash<int, bool> hash = skrdata->treePropertyHub()->getAllIsSystems(
        m_currentProjectId);

    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();

    QVERIFY(keyList.length() > 0);
}

void WriteCase::property_replace()
{
    QSignalSpy spy(skrdata->treePropertyHub(),
                   SIGNAL(propertyChanged(int,int,int,QString,QString)));

    SKRResult result = skrdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");

    QCOMPARE(result.isSuccess(), true);
    QVERIFY(spy.count() == 1);
    QList<QVariant> arguments = spy.takeFirst();
    int id                    = arguments.at(1).toInt();

    skrdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
    skrdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
}

void WriteCase::getTreeLabel()
{
    QString value = skrdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                            1,
                                                            "label");

    QCOMPARE(value, "this is a label");
}

void WriteCase::setTreeLabel()
{
    skrdata->treePropertyHub()->setProperty(m_currentProjectId, 1, "label", "new");
    QString value = skrdata->treePropertyHub()->getProperty(m_currentProjectId,
                                                            1,
                                                            "label");

    QCOMPARE(value, "new");
}

void WriteCase::setTag()
{
    SKRResult result = skrdata->tagHub()->setTagRelationship(m_currentProjectId, 2, 1);

    QCOMPARE(result.isSuccess(), true);
    result = skrdata->tagHub()->setTagRelationship(m_currentProjectId, 2, 1);
    QCOMPARE(result.isSuccess(), true);
}

void WriteCase::cleanUpHtml() {
    QString html = "style=\" font-family:'Cantarell'; font-size:16pt; font-weight:400; font-style:normal;\"";

    html.remove(QRegularExpression(" font-family:.*?;"));

    qDebug() << html;
}

void WriteCase::sortAlphabetically() {
    SKRResult result = skrdata->treeHub()->sortAlphabetically(m_currentProjectId, 0);

    QCOMPARE(result.isSuccess(), true);

    QList<int> ids = skrdata->treeHub()->getAllIds(m_currentProjectId);
    QList<int> wantedIds;
    wantedIds << 0 << 1 << 2 << 55 << 4 << 57 << 54 << 5 << 6 << 7 << 50 << 51 << 56 << 52 << 3 << 8 << 24;
    QCOMPARE(ids, wantedIds);
}

QTEST_GUILESS_MAIN(WriteCase)

#include "tst_writecase.moc"
