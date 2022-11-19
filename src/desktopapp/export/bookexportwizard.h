#ifndef BOOKEXPORTWIZARD_H
#define BOOKEXPORTWIZARD_H

#include <QCloseEvent>
#include <QWizard>
#include "interfaces/pagetypeiconinterface.h"

namespace Ui {
class BookExportWizard;
}

class BookExportWizard : public QWizard
{
    Q_OBJECT

public:
    explicit BookExportWizard(QWidget *parent, int projectId, bool enablePrint);
    ~BookExportWizard();

    // QDialog interface
public slots:
    void done(int result) override;

    // QWidget interface
protected:
    void closeEvent(QCloseEvent *event) override
    {
        event->accept();
    }

private:
    Ui::BookExportWizard *ui;
    bool m_enablePrint;
    int m_projectId;
    TreeItemAddress m_selectedBookId, m_selectedChapterId;
    QVariantMap m_parameters;
    QHash<QString, PageTypeIconInterface *> m_typeWithPlugin;
    QList<QList<TreeItemAddress>> m_chaptersWithContents;
    QList<TreeItemAddress> m_allItemsOfBook;
    QString m_currentFormat, m_destinationPath;


    void fillChapterListWidget(TreeItemAddress bookItemId);
    QList<TreeItemAddress> determineOutputItems();
    void setupOptionPage();


};

#endif // BOOKEXPORTWIZARD_H
