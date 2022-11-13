#ifndef NEWPROJECTWIZARD_H
#define NEWPROJECTWIZARD_H

#include <QUrl>
#include <QWizard>
#include "interfaces/newprojectformatinterface.h"

namespace Ui {
class NewProjectWizard;
}

class NewProjectWizard : public QWizard
{
    Q_OBJECT

public:
    explicit NewProjectWizard(QWidget *parent = nullptr);
    ~NewProjectWizard();

    const QString &currentFormat() const;
    void setCurrentFormat(const QString &newCurrentFormat);

    const QUrl &newProjectUrl() const;
    void setNewProjectUrl(const QUrl &newNewProjectUrl);

signals:
    void currentFormatChanged();
    void newProjectUrlChanged();

private:
    void setupInfoPage();
    void setupFormatPage();
    void setupTemplatePage();
    void determineFileName();
    Ui::NewProjectWizard *ui;
    QHash<QString, NewProjectFormatInterface *> m_formatWithPlugin;
    QString m_currentFormat;
    QString m_projectName;
    QString m_validProjectName;
    QString m_selectedTemplateInternalName;
    QUrl m_newProjectUrl;
    bool m_fileNameValidated;
    Q_PROPERTY(QString currentFormat READ currentFormat WRITE setCurrentFormat NOTIFY currentFormatChanged)

    // QDialog interface
    Q_PROPERTY(QUrl newProjectUrl READ newProjectUrl WRITE setNewProjectUrl NOTIFY newProjectUrlChanged)

public slots:
    void accept() override;

    // QDialog interface
public slots:
    void reject() override;
};

#endif // NEWPROJECTWIZARD_H
