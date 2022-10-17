#include "command.h"

Command::Command(const QString &text)
    : QUndoCommand(text)
{

}
