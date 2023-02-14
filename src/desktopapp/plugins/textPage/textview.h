#ifndef TEXTVIEW_H
#define TEXTVIEW_H

#include "skrwordmeter.h"
#include "view.h"
#include <QWidget>

namespace Ui
{
class TextView;
}

class TextView : public View
{
    Q_OBJECT

  public:
    explicit TextView(QWidget *parent = nullptr);
    ~TextView();
    QList<Toolbox *> toolboxes() override;

  public slots:
    void saveContent(bool sameThread = false);

  protected:
    void initialize() override;

  private:
    Ui::TextView *centralWidgetUi;
    bool m_isSecondaryContent;
    QMetaObject::Connection m_saveConnection;
    QTimer *m_saveTimer, *m_historyTimer;
    bool m_wasModified;
    int m_oldCursorPosition;
    SKRWordMeter *m_localWordMeter;

    void saveTextState();
    void connectSaveConnection();
    void addPositionToHistory();

  protected:
    void mousePressEvent(QMouseEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;
    void applySettingsChanges(const QHash<QString, QVariant> &newSettings) override;

    // View interface
  public:
    void applyParameters() override;
    void applyHistoryParameters(const QVariantMap &parameters) override;

    // View interface
  protected:
    QVariantMap addOtherViewParametersBeforeSplit() override;
};

#endif // TEXTVIEW_H
