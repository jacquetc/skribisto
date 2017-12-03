#include <QtTest>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>
#include <QTime>
#include <QDate>
#include <QDebug>


#include "plmdata.h"
#include "plmerror.h"

class WriteCase : public QObject
{
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
    void getAllTitles();
    void getAllIndents();
    void getAllSortOrders();
    void getAllIds();
    void setTitle();
    void getTitle();
    void setIndent();
    void getIndent();
    void setDeleted();
    void getDeleted();
    void setContent();
    void getContent();
    void setCreationDate();
    void getCreationDate();
    void setUpdateDate();
    void getUpdateDate();
    void setContentDate();
    void getContentDate();

    void queue();
    void missingProjectError();

    void addPaper();
    void removePaper();

    //properties
    void property();
    void property_replace();

private:
    PLMData *m_data;
    QString m_testProjectPath;
    int m_currentProjectId;

};

WriteCase::WriteCase()
{

}

WriteCase::~WriteCase()
{

}

void WriteCase::initTestCase()
{
    m_data = new PLMData(this);
    m_testProjectPath = ":/plume_test_project.sqlite";

}


void WriteCase::cleanupTestCase()
{

}

void WriteCase::init()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectLoaded(int)));
    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<int> idList = plmdata->projectHub()->getProjectIdList();
    if(idList.isEmpty()){
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
    while(!spy.isEmpty()){
        QList<QVariant> arguments = spy.takeFirst();
        //qDebug() << "project n°" << QString::number(arguments.at(0).toInt()) << " closed";
    }

}
//------------------------------------------------------------------------------------


//void WriteCase::getAll()
//{
//    //    QList<QHash<QString, QVariant> > list = plmdata->sheetHub()->getAll(1);
//    //    QVERIFY(list.length() > 0);
//    //    QList<QString> keyList = list.at(0).keys();
//    //    QVERIFY(keyList.length() > 0);

//    QVERIFY(true == false);

//}

//------------------------------------------------------------------------------------

void WriteCase::getAllTitles()
{


    QHash<int, QString> hash = plmdata->sheetHub()->getAllTitles(m_currentProjectId);
    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();
    QVERIFY(keyList.length() > 0);
}

//------------------------------------------------------------------------------------


void WriteCase::getAllIndents()
{


    QHash<int, int> hash = plmdata->sheetHub()->getAllIndents(m_currentProjectId);
    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();
    QVERIFY(keyList.length() > 0);
}

//------------------------------------------------------------------------------------

void WriteCase::getAllSortOrders()
{


    QHash<int, int> hash = plmdata->sheetHub()->getAllSortOrders(m_currentProjectId);
    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();
    QVERIFY(keyList.length() > 0);
}

//------------------------------------------------------------------------------------

void WriteCase::getAllIds()
{
    QList<int> list = plmdata->sheetHub()->getAllIds(m_currentProjectId);
    QVERIFY(list.size() > 0);
}

//------------------------------------------------------------------------------------

void WriteCase::setTitle()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(titleChanged(int,int,QString)));

    plmdata->sheetHub()->setTitle(m_currentProjectId, 1, "new_title");

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_title");

    //    QString value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}


//------------------------------------------------------------------------------------

void WriteCase::getTitle()
{
    QString title = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(title, QString("first_title"));

}

void WriteCase::setIndent()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(indentChanged(int,int,int)));

    plmdata->sheetHub()->setIndent(m_currentProjectId, 1, 1);

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toInt() == 1);

    //    QString value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    //    QCOMPARE(value, QString("new_title"));
}


//------------------------------------------------------------------------------------

void WriteCase::getIndent()
{
    int indent = plmdata->sheetHub()->getIndent(m_currentProjectId, 1);
    QCOMPARE(indent, 0);

}
//------------------------------------------------------------------------------------

void WriteCase::setDeleted()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(deletedChanged(int,int,bool)));

    plmdata->sheetHub()->setDeleted(m_currentProjectId, 1, true);

    QVERIFY(spy.count() > 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toBool() == true);

    bool value = plmdata->sheetHub()->getDeleted(m_currentProjectId, 1);
    QCOMPARE(value, true);

}


