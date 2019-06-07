#ifndef GLOBAL_CORE_H
#define GLOBAL_CORE_H

#include <QtCore/QtGlobal>

#if defined(PLUME_CREATOR_CORE_LIBRARY)
#  define EXPORT_CORE Q_DECL_EXPORT
#else
#  define EXPORT_CORE Q_DECL_IMPORT
#endif

#endif // GLOBAL_CORE_H
