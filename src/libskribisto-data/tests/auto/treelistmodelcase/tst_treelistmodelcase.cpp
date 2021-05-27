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
#include "models/skrtreelistmodel.h"

class TreeListModelCase : public QObject {
    Q_OBJECT

public:

    TreeListModelCase();
    ~TreeListModelCase();

public slots:

private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();
    void sortAlphabetically();

private:

    SKRData *m_data;
    SKRTreeListModel *m_treeListModel;
    QAbstractItemModelTester *m_tester;
    QUrl m_testProjectPath;
    int m_currentProjectId;
};

TreeListModelCase::TreeListModelCase()
{}

TreeListModelCase::~TreeListModelCase()
{}

void TreeListModelCase::initTestCase()
{
    m_data            = new SKRData(this);
    m_treeListModel   = new SKRTreeListModel(this);
    m_testProjectPath = "qrc:/testfiles/skribisto_test_project.skrib";
}

void TreeListModelCase::cleanupTestCase()
{}

void TreeListModelCase::init() {
    m_tester = new QAbstractItemModelTester(m_treeListModel);
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

void TreeListModelCase::cleanup()
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

void TreeListModelCase::sortAlphabetically() {
    skrdata->treeHub()->sortAlphabetically(m_currentProjectId, 0);
}

QTEST_GUILESS_MAIN(TreeListModelCase)

#include "tst_treelistmodelcase.moc"
