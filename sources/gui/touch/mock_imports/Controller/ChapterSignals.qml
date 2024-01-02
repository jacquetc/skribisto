pragma Singleton

import QtQuick

QtObject {


    signal getReplied(var dto)
    signal getWithDetailsReplied(var dto);
    signal removed(var removedIds)
    signal activeStatusChanged(list<int> changedIds, bool isActive)
    signal getAllReplied(list<var> dtoList)
    signal created(var dto)
    signal updated(var dto)
    signal detailsUpdated(int id)
    signal insertedIntoBookChapters(var dto)


        }
