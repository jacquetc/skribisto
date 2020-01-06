import QtQuick 2.4
import ".."

ProjectPageForm {

    //compact mode :
    gridLayout.columns: Globals.compactSize ? 1 : 3

    //new_project.onClicked:
    onActiveFocusChanged: {
        if (activeFocus) {

            //sssss.forceActiveFocus()
        }
    }
}
