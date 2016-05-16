import QtQuick 2.3

ListView {
    id: list
    width: 180
    height: 60
    spacing: 1
    signal openNoteSignal(int id)
    signal displayContextMenuSignal(int id, int x, int y)

    Component {
        id: itemDelegate
        Rectangle {
            id: wrapper
            width: list.width
            height: col.height
            border.color: "#595959"
            radius: 5
            focus: true
            border.width: wrapper.ListView.isCurrentItem ? 2 : 1
            Column {
                id: col
                Text {
                    id: itemTitle
                    x: 2
                    width: wrapper.width
                    elide: Text.ElideLeft
                    text: "<b>" + title + ":</b>"
                    //                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                }
                Text {
                    id: itemContent
                    x: 2
                    elide: Text.ElideLeft
                    width: wrapper.width
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                    maximumLineCount: 1
                    text: content
                    //                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                }

            }

            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onClicked: {

                    if (mouse.button == Qt.LeftButton)
                    {
                        list.currentIndex = index;
                        wrapper.state = 'extended'

                    }
                    else if (mouse.button == Qt.RightButton)
                    {
                        list.currentIndex = index ;
                        list.displayContextMenuSignal(id, mouse.x, mouse.y)
                    }
                }
                onDoubleClicked: list.openNoteSignal(id)
            }
            states: [
                State {
                    name: "extended"
                    PropertyChanges {
                        target: wrapper
                        height: col.height*2
                    }
                }
            ]
        }

    }



    //    anchors.fill: parent
    model: myModel
    delegate: itemDelegate
    clip: true


}
