#include "database/database_table.h"
#include "database/tools.h"
#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Database;

class DatabaseToolsTest : public QObject
{
    Q_OBJECT

  public:
    DatabaseToolsTest();
    ~DatabaseToolsTest();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testStringCaseConversion();
    void testGetEntityTableName();

  private:
    Database::DatabaseTable<Domain::DummyBasicEntity> *m_entityTable;
};

DatabaseToolsTest::DatabaseToolsTest()
{
}

DatabaseToolsTest::~DatabaseToolsTest()
{
}

void DatabaseToolsTest::initTestCase()
{

    DummyDatabaseContext<Domain::DummyBasicEntity, Domain::DummyBasicEntity> *context =
        new DummyDatabaseContext<Domain::DummyBasicEntity, Domain::DummyBasicEntity>();
    context->setEntityClassNames(QStringList() << "DummyEntity");
    context->init();
    m_entityTable = new Database::DatabaseTable<Domain::DummyBasicEntity>(context);
}

void DatabaseToolsTest::cleanupTestCase()
{
}

void DatabaseToolsTest::init()
{
}

void DatabaseToolsTest::cleanup()
{
    m_entityTable->clear();
}

void DatabaseToolsTest::testStringCaseConversion()
{
    // Test data
    QStringList pascalCaseStrings = {"PascalCaseExample", "AnotherExample", "TestString", "Single"};

    QStringList camelCaseStrings = {"pascalCaseExample", "anotherExample", "testString", "single"};

    QStringList snakeCaseStrings = {"pascal_case_example", "another_example", "test_string", "single"};

    // Test fromPascalToSnakeCase
    for (int i = 0; i < pascalCaseStrings.size(); ++i)
    {
        QString pascalCaseString = pascalCaseStrings.at(i);
        QString expectedSnakeCaseString = snakeCaseStrings.at(i);
        QCOMPARE(Tools<Domain::DummyBasicEntity>::fromPascalToSnakeCase(pascalCaseString), expectedSnakeCaseString);
    }
    // Test fromSnakeCaseToCamel
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedCamelCaseString = camelCaseStrings.at(i);
        QCOMPARE(Tools<Domain::DummyBasicEntity>::fromSnakeCaseToCamelCase(snakeCaseString), expectedCamelCaseString);
    }
    // Test fromSnakeCaseToPascal
    for (int i = 0; i < snakeCaseStrings.size(); ++i)
    {
        QString snakeCaseString = snakeCaseStrings.at(i);
        QString expectedPascalCaseString = pascalCaseStrings.at(i);
        QCOMPARE(Tools<Domain::DummyBasicEntity>::fromSnakeCaseToPascalCase(snakeCaseString), expectedPascalCaseString);
    }
}

void DatabaseToolsTest::testGetEntityTableName()
{
    // Create a Tools object for DummyEntity
    Tools<Domain::DummyBasicEntity> dummyEntityTools;

    // Call the getEntityClassName method
    QString entityClassName = dummyEntityTools.getEntityTableName();

    // Verify the output
    QCOMPARE(entityClassName, QString("dummy_basic_entity"));
}

QTEST_APPLESS_MAIN(DatabaseToolsTest)

#include "tst_database_tools.moc"
