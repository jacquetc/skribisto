#ifndef SECTIONVIEW_H
#define SECTIONVIEW_H

#include <QWidget>
#include "view.h"

namespace Ui {
class SectionView;
}

class SectionView : public View
{
    Q_OBJECT

public:
    explicit SectionView(QWidget *parent = nullptr);
    ~SectionView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();

private:
    Ui::SectionView *centralWidgetUi;
};

#endif // SECTIONVIEW_H
