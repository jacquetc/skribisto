#include "plmnotelistmodel.h"
#include "skr.h"

PLMNoteListModel::PLMNoteListModel(QObject *parent)
    : SKRPaperListModel(parent, SKR::Note)
{}
