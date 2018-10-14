#include "plmtaskerror.h"
#include "plmdata.h"

PLMTaskError::PLMTaskError(QObject *parent) : QObject(parent)
{
    m_instance = this;
    //connect(this, &PLMTaskError::errorSent, plmdata->errorHub(), &PLMErrorHub::addError, Qt::QueuedConnection);
}

//-----------------------------------------------------------------------------

PLMTaskError* PLMTaskError::m_instance = 0;