//------------------------------------------------------------------------------------

void WriteCase::getDeleted()
{

    bool value = plmdata->sheetHub()->getDeleted(m_currentProjectId, 1);
    QCOMPARE(value, false);


}
//------------------------------------------------------------------------------------

void WriteCase::setContent()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(contentChanged(int,int,QString)));

    plmdata->sheetHub()->setContent(m_currentProjectId, 1, "new_content");

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toString() == "new_content");

    QString value = plmdata->sheetHub()->getContent(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_content"));
}

//------------------------------------------------------------------------------------

void WriteCase::getContent()
{

    QString value = plmdata->sheetHub()->getContent(m_currentProjectId, 1);
    QCOMPARE(value, QString("first content"));

    // lorem ipsum :
    value = plmdata->sheetHub()->getContent(m_currentProjectId, 6);
    QVERIFY(value.size() > 5000);
}

//------------------------------------------------------------------------------------

void WriteCase::setCreationDate()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(creationDateChanged(int,int,QDateTime)));

    QDateTime date(QDate(2010,1,1), QTime(1,0,0));
    plmdata->sheetHub()->setCreationDate(m_currentProjectId, 1, date);

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010,1,1), QTime(1,0,0)));

    QDateTime value = plmdata->sheetHub()->getCreationDate(m_currentProjectId, 1);
    QCOMPARE(value, QDateTime(QDate(2010,1,1), QTime(1,0,0)));

}

//------------------------------------------------------------------------------------

void WriteCase::getCreationDate()
{

    QDateTime value = plmdata->sheetHub()->getCreationDate(m_currentProjectId, 1);
    QCOMPARE(value, QDateTime(QDate(2000,1,1), QTime(0,0,0)));

}

//------------------------------------------------------------------------------------

void WriteCase::setUpdateDate()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(updateDateChanged(int,int,QDateTime)));

    QDateTime date(QDate(2010,1,1), QTime(1,0,0));
    plmdata->sheetHub()->setUpdateDate(m_currentProjectId, 1, date);

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010,1,1), QTime(1,0,0)));

    QDateTime value = plmdata->sheetHub()->getUpdateDate(m_currentProjectId, 1);
    QVERIFY(value < QDateTime::currentDateTime());

}

//------------------------------------------------------------------------------------

void WriteCase::getUpdateDate()
{

    QDateTime value = plmdata->sheetHub()->getUpdateDate(m_currentProjectId, 1);
    QCOMPARE(value, QDateTime(QDate(2010,1,1), QTime(1,1,1)));

}
//------------------------------------------------------------------------------------

void WriteCase::setContentDate()
{

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(contentDateChanged(int,int,QDateTime)));

    QDateTime date(QDate(2010,1,1), QTime(1,0,0));
    plmdata->sheetHub()->setContentDate(m_currentProjectId, 1, date);

    QCOMPARE(spy.count(), 1);
    // make sure the signal was emitted exactly one time

    QList<QVariant> arguments = spy.takeFirst(); // take the first signal

    QVERIFY(arguments.at(2).toDateTime() == QDateTime(QDate(2010,1,1), QTime(1,0,0)));

    QDateTime value = plmdata->sheetHub()->getContentDate(m_currentProjectId, 1);
    QCOMPARE(value, QDateTime(QDate(2010,1,1), QTime(1,0,0)));

}

//------------------------------------------------------------------------------------

void WriteCase::getContentDate()
{

    QDateTime value = plmdata->sheetHub()->getContentDate(m_currentProjectId, 1);
    QCOMPARE(value, QDateTime(QDate(2000,1,1), QTime(0,0,0)));

}
//------------------------------------------------------------------------------------

