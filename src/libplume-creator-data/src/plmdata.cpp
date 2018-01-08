#include "plmdata.h"

PLMData::PLMData(QObject *parent) : QObject(parent)
{
    m_instance = this;
    m_signalHub = new PLMSignalHub(this);
    m_errorHub = new PLMErrorHub(this);
    m_projectManager = new PLMProjectManager(this);
    m_projectHub = new PLMProjectHub(this);
    m_sheetHub = new PLMSheetHub(this);
    m_sheetPropertyHub = new PLMPropertyHub(this, "tbl_sheet_property", "l_sheet_code");
    m_noteHub = new PLMNoteHub(this);
    m_notePropertyHub = new PLMPropertyHub(this, "tbl_sheet_property", "l_note_code");
}


//-----------------------------------------------------------------------------

PLMData::~PLMData()
{
}

//-----------------------------------------------------------------------------

PLMSignalHub *PLMData::signalHub()
{
    return m_signalHub;
}

//-----------------------------------------------------------------------------

PLMErrorHub *PLMData::errorHub()
{
    return m_errorHub;
}

//-----------------------------------------------------------------------------


PLMProjectHub *PLMData::projectHub()
{
    return m_projectHub;
}

//-----------------------------------------------------------------------------


PLMData *PLMData::m_instance = 0;

//-----------------------------------------------------------------------------


PLMSheetHub *PLMData::sheetHub()
{
    return m_sheetHub;
}

//-----------------------------------------------------------------------------

PLMPropertyHub *PLMData::sheetPropertyHub()
{
    return m_sheetPropertyHub;
}

//-----------------------------------------------------------------------------

PLMNoteHub *PLMData::noteHub()
{
    return m_noteHub;
}

//-----------------------------------------------------------------------------

PLMPropertyHub *PLMData::notePropertyHub()
{
    return m_notePropertyHub;
}

//-----------------------------------------------------------------------------
