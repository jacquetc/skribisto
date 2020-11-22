import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15

Item {
    id: base
    property alias  stackView: stackView

    implicitHeight: Window.window.height * 3/4


    StackView {
        id: stackView
        anchors.fill: parent
        initialItem: treeListViewComponent

    }
}


/*##^##
Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
##^##*/

