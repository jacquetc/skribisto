// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    function canUndo() {
        return false
    }
    function canRedo() {
        return false
    }
    function setUndoLimit(limit) {
    }
    function undoLimit() {
        return 10
    }
    function undoText() {
        return "undo text"
    }
    function redoText() {
        return "redo text"
    }
    function undoRedoTextList() {
        return ["undo text", "redo text"]
    }
    function currentIndex() {
        return 0
    }
    function activeStackId() {
        return 0
    }
    function queuedCommandTextListByScope(scopeFlagString) {
        return ["queued command text"]
    }
    function isRunning() {
        return false
    }
    function numberOfCommands() {
        return 0
    }
    function undo() {
    }
    function redo() {
    }
    function clear() {
    }
    function setCurrentIndex(index) {
    }
    function setActiveStack(stackId) {
    }

}