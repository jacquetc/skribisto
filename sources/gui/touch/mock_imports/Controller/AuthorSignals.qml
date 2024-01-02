pragma Singleton

import QtQuick

QtObject {


    signal getReplied(var dto)
    signal removed(var removedIds)
    signal activeStatusChanged(list<int> changedIds, bool isActive)
    signal getReplied(var dto)
    signal getAllReplied(list<var> dtoList)
    signal created(var dto)
    signal updated(var dto)
    signal detailsUpdated(int id)


        }
