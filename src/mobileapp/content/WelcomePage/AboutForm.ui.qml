import QtQuick
import QtQuick.Controls

import SkrControls
import Skribisto

Item {

    property bool qt: false
    property alias textArea: textArea

    ScrollView {
        anchors.fill: parent
        id: scrollView
        padding: 2
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded

        TextArea {
            id: textArea
            width: scrollView.width
            readOnly: true
            color: SkrTheme.buttonForeground
            font.pointSize: 20
            wrapMode: TextEdit.WordWrap
            background: SkrPane {}
        }
    }
}
