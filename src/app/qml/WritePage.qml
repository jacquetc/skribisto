import QtQuick 2.9
import "."

WritePageForm {

    //base.onWidthChanged: writingZone.width = base.width / 2

    //writingZone.textAreaWidth: 30
    writingZone.stretch: true
    writingZone.text: "bla"


    Component.onCompleted: {
        treedock.createBody("qrc:/qml/WriteTreeView.qml")

    }
}

