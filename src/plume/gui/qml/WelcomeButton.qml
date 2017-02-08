import QtQuick 2.4
import QtQuick.Controls 1.4
import QtQuick.Controls.Styles 1.4


Button{
    id: button
    style: ButtonStyle {
        label: Component {
            Text {
                //color: button.checked | button.pressed ? "#ffffff" : "#444444"
                text: button.text
                clip: true
                wrapMode: Text.WordWrap
                font.pointSize: 14
                verticalAlignment: Text.AlignVCenter
                horizontalAlignment: Text.AlignHCenter
                anchors.fill: parent
            }

        }


        background: Rectangle {
            color: button.checked | button.pressed ? "#ff7b00" : "#ffffff"
            implicitWidth: 100
            implicitHeight: 25
            border.width: button.activeFocus ? 2 : 1
            border.color: "#888"
            radius: 4
    }

}

}

