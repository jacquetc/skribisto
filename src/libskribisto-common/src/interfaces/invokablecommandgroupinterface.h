#ifndef INVOKABLECOMMANDGROUPINTERFACE_H
#define INVOKABLECOMMANDGROUPINTERFACE_H

#include <QString>
#include <QObject>
#include "command.h"



class InvokableCommandGroupInterface  {

public:

    virtual ~InvokableCommandGroupInterface() {}

    virtual QString address() const = 0;

    virtual Command *getCommand(const QString &action, const QVariantMap &parameters) = 0;

};


#define InvokableCommandGroupInterface_iid "com.skribisto.InvokableCommandGroupInterface/1.0"

Q_DECLARE_INTERFACE(InvokableCommandGroupInterface, InvokableCommandGroupInterface_iid)

#endif // INVOKABLECOMMANDGROUPINTERFACE_H
