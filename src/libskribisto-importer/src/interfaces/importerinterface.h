#ifndef IMPORTERINTERFACE_H
#define IMPORTERINTERFACE_H

#include "skrresult.h"
#include <QString>
#include <QObject>
#include <skrcoreinterface.h>



class ImporterInterface : public SKRCoreInterface   {

public:

    virtual ~ImporterInterface() {}

    virtual QString extension() const = 0;
    virtual QString extensionHumanName() const = 0;
    virtual QString extensionShortName() const = 0;

    virtual QString importProject(const QUrl &url, const QVariantMap &parameters, SKRResult    & result) const = 0;


};


#define ImporterInterface_iid "com.skribisto.ImporterInterface/1.0"

Q_DECLARE_INTERFACE(ImporterInterface, ImporterInterface_iid)

#endif // IMPORTERINTERFACE_H
