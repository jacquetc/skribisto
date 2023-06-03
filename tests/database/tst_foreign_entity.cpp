#include "database/database_table.h"
#include "dummy_database_context.h"
#include "dummy_entity.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

class ForeignEntityTest : public QObject
{
    Q_OBJECT

  public:
    ForeignEntityTest();
    ~ForeignEntityTest();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

  private:
    Database::DatabaseTable<Domain::DummyEntity> *m_entityTable;
};

ForeignEntityTest::ForeignEntityTest()
{
}

ForeignEntityTest::~ForeignEntityTest()
{
}

void ForeignEntityTest::initTestCase()
{
    DummyDatabaseContext *context = new DummyDatabaseContext();
    m_entityTable = new Database::DatabaseTable<Domain::DummyEntity>(context);
}

void ForeignEntityTest::cleanupTestCase()
{
}

void ForeignEntityTest::init()
{
}

void ForeignEntityTest::cleanup()
{
    m_entityTable->clear();
}

QTEST_APPLESS_MAIN(ForeignEntityTest)

#include "tst_foreign_entity.moc"
