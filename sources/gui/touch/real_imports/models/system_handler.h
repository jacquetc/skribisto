#pragma once

#include <QObject>
#include <QQmlEngine>

class SystemHandler : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(System)
    QML_SINGLETON
  public:
    SystemHandler();

  public slots:
    void loadSystem(const QJSValue &jsDto);
    void saveSystem();
    void saveSystemAs(const QJSValue &jsDto);
    void closeSystem();

  signals:

    void loadSystemProgressFinished();
    void loadSystemProgressRangeChanged(int minimum, int maximum);
    void loadSystemProgressTextChanged(const QString &progressText);
    void loadSystemProgressValueChanged(int progressValue);
    void systemLoaded();
    void systemSaved();
    void systemClosed();
};
