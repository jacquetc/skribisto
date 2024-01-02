

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Page {
    property alias sceneSynopsisListView: sceneSynopsisListView
    property alias chapterTitleLabel: chapterTitleLabel

    header: ToolBar {
        id: toolBar

        RowLayout {
            anchors.fill: parent
            Item {
                Layout.fillWidth: true
            }
            ToolButton {
                text: qsTr("Open")
                display: AbstractButton.IconOnly
            }

            Label {
                id: chapterTitleLabel
                elide: Text.ElideMiddle
                horizontalAlignment: Text.AlignHCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }

            ToolButton {
                text: qsTr("Export")
                display: AbstractButton.IconOnly
            }
        }
    }

    contentItem: ListView {
        id: sceneSynopsisListView
        boundsMovement: Flickable.StopAtBounds
        boundsBehavior: Flickable.StopAtBounds
    }
}
