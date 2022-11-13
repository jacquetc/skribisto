#ifndef IMPORTER_H
#define IMPORTER_H

#include <QObject>
#include "skribisto_importer_global.h"
#include "skrresult.h"

class SKRIMPORTEREXPORT Importer : public QObject
{
    Q_OBJECT
public:
    explicit Importer(QObject *parent = nullptr);

    static void init();

    static int importProject(const QUrl &url, const QString &extension, const QVariantMap &parameters, SKRResult &result);

signals:

};

#endif // IMPORTER_H
