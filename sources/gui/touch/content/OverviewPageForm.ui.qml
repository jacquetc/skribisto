

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Skribisto

Page {
    property alias listView: listView
    header: ToolBar {
        id: toolBar

        RowLayout {
            anchors.fill: parent
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("Main menu")
            }
            ToolButton {
                text: qsTr("⋮")
            }
        }
    }
    ColumnLayout {
        anchors.fill: parent

        //        Rectangle {

        //            color: Constants.backgroundColor
        //            Layout.fillHeight: true
        //            Layout.fillWidth: true

        //            //    property alias textAreaOther: textAreaOther
        //            //    property alias textAreaBis: textAreaBis
        //            //    property alias textArea: textArea
        //            property alias listView: listView

        //            ColumnLayout {
        //                id: row
        //                anchors.fill: parent

        //                Rectangle {
        //                    id: rectangle
        //                    color: "#ffffff"
        //                    radius: 8
        //                    Layout.fillWidth: true
        //                    Layout.minimumHeight: toolBar.height + 2
        //                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
        //                    Flickable {
        //                        flickableDirection: Flickable.HorizontalFlick
        //                        anchors.fill: parent
        //                        contentWidth: toolBar.width; contentHeight: toolBar.height

        //                    }
        //                }
        ListView {
            id: listView
            spacing: 4
            Layout.fillWidth: true
            Layout.fillHeight: true
            boundsBehavior: Flickable.StopAtBounds

            delegate: ChapterListDelegate {
                chapterId: model.itemId
                chapterTitle: model.title
                chapterSynopsis: model.synopsis
            }

            ScrollIndicator.vertical: ScrollIndicator {}
        }

        //        ColumnLayout {
        //            id: columnLayout
        //            Layout.minimumWidth: 340
        //            Layout.fillHeight: true
        //            TextArea {
        //                id: textArea
        //                Layout.fillHeight: true
        //                Layout.fillWidth: true
        //                placeholderText: qsTr("Text Area")
        //            }
        //            TextArea {
        //                id: textAreaBis
        //                Layout.fillHeight: true
        //                Layout.fillWidth: true
        //                placeholderText: qsTr("Text Area Bis")
        //            }
        //        }
        //        ColumnLayout {
        //            id: columnLayout2
        //            Layout.minimumWidth: 340
        //            Layout.fillHeight: true
        //            TextArea {
        //                id: textAreaOther
        //                Layout.fillHeight: true
        //                Layout.fillWidth: true
        //                placeholderText: qsTr("Text Area Other")
        //            }
        //        }
    }
}
