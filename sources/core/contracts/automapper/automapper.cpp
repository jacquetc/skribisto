#include "automapper.h"

// QHash<QMetaType, std::function<QVariant(const QVariant&)>>
// AutoMapper::AutoMapper::conversions;
QHash<QMetaType, std::function<QVariant()> > AutoMapper::AutoMapper::getSiblingFunctions;

QHash<QMetaType, std::function<bool(const QMetaProperty&  destinationProperty, void *gadgetPointer,
                                    const QVariant& sourceValue)> > AutoMapper::AutoMapper::writerHash;

QHash<QMetaType, std::function<bool(const QMetaProperty&  destinationProperty, void *gadgetPointer,
                                    const QList<QVariant>& sourceList)> > AutoMapper::AutoMapper::writerForListHash;
QMutex AutoMapper::AutoMapper::mutex;
