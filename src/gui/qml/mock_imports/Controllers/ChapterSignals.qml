// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    signal getReplied(var dto)
    signal getWithDetailsReplied(var dto)
    signal removed(list<int> removedIds)
    signal created(var dto)
    signal updated(var dto)
    signal allRelationsInvalidated(int itemId)
    signal relationInserted(var relationDto)
    signal relationRemoved(var relationDto)
}