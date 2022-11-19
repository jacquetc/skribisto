#ifndef EXPORTERINTERFACE_H
#define EXPORTERINTERFACE_H

#include "skrresult.h"
#include <QString>
#include <QObject>
#include "interfaces/skrcoreinterface.h"
#include "treeitemaddress.h"



class ExporterInterface : public SKRCoreInterface  {

public:

    virtual ~ExporterInterface() {}

    virtual int weight() const = 0;

    virtual QStringList extensions() const = 0;
    virtual QStringList  extensionHumanNames() const = 0;
    virtual QStringList  extensionShortNames() const = 0;


    virtual bool canSave() = 0;

    virtual SKRResult run(QList<TreeItemAddress> treeItemAddresses, const QUrl &url, const QString &extension, const QVariantMap &parameters) const = 0;


};


#define ExporterInterface_iid "com.skribisto.ExporterInterface/1.0"

Q_DECLARE_INTERFACE(ExporterInterface, ExporterInterface_iid)

#endif // EXPORTERINTERFACE_H
