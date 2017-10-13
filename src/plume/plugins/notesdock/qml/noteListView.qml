import QtQuick 2.3
import QtQuick.Controls 1.4


Rectangle {
    SystemPalette { id: myPalette; colorGroup: SystemPalette.Active }
    id: base
    color: myPalette.window

    signal openNoteSignal(int note_id)
    signal displayContextMenuSignal(int note_id, point root_point)

    ListView {
        id: list
        width: parent.width
        height: parent.height
        spacing: 1



        Component {
            id: itemDelegate
            Rectangle {
                id: wrapper
                width: list.width
                height: 30
                border.color: "#595959"
                radius: 5
                focus: true
                border.width: wrapper.ListView.isCurrentItem ? 2 : 1
                state : wrapper.ListView.isCurrentItem ? 'extended' : ''
                clip:true
                Column {
                    id: col
                    Text {
                        id: textTitle
                        width: wrapper.width
                        elide: Text.ElideLeft
                        text: "<b>" + model.title + ":</b>"
                        //                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                    }
                    Text {
                        id: textContent
                        elide: Text.ElideLeft
                        width: wrapper.width
                        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                        text: ""
                        // text: model.content
                        //                    color: wrapper.ListView.isCurrentItem ? "red" : "black"
                        MouseArea {
                            anchors.fill: parent
                            preventStealing: true
                            acceptedButtons: Qt.LeftButton

                            onDoubleClicked:                                
                                if (mouse.button === Qt.LeftButton)
                                {
                                    textContent.visible = false
                                    contentTextEdit.visible = true


                                }
                        }



                    }
                    TextArea{
                        id: contentTextEdit
                        width: wrapper.width
                        text: ""
                        // text: model.content
                        visible: false
                    }


                }

                MouseArea {
                    anchors.fill: parent
                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                    onClicked: {

                        if (mouse.button === Qt.LeftButton)
                        {
                            list.currentIndex = index

                        }
                        else if (mouse.button === Qt.RightButton)
                        {
                            list.currentIndex = index
                            base.displayContextMenuSignal(note_id, wrapper.mapToItem(base, mouse.x, mouse.y))
                        }
                    }
                    onDoubleClicked:
                        if (mouse.button === Qt.LeftButton)
                        {

                            base.openNoteSignal(model.note_id)


                        }
                }



                states: [
                    State {
                        name: "extended"
                        PropertyChanges {
                            target: wrapper
                            height: 100
                        }
                    }
                ]
                transitions: Transition {
                    from: "*"; to: "extended"
                    PropertyAnimation { target: wrapper
                        properties: "height"
                        duration: 1000
                        easing.type: Easing.OutExpo}
                }
            }



        }


        //    anchors.fill: parent
        model: myModel
        delegate: itemDelegate
        clip: true


    }
}
