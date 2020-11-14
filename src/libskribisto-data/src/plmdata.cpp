#include "plmdata.h"


PLMData::PLMData(QObject *parent) : QObject(parent)
{
    m_instance = this;

    m_signalHub      = new PLMSignalHub(this);
    m_errorHub       = new SKRErrorHub(this);
    m_projectManager = new PLMProjectManager(this);
    m_projectHub     = new PLMProjectHub(this);

    m_sheetHub         = new PLMSheetHub(this);
    m_sheetPropertyHub = new PLMPropertyHub(this,
                                            "tbl_sheet_property",
                                            "l_sheet_code");
    m_noteHub         = new PLMNoteHub(this);
    m_notePropertyHub = new PLMPropertyHub(this,
                                           "tbl_note_property",
                                           "l_note_code");

    m_tagHub         = new SKRTagHub(this);
    m_projectDictHub = new SKRProjectDictHub(this);
    m_pluginHub      = new PLMPluginHub(this);
    m_statHub      = new SKRStatHub(this);


    connect(m_sheetHub,
            &PLMSheetHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_sheetPropertyHub,
            &PLMPropertyHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_noteHub,
            &PLMNoteHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_notePropertyHub,
            &PLMPropertyHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);
    connect(m_tagHub,
            &SKRTagHub::projectModified,
            m_projectHub,
            &PLMProjectHub::setProjectNotSavedAnymore);


    // connect errors :

    connect(m_sheetHub,
            &PLMSheetHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_sheetPropertyHub,
            &PLMPropertyHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_noteHub,
            &PLMNoteHub::errorSent,
            m_errorHub,
            &SKRErrorHub::addError);
    connect(m_notePropertyHub,
            &PLMPropertyHub::errorSent,
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


PLMSheetHub * PLMData::sheetHub()
{
    return m_sheetHub;
}

// -----------------------------------------------------------------------------

PLMPropertyHub * PLMData::sheetPropertyHub()
{
    return m_sheetPropertyHub;
}

// -----------------------------------------------------------------------------

PLMNoteHub * PLMData::noteHub()
{
    return m_noteHub;
}

// -----------------------------------------------------------------------------

PLMPropertyHub * PLMData::notePropertyHub()
{
    return m_notePropertyHub;
}

// -----------------------------------------------------------------------------
PLMPluginHub * PLMData::pluginHub()
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
