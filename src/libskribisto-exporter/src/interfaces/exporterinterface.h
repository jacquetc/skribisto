#ifndef EXPORTERINTERFACE_H
#define EXPORTERINTERFACE_H

#include "skrresult.h"
#include <QString>
#include <QObject>
#include <skrcoreinterface.h>



class ExporterInterface : public SKRCoreInterface  {

public:

    virtual ~ExporterInterface() {}

    virtual QString extension() const = 0;
    virtual QString extensionHumanName() const = 0;
    virtual QString extensionShortName() const = 0;

    virtual SKRResult run(int projectId, const QUrl &url, const QVariantMap &parameters, QList<int> treeItemIds) const = 0;


};


#define ExporterInterface_iid "com.skribisto.ExporterInterface/1.0"

Q_DECLARE_INTERFACE(ExporterInterface, ExporterInterface_iid)

#endif // EXPORTERINTERFACE_H
