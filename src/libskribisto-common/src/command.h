#ifndef COMMAND_H
#define COMMAND_H

#include <QObject>
#include <QUndoCommand>
#include "skribisto_common_global.h"

class SKRCOMMONEXPORT Command : public QUndoCommand
{
   Q_GADGET
public:
    explicit Command(const QString &text);

};

#endif // COMMAND_H
