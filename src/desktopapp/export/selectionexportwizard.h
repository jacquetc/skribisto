#ifndef SELECTIONEXPORTWIZARD_H
#define SELECTIONEXPORTWIZARD_H

#include <QCloseEvent>
#include <QWizard>

namespace Ui {
class SelectionExportWizard;
}

class SelectionExportWizard : public QWizard
{
    Q_OBJECT

public:
    explicit SelectionExportWizard(QWidget *parent, int projectId, bool enablePrint);
    ~SelectionExportWizard();

private:
    Ui::SelectionExportWizard *ui;
    bool m_enablePrint;
    int m_projectId;
    QVariantMap m_parameters;
    QString m_currentFormat, m_destinationPath;

    // QDialog interface
    void setupOptionPage();
public slots:
    void done(int) override;

    // QWidget interface
protected:
    void closeEvent(QCloseEvent *event) override
    {
        event->accept();
    }
};

#endif // SELECTIONEXPORTWIZARD_H
