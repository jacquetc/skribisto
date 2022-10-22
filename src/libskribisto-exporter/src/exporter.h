#ifndef EXPORTER_H
#define EXPORTER_H

#include <QObject>
#include "skribisto_exporter_global.h"
#include "skrresult.h"

class SKREXPORTEREXPORT Exporter : public QObject
{
    Q_OBJECT
public:
    explicit Exporter(QObject *parent = nullptr);

    static void init();

    static SKRResult exportProject(int projectId, const QUrl &url, const QString &exportType, const QVariantMap &parameters, QList<int> treeItemIds = QList<int>());

signals:

};

#endif // EXPORTER_H
