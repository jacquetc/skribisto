#include "skrdata.h"
#include "treeitemaddress.h"
#include <QDebug>
#include <QCoreApplication>

SKRData::SKRData(QObject *parent) : QObject(parent)
{
    m_instance = this;


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
    m_statHub        = new SKRStatHub(this);

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

SKRData::~SKRData()
{}

// -----------------------------------------------------------------------------

SKRErrorHub * SKRData::errorHub()
{
    return m_errorHub;
}

// -----------------------------------------------------------------------------


PLMProjectHub * SKRData::projectHub()
{
    return m_projectHub;
}

// -----------------------------------------------------------------------------


SKRData *SKRData::m_instance = nullptr;

// -----------------------------------------------------------------------------


SKRTreeHub * SKRData::treeHub()
{
    return m_treeHub;
}

// -----------------------------------------------------------------------------

SKRPropertyHub * SKRData::treePropertyHub()
{
    return m_treePropertyHub;
}

// -----------------------------------------------------------------------------
SKRPluginHub * SKRData::pluginHub()
{
    return m_pluginHub;
}

// -----------------------------------------------------------------------------

SKRTagHub * SKRData::tagHub()
{
    return m_tagHub;
}

// -----------------------------------------------------------------------------

SKRProjectDictHub * SKRData::projectDictHub()
{
    return m_projectDictHub;
}

// -----------------------------------------------------------------------------

SKRStatHub * SKRData::statHub()
{
    return m_statHub;
}
