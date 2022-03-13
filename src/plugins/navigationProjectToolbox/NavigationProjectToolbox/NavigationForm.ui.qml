import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window

Item {
    id: base
    property alias  stackView: stackView

    implicitHeight: Window.window === null ? 300 : Window.window.height * 1/2


    StackView {
        id: stackView
        anchors.fill: parent
        initialItem: navigationListComponent

    }
}


/*##^##
Designer {
    D{i:0;autoSize:true;height:300;width:300}
}
##^##*/

