import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    width: 400
    height: 400

    property alias stackView: stackView

    SkrPane {
        id: pane1
        anchors.fill: parent


        StackView {
            id: stackView

            anchors.fill: parent

            clip: true

        }

    }
}

/*##^##
Designer {
    D{i:0;height:800;width:800}
}
##^##*/

