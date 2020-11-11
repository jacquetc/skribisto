#ifndef SKR_H
#define SKR_H

#include <QtCore>
#include <QObject>
#include "./skribisto_data_global.h"

class EXPORT SKR : public QObject {
    Q_OBJECT

public:

    enum PaperType {
        Sheet,
        Note
    };
    Q_ENUM(PaperType)
};

#endif // SKR_H