void WriteCase::queue()
{
    QString value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("first_title"));

    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(titleChanged(int,int,QString)));
    plmdata->sheetHub()->setTitle(m_currentProjectId, 1, "new_title1");
    QVERIFY(spy.count() == 1);

    value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title1"));

    plmdata->sheetHub()->setTitle(m_currentProjectId, 1, "new_title2");

    value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title2"));

    plmdata->sheetHub()->setTitle(m_currentProjectId, 1, "new_title3");

    value = plmdata->sheetHub()->getTitle(m_currentProjectId, 1);
    QCOMPARE(value, QString("new_title3"));

    plmdata->sheetHub()->setTitle(m_currentProjectId, 1, "new_title4");
}


//------------------------------------------------------------------------------------

void WriteCase::missingProjectError()
{
    //    QSignalSpy spy(plmdata->errorHub(), SIGNAL(errorSent()));
    //    plmdata->sheetHub()->getTitle(9999, 1);
    //    QCOMPARE(spy.count(), 1);

}

//------------------------------------------------------------------------------------

void WriteCase::addPaper()
{
    PLMError error = plmdata->sheetHub()->addPaperBelow(m_currentProjectId, 1);
    int lastId = plmdata->sheetHub()->getLastAddedId();
    QVERIFY(error.isSuccess() == true);
    QVERIFY(lastId > 1);

    int sortOrder1 = plmdata->sheetHub()->getSortOrder(m_currentProjectId, 1);
    int sortOrder2 = plmdata->sheetHub()->getSortOrder(m_currentProjectId, lastId);
    QVERIFY(sortOrder1 + 2000 == sortOrder2);

    error = plmdata->sheetHub()->addPaperBelow(m_currentProjectId, 100);
    lastId = plmdata->sheetHub()->getLastAddedId();
    QVERIFY(error.isSuccess() == true);
    QVERIFY(lastId > 1);

    sortOrder1 = plmdata->sheetHub()->getSortOrder(m_currentProjectId, 100);
    sortOrder2 = plmdata->sheetHub()->getSortOrder(m_currentProjectId, lastId);
    QVERIFY(sortOrder1 + 1000 == sortOrder2);

    error = plmdata->sheetHub()->addChildPaper(m_currentProjectId, 1);
    lastId = plmdata->sheetHub()->getLastAddedId();
    QVERIFY(error.isSuccess() == true);
    QVERIFY(lastId > 1);

}
//------------------------------------------------------------------------------------

void WriteCase::removePaper()
{
    PLMError error = plmdata->sheetHub()->removePaper(m_currentProjectId, 1);
    QVERIFY(error.isSuccess() == true);


}
//------------------------------------------------------------------------------------

void WriteCase::property()
{
    QSignalSpy spy(plmdata->sheetPropertyHub(), SIGNAL(propertyChanged(int,int,int,QString,QString)));
    plmdata->sheetPropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    QVERIFY(spy.count() == 1);


    QString value =  plmdata->sheetPropertyHub()->getProperty(m_currentProjectId, 1, "test0");
    QCOMPARE(value, QString("value0"));

    value =  plmdata->sheetPropertyHub()->getProperty(m_currentProjectId, 1, "test1");
    QCOMPARE(value, QString("value1"));

    QHash<int, bool> hash = plmdata->sheetPropertyHub()->getAllIsSystems(m_currentProjectId);
    QVERIFY(hash.size() > 0);
    QList<int> keyList = hash.keys();
    QVERIFY(keyList.length() > 0);


}

void WriteCase::property_replace()
{
    QSignalSpy spy(plmdata->sheetPropertyHub(), SIGNAL(propertyChanged(int,int,int,QString,QString)));
    plmdata->sheetPropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    QVERIFY(spy.count() == 1);

    QList<QVariant> arguments = spy.takeFirst();
    int id = arguments.at(1).toInt();
    plmdata->sheetPropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
    plmdata->sheetPropertyHub()->setProperty(m_currentProjectId, 1, "test1", "value1");
    arguments = spy.takeFirst();
    QCOMPARE(arguments.at(1).toInt(), id);
}

QTEST_GUILESS_MAIN(WriteCase)

#include "tst_writecase.moc"
