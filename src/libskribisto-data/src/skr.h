#ifndef SKR_H
#define SKR_H

#include <QtCore>
#include <QObject>
#include "./skribisto_data_global.h"

class EXPORT SKR : public QObject {
    Q_OBJECT

public:

    enum ItemType {
        Sheet,
        Note
    };
    Q_ENUM(ItemType)
};

#endif // SKR_H
