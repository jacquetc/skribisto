#ifndef PROJECTDICTDIALOG_H
#define PROJECTDICTDIALOG_H

#include <QDialog>

namespace Ui
{
class ProjectDictDialog;
}

class ProjectDictDialog : public QDialog
{
    Q_OBJECT

  public:
    explicit ProjectDictDialog(QWidget *parent, int projectId);
    ~ProjectDictDialog();

  private slots:
    void populateWordList(const QString &filter = QString());

  private:
    Ui::ProjectDictDialog *ui;
    int m_projectId;
    QStringList m_newWordList;
    QStringList m_deletedWordList;
    QStringList m_originalWords;
};

#endif // PROJECTDICTDIALOG_H
