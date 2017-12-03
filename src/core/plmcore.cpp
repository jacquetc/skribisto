#include "plmcore.h"
#include "plmmessagehandler.h"

#include <QFileInfo>
#include <QDebug>
#include <QSettings>
#include "plmdata.h"


PLMCore::PLMCore(QObject *parent) :
    QObject(parent)
{

    m_instance = this;
    new PLMMessageHandler(this);
    new PLMData(this);

    QSettings().setValue("Core/firstStart", false);
}


//-----------------------------------------------------------------------------

PLMCore::~PLMCore()
{

}

//-----------------------------------------------------------------------------


PLMCore* PLMCore::m_instance = 0;

//-----------------------------------------------------------------------------



void PLMCore::startProject(int projectId, const QString& fileName)
{
    //Empty project with no saves yet. Add a blank sheet to write in :

    if(QFileInfo(fileName).exists() || fileName == ""){

        //        connect(m_project, SIGNAL(projectStarted(bool)), this, SIGNAL(projectStarted(bool)), Qt::UniqueConnection);
        //        connect(m_project, SIGNAL(projectStarted(bool)), this, SLOT(setIsProjectStarted(bool)), Qt::UniqueConnection);

        //        m_project->set_point_plume_fileName(_point_plume_fileName);
        //        m_project->openProject();


        //        connect(m_project->textRepository(), SIGNAL(externalChangesWarning(Warning)), this, SIGNAL(externalChangesWarning(Warning)), Qt::UniqueConnection);
        PLMProject *project = new PLMProject(this);
        m_projectForIdHash.insert(projectId, project);
        project->setProjectId(projectId);
        project->loadProject(fileName);
        if(projectId == 0)
            emit mainProjectStarted(true);
    }
    else
        qCritical() << "error : Core::startProject : !QFileInfo(point_plume_fileName).exists()";
}

//----------------------------------------------------------------

bool PLMCore::closeProject(int projectId)
{
    if(!m_projectForIdHash.contains(projectId))
        return false;
    m_projectForIdHash.remove(projectId);
    delete m_projectForIdHash.value(projectId);
    if(projectId == 0)
        emit mainProjectStarted(false);

    return true;
}

bool PLMCore::isMainProjectStarted()
{
    if(m_projectForIdHash.contains(0))
        return true;
    return false;
}

//--------------------------------------------------------------------------------------

PLMProject *PLMCore::getProject(int projectId){
    if(m_projectForIdHash.contains(projectId))
        return m_projectForIdHash.value(projectId);
    return new PLMProject();
}









