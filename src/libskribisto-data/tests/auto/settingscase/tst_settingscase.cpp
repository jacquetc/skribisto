#include <QString>
#include <QtTest>
#include <QDebug>
#include <QVariant>
#include <QByteArray>

#include "plmdata.h"

class SettingsCase : public QObject
{
    Q_OBJECT

public:
    SettingsCase();

public slots:
private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void userDBLoaded();
//    void setSplitterState();
//    void getSplitterState();
//    void setDocVisibleState();
//    void getDocVisibleState();
private:
    PLMData *m_data;
    QString m_testProjectPath;
    int m_currentProjectId;

};

SettingsCase::SettingsCase()
{
}

void SettingsCase::initTestCase()
{
    m_data = new PLMData(this);
    m_testProjectPath = ":/testfiles/skribisto_test_project.sqlite";
}

void SettingsCase::cleanupTestCase()
{
}

void SettingsCase::init()
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

void SettingsCase::cleanup()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectClosed(int)));
    plmdata->projectHub()->closeAllProjects();
    QCOMPARE(spy.count(), 1);

    while (!spy.isEmpty()) {
        QList<QVariant> arguments = spy.takeFirst();
        //qDebug() << "project n°" << QString::number(arguments.at(0).toInt()) << " closed";
    }
}

void SettingsCase::userDBLoaded()
{
    // plmdata->projectHub()->
}

//void SettingsCase::setSplitterState()
//{
//    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(settings_settingChanged(PLMPaperHub::Stack, PLMPaperHub::Setting, QVariant)));
//    plmdata->sheetHub()->settings_setStackSetting(PLMPaperHub::Zero, PLMPaperHub::SplitterState, QByteArray("0"));
//    QCOMPARE(spy.count(), 1);
//    // make sure the signal was emitted exactly one time
//    QList<QVariant> arguments = spy.takeFirst(); // take the first signal
//    QVERIFY(arguments.at(2).toByteArray() == QByteArray("0"));
//}

//void SettingsCase::getSplitterState()
//{
//    QByteArray value = plmdata->sheetHub()->settings_getStackSetting(PLMPaperHub::Zero, PLMPaperHub::SplitterState).toByteArray();
//    QCOMPARE(value, QByteArray("0"));
//}

//void SettingsCase::setDocVisibleState()
//{
//    QSignalSpy spy(plmdata->sheetHub(), SIGNAL(settings_docSettingChanged(int, int, PLMPaperHub::OpenedDocSetting, QVariant)));
//    plmdata->sheetHub()->settings_setDocSetting(m_currentProjectId, 3, PLMPaperHub::Visible, false);
//    QCOMPARE(spy.count(), 1);
//    // make sure the signal was emitted exactly one time
//    QList<QVariant> arguments = spy.takeFirst(); // take the first signal
//    QVERIFY(arguments.at(3).toBool() == false);
//}

//void SettingsCase::getDocVisibleState()
//{
//    bool value = plmdata->sheetHub()->settings_getDocSetting(m_currentProjectId, 3, PLMPaperHub::Visible).toBool();
//    QCOMPARE(value, true);
//}
QTEST_GUILESS_MAIN(SettingsCase)

#include "tst_settingscase.moc"
