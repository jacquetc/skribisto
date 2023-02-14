#ifndef OUTLINETOOLBOX_H
#define OUTLINETOOLBOX_H

#include <QWheelEvent>
#include <QWidget>
#include <QIcon>
#include "toolbox.h"
#include "skribisto_desktop_common_global.h"

namespace Ui {
class OutlineToolbox;
}

class SKRDESKTOPCOMMONEXPORT OutlineToolbox : public Toolbox
{
    Q_OBJECT

public:
    explicit OutlineToolbox(QWidget *parent = nullptr);
    ~OutlineToolbox();

    // Toolbox interface
public:
    QString title() const override{
        return tr("Outline");
    }
    QIcon icon() const override{
        return QIcon(":/icons/backup/story-editor.svg");
    }
    void initialize() override;

public slots:
    void saveContent();

protected:
    void wheelEvent(QWheelEvent *event) override;
    void applySettingsChanges(const QHash<QString, QVariant> &newSettings) override;
private:
    Ui::OutlineToolbox *ui;

    QMetaObject::Connection m_saveConnection;
    QTimer *m_saveTimer;
    bool m_wasModified;

    void saveTextState();
    void connectSaveConnection();

};

#endif // OUTLINETOOLBOX_H
