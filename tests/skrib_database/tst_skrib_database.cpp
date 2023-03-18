#include "author.h"
#include "database/skrib/skrib_file.h"
#include "database/skrib/skrib_file_context.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Database::Skrib;

class SkribDatabase : public QObject
{
    Q_OBJECT

  public:
    SkribDatabase();
    ~SkribDatabase();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void get();
    void getAll();
    void add();
    void update();
    void remove();
    void exists();

  private:
    QUuid m_authorUuid;
    QUrl m_testProjectPath;
    Database::SkribFileContext *m_context;
};

SkribDatabase::SkribDatabase()
{
}

SkribDatabase::~SkribDatabase()
{
}

void SkribDatabase::initTestCase()
{
    m_authorUuid = QUuid("{aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa}");
    m_testProjectPath = "qrc:/testfiles/test_project.skrib";
}

void SkribDatabase::cleanupTestCase()
{
}

void SkribDatabase::init()
{

    m_context = new Database::SkribFileContext();
    m_context->init();
}

void SkribDatabase::cleanup()
{
}

// ----------------------------------------------------------

void SkribDatabase::get()
{
    SkribFile<Domain::Author> skribFile(m_context);

    Result<Domain::Author> result = skribFile.get(m_authorUuid);
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }

    QVERIFY(result);
    QCOMPARE("test", result.value().getName());
    QCOMPARE("{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}", result.value().relative().toString());
}

void SkribDatabase::getAll()
{
    SkribFile<Domain::Author> skribFile(m_context);

    Result<QList<Domain::Author>> result = skribFile.getAll();
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }

    QVERIFY(result);
    QCOMPARE(1, result.value().count());
    QCOMPARE("test", result.value().first().getName());
    QCOMPARE("{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}", result.value().first().relative().toString());
}

void SkribDatabase::add()
{
    SkribFile<Domain::Author> skribFile(m_context);

    Domain::Author author(QUuid::createUuid(), "New Author", QUuid::createUuid());
    author.setCreationDate(QDateTime::currentDateTime());
    author.setUpdateDate(QDateTime::currentDateTime());

    Result<Domain::Author> result = skribFile.add(std::move(author));
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }

    QVERIFY(result);
    QVERIFY(result.value().getUuid() != QUuid());
}

void SkribDatabase::update()
{
    SkribFile<Domain::Author> skribFile(m_context);

    Result<Domain::Author> result = skribFile.get(m_authorUuid);
    Domain::Author author = result.value();

    author.setName("Updated Author");
    author.setUpdateDate(QDateTime::currentDateTime());

    Result<Domain::Author> updateResult = skribFile.update(std::move(author));
    if (!updateResult)
    {
        qDebug() << updateResult.error().message() << updateResult.error().data();
    }

    QVERIFY(updateResult);
    QCOMPARE("Updated Author", updateResult.value().getName());
}

void SkribDatabase::remove()
{
    SkribFile<Domain::Author> skribFile(m_context);

    Result<Domain::Author> result = skribFile.get(m_authorUuid);
    QCOMPARE("test", result.value().name());

    Result<Domain::Author> removeResult = skribFile.remove(std::move(result.value()));
    QVERIFY(removeResult.isSuccess());

    Result<Domain::Author> removedResult = skribFile.get(m_authorUuid);
    QVERIFY(!removedResult.isSuccess());
}

void SkribDatabase::exists()
{

    SkribFile<Domain::Author> skribFile(m_context);

    Result<bool> existsResult = skribFile.exists(m_authorUuid);
    if (!existsResult)
    {
        qDebug() << existsResult.error().message() << existsResult.error().data();
    }

    QVERIFY(existsResult);
    QVERIFY(existsResult.value());
}

QTEST_MAIN(SkribDatabase)

#include "tst_skrib_database.moc"
