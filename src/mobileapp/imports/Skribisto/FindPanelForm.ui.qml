import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import SkrControls

Item {
    id: base
    property alias findTextField: findTextField
    property alias findPreviousToolButton: findPreviousToolButton
    property alias findNextToolButton: findNextToolButton
    property alias showFindReplaceToolButton: showFindReplaceToolButton
    property alias closeToolButton: closeToolButton
    property alias replaceTextField: replaceTextField
    property alias replaceToolButton: replaceToolButton
    property alias findAndReplaceWithToolButton: findAndReplaceWithToolButton
    property alias replaceRowLayout: replaceRowLayout
    property alias replaceAllToolButton: replaceAllToolButton

    implicitHeight: columnLayout.childrenRect.height

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent
        anchors.margins: 2

        RowLayout {
            id: findRowLayout
            Layout.fillWidth: true

            SkrTextField {
                id: findTextField
                Layout.fillWidth: true
                placeholderText: qsTr("Find")
            }

            SkrToolButton {
                id: findPreviousToolButton
                text: qsTr("Find Previous")
                icon.source: "icons/3rdparty/backup/go-previous.svg"
            }

            SkrToolButton {
                id: findNextToolButton
                text: qsTr("Find Next")
                icon.source: "icons/3rdparty/backup/go-next.svg"
            }
            SkrToolButton {
                id: showFindReplaceToolButton
                text: qsTr("Show replace tool")
                icon.source: "icons/3rdparty/backup/edit-find-replace.svg"
                visible: !replaceRowLayout.visible
            }
            SkrToolButton {
                id: closeToolButton
                text: qsTr("Close")
                icon.source: "icons/3rdparty/backup/arrow-down.svg"
            }
        }

        RowLayout {
            id: replaceRowLayout
            Layout.fillWidth: true
            anchors.margins: 2
            visible: false

            SkrTextField {
                id: replaceTextField
                Layout.fillWidth: true
                placeholderText: qsTr("Replace with")
            }

            SkrToolButton {
                id: replaceToolButton
                text: qsTr("Replace")
                icon.source: "icons/3rdparty/backup/document-edit.svg"
            }

            SkrToolButton {
                id: findAndReplaceWithToolButton
                text: qsTr("Find & Replace")
                icon.source: "icons/3rdparty/backup/edit-find-replace.svg"
            }

            SkrToolButton {
                id: replaceAllToolButton
                text: qsTr("Replace all")
                icon.source: "icons/3rdparty/backup/go-next-skip.svg"
            }
        }
    }
}
