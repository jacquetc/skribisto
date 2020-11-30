import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

ComboBox {
    id: control

    Material.background: SkrTheme.pageBackground
    Material.foreground: control.activeFocus ?  SkrTheme.accent : SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent


    delegate: ItemDelegate {
        id: inner_itemDelegate
        width: control.width
        contentItem: Label {
            text: modelData
            color: SkrTheme.buttonForeground
            font: control.font
            elide: Text.ElideRight
            verticalAlignment: Text.AlignVCenter
        }
        highlighted: control.highlightedIndex === index

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
        implicitHeight: 400
        padding: 1

        contentItem: SkrListItemPane{

            ScrollView {
                id: scrollView
                clip: true
                anchors.fill: parent

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                focusPolicy: Qt.StrongFocus
                implicitHeight: contentHeight

                ListView {
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

