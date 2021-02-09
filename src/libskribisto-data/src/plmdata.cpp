#include "plmdata.h"

PLMData::PLMData(QObject *parent) : QObject(parent)
{
    m_instance = this;

    m_signalHub      = new PLMSignalHub(this);
    m_errorHub       = new SKRErrorHub(this);
    m_projectManager = new PLMProjectManager(this);
    m_projectHub     = new PLMProjectHub(this);

    m_treeHub         = new SKRTreeHub(this);
    m_treePropertyHub = new SKRPropertyHub(this,
                                            "tbl_tree_property",
                                            "l_tree_code");
    m_tagHub         = new SKRTagHub(this);
    m_projectDictHub = new SKRProjectDictHub(this);
    m_pluginHub      = new SKRPluginHub(this);
    m_statHub      = new SKRStatHub(this);


    connect(m_treeHub,
            &SKRTreeHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_treePropertyHub,
            &SKRPropertyHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_tagHub,
            &SKRTagHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);


    // connect errors :

    connect(m_treeHub,
            &SKRTreeHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_treePropertyHub,
            &SKRPropertyHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_tagHub,
            &SKRTagHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_projectHub,
            &PLMProjectHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);



}

// -----------------------------------------------------------------------------

PLMData::~PLMData()
{}

// -----------------------------------------------------------------------------

PLMSignalHub * PLMData::signalHub()
{
    return m_signalHub;
}

// -----------------------------------------------------------------------------

SKRErrorHub * PLMData::errorHub()
{
    return m_errorHub;
}

// -----------------------------------------------------------------------------


PLMProjectHub * PLMData::projectHub()
{
    return m_projectHub;
}

// -----------------------------------------------------------------------------


PLMData *PLMData::m_instance = nullptr;

// -----------------------------------------------------------------------------


SKRTreeHub * PLMData::treeHub()
{
    return m_treeHub;
}

// -----------------------------------------------------------------------------

SKRPropertyHub * PLMData::treePropertyHub()
{
    return m_treePropertyHub;
}

// -----------------------------------------------------------------------------
SKRPluginHub * PLMData::pluginHub()
{
    return m_pluginHub;
}

// -----------------------------------------------------------------------------

SKRTagHub * PLMData::tagHub()
{
    return m_tagHub;
}

// -----------------------------------------------------------------------------

SKRProjectDictHub * PLMData::projectDictHub()
{
    return m_projectDictHub;
}

// -----------------------------------------------------------------------------

SKRStatHub *PLMData::statHub()
{
    return m_statHub;
}
