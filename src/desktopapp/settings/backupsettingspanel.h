#ifndef BACKUPSETTINGSPANEL_H
#define BACKUPSETTINGSPANEL_H

#include "basicsettingspanel.h"
#include <QWidget>

namespace Ui {
class BackupSettingsPanel;
}

class BackupSettingsPanel : public BasicSettingsPanel
{
    Q_OBJECT

public:
    explicit BackupSettingsPanel(QWidget *parent = nullptr);
    ~BackupSettingsPanel();

    void accept() override;
    void reset() override;

private slots:
    void checkConfiguration();

private:
    Ui::BackupSettingsPanel *ui;
    QList<QUrl> m_urlPathsToRemove, m_urlPathsToAdd;
    QHash<QString, QVariant> m_defaultValuesHash;
};

#endif // BACKUPSETTINGSPANEL_H
