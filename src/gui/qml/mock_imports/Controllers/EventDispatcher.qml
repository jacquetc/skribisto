// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    function progress() {
        return ProgressSignals;
    }
    function error() {
        return ErrorSignals;
    }

    function undoRedo() {
        return UndoRedoSignals;
    }



    function user() {
        return UserSignals;
    }

    function book() {
        return BookSignals;
    }

    function workspace() {
        return WorkspaceSignals;
    }

    function atelier() {
        return AtelierSignals;
    }

    function chapter() {
        return ChapterSignals;
    }

    function scene() {
        return SceneSignals;
    }

    function sceneParagraph() {
        return SceneParagraphSignals;
    }

    function system() {
        return SystemSignals;
    }

}