#ifndef GLOBAL_H
#define GLOBAL_H

#include <QtCore/QtGlobal>

#if defined(PLUME_CREATOR_GUI_LIBRARY)
#  define EXPORT_GUI Q_DECL_EXPORT
#else
#  define EXPORT_GUI Q_DECL_IMPORT
#endif

#endif // GLOBAL_H
