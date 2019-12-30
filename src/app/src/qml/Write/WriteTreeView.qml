import QtQuick 2.4
import eu.skribisto.sheetlistmodel 1.0
import QtQuick.Controls 2.5


WriteTreeViewForm {
listView.model: PLMSheetListModel {

}


listView.delegate: SwipeDelegate {
    id: swipeDelegate
    width: parent.width
    enabled: true
//                    hoverEnabled: true
//                    wheelEnabled: true
    display: AbstractButton.TextBesideIcon
    checkable: true

    text: name
    onClicked: listView.currentIndex = index
    highlighted: ListView.isCurrentItem
    padding: 2


    ListView.onRemove: SequentialAnimation {
        PropertyAction {
            target: swipeDelegate
            property: "ListView.delayRemove"
            value: true
        }
        NumberAnimation {
            target: swipeDelegate
            property: "height"
            to: 0
            easing.type: Easing.InOutQuad
        }
        PropertyAction {
            target: swipeDelegate
            property: "ListView.delayRemove"
            value: false
        }
    }

    swipe.right: Label {
        id: deleteLabel
        text: qsTr("Delete")
        color: "white"
        verticalAlignment: Label.AlignVCenter
        padding: 12
        height: parent.height
        anchors.right: parent.right

        SwipeDelegate.onClicked: listView.model.remove(index)

        background: Rectangle {
            color: deleteLabel.SwipeDelegate.pressed ? Qt.darker("tomato", 1.1) : "tomato"
        }
    }

//                    contentItem: RowLayout {
//                        Image {
//                            id: image
//                                                       fillMode: Image.PreserveAspectFit
//                                                       source: "qrc:/pics/author.svg"
//                        }
//                        Label{
//                            text: delegate.text
//                        }

//                    }


//                    Row {
//                        id: row1
//                        spacing: 10
//                        anchors.fill: parent
//                        //                        Repeater{
//                        //                            model: indent
//                        //                            Item {
//                        //                                width: 20; height: 20
//                        //                            }
//                        //                        }
////                        CheckBox {

////                            //visible: base.selectionMode ? true : false
////                            width: 10
////                            height: 10
////                        }

//                        Image {
//                            id: image
//                            width: 40
//                            height: 40
//                            fillMode: Image.PreserveAspectFit
//                            source: "qrc:/pics/author.svg"
//                        }

//                        Item {
//                            id: item3
//                            width: 200
//                            height: 50

//                            Column {
//                                id: column
//                                anchors.fill: parent

//                                Text {
//                                    text: name
//                                    font.bold: true
//                                }

//                                Text {
//                                    text: tag
//                                    font.pixelSize: 12
//                                }
//                            }
//                        }
//                    }



}
}
