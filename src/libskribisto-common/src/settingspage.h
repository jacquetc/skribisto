#ifndef SETTINGSPAGE_H
#define SETTINGSPAGE_H

#include <QWidget>

namespace Ui {
class SettingsPage;
}

class SettingsPage : public QWidget
{
    Q_OBJECT

public:
    explicit SettingsPage(QWidget *parent = nullptr);
    ~SettingsPage();

    const QString &settingsGroup() const;
    void setSettingsGroup(const QString &newSettingsGroup);

signals:
    void settingsGroupChanged();

private:
    Ui::SettingsPage *ui;
    QString m_settingsGroup;
    Q_PROPERTY(QString settingsGroup READ settingsGroup WRITE setSettingsGroup NOTIFY settingsGroupChanged)
};

#endif // SETTINGSPAGE_H
