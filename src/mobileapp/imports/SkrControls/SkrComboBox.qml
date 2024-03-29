import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import theme 1.0
import Skribisto

ComboBox {
    id: control

    Material.background: SkrTheme.pageBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    property bool sizeToContents: true
    property int modelWidth: 50

    font.pointSize: Application.font.pointSize * SkrSettings.interfaceSettings.zoom

    implicitWidth: sizeToContents ? (modelWidth + leftPadding
                                     + contentItem.leftPadding + rightPadding
                                     + contentItem.rightPadding) : contentItem.implicitWidth

    Component.onCompleted: {
        determineModelWidth()
    }

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus & Globals.focusVisible
    }
    Keys.onPressed: function (event) {
        if (event.key === Qt.Key_Tab) {
            Globals.setFocusTemporarilyVisible()
        }
        if (event.key === Qt.Key_Backtab) {
            Globals.setFocusTemporarilyVisible()
        }
    }

    delegate: ItemDelegate {
        id: delegate
        width: control.width
        contentItem: Label {
            text: control.textRole ? (Array.isArray(
                                          control.model) ? modelData[control.textRole] : model[control.textRole]) : modelData
            color: SkrTheme.buttonForeground
            font: control.font
            elide: Text.ElideRight
            verticalAlignment: Text.AlignVCenter
        }
        highlighted: control.highlightedIndex === index
        hoverEnabled: control.hoverEnabled
    }

    TextMetrics {
        id: textMetrics
        font: control.font
    }
    onTextRoleChanged: {
        determineModelWidth()
    }

    onModelChanged: {
        modelCountConnection.enabled = true
        determineModelWidth()
    }

    Component.onDestruction: {
        modelCountConnection.enabled = false
        control.delegate = Qt.createComponent("Item{}")
    }

    Connections {
        id: modelCountConnection
        target: Qt.isQtObject(model) ? model : null
        enabled: Qt.isQtObject(model)
        function onCountChanged() {
            determineModelWidth()
        }
    }

    function determineModelWidth() {

        if (control.textRole && model) {
            for (var i = 0; i < model.count; i++) {
                textMetrics.text = model.get(i)[control.textRole]
                modelWidth = Math.max(textMetrics.width, modelWidth)
            }
        } else {
            if (Array.isArray(control.model)) {
                for (var j = 0; j < model.length; j++) {
                    textMetrics.text = model[j]
                    modelWidth = Math.max(textMetrics.width, modelWidth)
                }
            }
        }
    }

    //        SkrListItemPane {
    //            id: inner_delegateRoot
    //            height: 30
    //            focus: true

    //            Accessible.name: model.dataValue
    //            Accessible.role: Accessible.ListItem

    //            anchors {
    //                left: Qt.isQtObject(parent) ? parent.left : undefined
    //                right: Qt.isQtObject(parent) ? parent.right : undefined
    //                leftMargin: 5
    //                rightMargin: 5
    //            }

    //            TapHandler {
    //                id: inner_tapHandler

    //                onSingleTapped: function(eventPoint) {

    //                    eventPoint.accepted = true
    //                }

    //                onGrabChanged: function(transition, point) {
    //                    point.accepted = false
    //                }
    //            }

    //            SkrLabel {
    //                text: modelData
    //                anchors.fill: parent
    //                horizontalAlignment: Qt.AlignLeft
    //                verticalAlignment: Qt.AlignVCenter

    //                highlighted: control.highlightedIndex === index
    //            }

    //}

    //}
    popup: popup
    SkrPopup {
        id: popup
        y: control.height - 1
        width: control.width

        property int listViewContentHeight: listView.contentHeight
        implicitHeight: listViewContentHeight > 400 ? 400 : listViewContentHeight
        padding: 1

        contentItem: listItemPane
        SkrListItemPane {
            id: listItemPane

            contentItem: scrollView
            ScrollView {
                id: scrollView
                clip: true
                anchors.fill: parent

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                focusPolicy: Qt.StrongFocus
                implicitHeight: contentHeight

                ListView {
                    id: listView
                    clip: true
                    model: control.popup.visible ? control.delegateModel : null
                    currentIndex: control.highlightedIndex
                    interactive: true
                    smooth: true
                    focus: true
                }
            }
        }

        background: Rectangle {
            border.color: SkrTheme.pageBackground
            radius: 2
        }
    }
}
