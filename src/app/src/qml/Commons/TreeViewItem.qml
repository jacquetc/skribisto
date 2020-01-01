import QtQuick 2.12
import QtQuick.Controls 2.14

TreeViewItemForm {

    menuButton.onClicked: {
        console.log("clicked")
        menu.open()
    }
    Menu {
        id: menu
        y: menuButton.height
        Action {
            text: "Cut"
        }
        Action {
            text: "Copy"
        }
        Action {
            text: "Paste"
        }

        MenuSeparator {}

        Menu {
            title: "Find/Replace"
            Action {
                text: "Find Next"
            }
            Action {
                text: "Find Previous"
            }
            Action {
                text: "Replace"
            }
        }
    }

    ListView.onRemove: SequentialAnimation {
        PropertyAction {
            target: baseDelegateItem
            property: "ListView.delayRemove"
            value: true
        }
        NumberAnimation {
            target: baseDelegateItem
            property: "height"
            to: 0
            easing.type: Easing.InOutQuad
        }
        PropertyAction {
            target: baseDelegateItem
            property: "ListView.delayRemove"
            value: false
        }
    }
}
