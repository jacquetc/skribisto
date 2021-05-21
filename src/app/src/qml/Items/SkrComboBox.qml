import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

ComboBox {
    id: control

    Material.background: SkrTheme.pageBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    property bool sizeToContents: true
    property int modelWidth: 50

    implicitWidth: sizeToContents
                   ? (modelWidth + leftPadding + contentItem.leftPadding
                      + rightPadding + contentItem.rightPadding)
                   : contentItem.implicitWidth


    Component.onCompleted: {
        determineModelWidth()
    }

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus & Globals.focusVisible

    }
    Keys.onPressed: {
        if (event.key === Qt.Key_Tab){
            Globals.setFocusTemporarilyVisible()
        }
    }

    delegate: ItemDelegate {
        id: delegate
        width: control.width
        contentItem: Label {
            id: label
            text: control.textRole ? (Array.isArray(control.model) ? modelData[control.textRole] : model[control.textRole]) : modelData
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
        console.log("1 control.textRole", control.textRole)
        determineModelWidth()
    }

    onModelChanged: {
        modelCountConnection.enabled = true
        determineModelWidth()
    }

    Component.onDestruction: {
        modelCountConnection.enabled = false
    }

    Connections{
        id: modelCountConnection
        target: model
        enabled: false
        function onCountChanged(){
            determineModelWidth()
        }
    }

    function determineModelWidth(){


        if(control.textRole && model){
            for(var i = 0; i < model.count; i++){
                textMetrics.text = model.get(i)[control.textRole]
                console.log("r", model.get(i)[control.textRole])
                modelWidth = Math.max(textMetrics.width, modelWidth)
            }
        }
        else {
            if(Array.isArray(control.model)){
                for(var j = 0; j < model.length; j++){
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

    //                onSingleTapped: {

    //                    eventPoint.accepted = true
    //                }


    //                onGrabChanged: {
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

    popup: SkrPopup {
        y: control.height - 1
        width: control.width

        property int listViewContentHeight: listView.contentHeight
        implicitHeight: listViewContentHeight > 400 ? 400 : listViewContentHeight
        padding: 1

        contentItem: SkrListItemPane{

            contentItem: ScrollView {
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

