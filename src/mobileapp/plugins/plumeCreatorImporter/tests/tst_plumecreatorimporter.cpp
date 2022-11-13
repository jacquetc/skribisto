
#include <QtTest>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>
#include <QTime>
#include <QDate>
#include <QDebug>


#include "skrdata.h"
#include "skrresult.h"
#include "skrplumecreatorimporter.h"

class PlumeCreatorTest : public QObject {
    Q_OBJECT

public:

    PlumeCreatorTest();
    ~PlumeCreatorTest();

public slots:

private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();
    void importAll();

private:

    SKRData *m_data;
    QUrl m_testPlumePath;
    QUrl m_testSkribistoTempFilePath;
    QTemporaryFile m_testPlumeTempFile;
    QTemporaryFile m_testSkribistoTempFile;
    int m_currentProjectId;
};

PlumeCreatorTest::PlumeCreatorTest()
{}

PlumeCreatorTest::~PlumeCreatorTest()
{}

void PlumeCreatorTest::initTestCase()
{
    m_data = new SKRData(this);

    m_testPlumeTempFile.setAutoRemove(false);
    m_testPlumeTempFile.open();
    QString testPlumeTempFileName = m_testPlumeTempFile.fileName();
    m_testPlumeTempFile.remove();

    bool result = QFile::copy(":/testfiles/plume-test.plume", testPlumeTempFileName);
    QVERIFY(result);

    m_testPlumePath = testPlumeTempFileName;

    m_testSkribistoTempFile.setAutoRemove(false);
    m_testSkribistoTempFile.open();
    m_testSkribistoTempFilePath = m_testSkribistoTempFile.fileName();
}

void PlumeCreatorTest::cleanupTestCase()
{
    QFile testPlumeTempFile(m_testPlumePath.fileName());

    testPlumeTempFile.remove();

    m_testPlumeTempFile.remove();
}

void PlumeCreatorTest::init() {}

void PlumeCreatorTest::cleanup()
{
    QSignalSpy spy(skrdata->projectHub(), SIGNAL(projectClosed(int)));

    skrdata->projectHub()->closeAllProjects();

    //    QCOMPARE(spy.count(), 1);

    //    while (!spy.isEmpty()) {
    //        QList<QVariant> arguments = spy.takeFirst();

    //        // qDebug() << "project n°" <<
    // QString::number(arguments.at(0).toInt())
    //        // << " closed";
    //    }
}

void PlumeCreatorTest::importAll() {
    SKRPlumeCreatorImporter *importer = new SKRPlumeCreatorImporter(this);

    SKRResult result = importer->importPlumeCreatorProject(m_testPlumePath, m_testSkribistoTempFilePath);

    QVERIFY(result.isSuccess());

    int projectId = skrdata->projectHub()->getProjectIdList().first();

    // [Project]
    // - Text [FOLDER]
    // - - Book 1 [FOLDER]
    // - - - Book 1 (content)
    // - - - [book-beginnning]
    // - - - Chapter 1 [FOLDER]
    // - - - - Chapter 1 (content)
    // - - - - [chapter]
    // - - - - Scene 1.1
    // - - - - [separator]
    // - - - - Scene 1.2
    // - - - - Scene 1.3
    // - - - Chapter 2 [FOLDER]
    // - - - - Chapter 2 (content)
    // - - - - [chapter]
    // - - - - Scene 2.1
    // - - - - Scene 2.2
    // - - - - Scene 2.3
    // - - - - [separator]
    // - - - Act 1 [FOLDER]
    // - - - Act 1 (content)
    // - - - - Chapter 1.1 [FOLDER]
    // - - - - - Chapter 1.1 (content)
    // - - - - - [chapter]
    // - - - - - Scene 1.1.1
    // - - - - - Scene 1.1.2
    // - - [book-ending]
    // - - Book 2 [FOLDER]
    // - - - Book 2 (content)
    // - - - [book-beginnning]
    // - - - Act 1 [FOLDER]
    // - - - - Act 1 (content)
    // - - - Act 2 [FOLDER]
    // - - - - Act 2 (content)
    // - - [book-ending]
    // - Attendance [FOLDER]
    // - - group 1 [FOLDER]
    // - - - group 1 (content)
    // - - - obj 1
    // - - - obj 2
    // - - group 2 [FOLDER]
    // - - - group 2 (content)
    // - - - obj 5
    // - Notes [FOLDER]
    // - - Chapter 1
    // - - Scene 1.1

    QList<int> allIds = skrdata->treeHub()->getAllIds(projectId);

    qDebug() << "count:" << skrdata->treeHub()->getAllIds(projectId).count();

    for (int treeItemId : qAsConst(allIds)) {
        qDebug() << "title:" << skrdata->treeHub()->getTitle(projectId, treeItemId) << skrdata->treeHub()->getType(
            projectId,
            treeItemId)  << skrdata->treeHub()->getIndent(projectId, treeItemId);
    }

    QCOMPARE(skrdata->treeHub()->getTitle(projectId, allIds.at(6)),  "Chapter 1 (content)");
    QCOMPARE(skrdata->treeHub()->getTitle(projectId, allIds.at(27)), "Book 2");
    QCOMPARE(skrdata->treeHub()->getTitle(projectId, allIds.at(33)), "Act 2 (content)");
    QCOMPARE(skrdata->treeHub()->getTitle(projectId, allIds.at(34)), "");
    QCOMPARE(skrdata->treeHub()->getTitle(projectId, allIds.at(45)), "Scene 1.1");

    QCOMPARE(skrdata->treeHub()->getAllIds(projectId).count(),       46);
}

QTEST_GUILESS_MAIN(PlumeCreatorTest)

#include "tst_plumecreatorimporter.moc"
