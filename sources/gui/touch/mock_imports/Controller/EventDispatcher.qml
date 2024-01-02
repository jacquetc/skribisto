pragma Singleton

import QtQuick

QtObject {

    function system() {
        return SystemSignals;
    }
    function author() {
        return AuthorSignals;
    }

    function book() {
        return BookSignals;

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


}
