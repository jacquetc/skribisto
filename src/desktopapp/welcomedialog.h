#pragma once


#include <QWidget>

namespace Ui
{
class WelcomeDialog;
}

class WelcomeDialog : public QWidget
{
    Q_OBJECT

  public:
    explicit WelcomeDialog(QWidget *parent = nullptr);
    ~WelcomeDialog();

  signals:

  private:
    Ui::WelcomeDialog *ui;
    void populateRecentProjects();
};


