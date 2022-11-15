#ifndef THEMEWIZARD_H
#define THEMEWIZARD_H

#include <QPaintEvent>
#include <QStyledItemDelegate>
#include <QTableWidgetItem>
#include <QWizard>
#include "thememanager.h"

namespace Ui {
class ThemeWizard;
}

class ThemeWizard : public QWizard
{
    Q_OBJECT

public:
    explicit ThemeWizard(QWidget *parent = nullptr);
    ~ThemeWizard();

private slots:
    void resetThemeList();

private:
    Ui::ThemeWizard *ui;
    QString m_selectedTheme, m_selectedLocation;
    void setColorTableColors(QMap<QString, QString> colorMap);
    QString getHumanReadableName(const QString &colorPropertyName) const;
    QMap<QString, QString> getColorMapFromTable() const;
    QString m_outputFileName;
    ThemeManager::ThemeType m_outputThemeType;


    // QDialog interface
public slots:
    void done(int result) override;
};



#endif // THEMEWIZARD_H
