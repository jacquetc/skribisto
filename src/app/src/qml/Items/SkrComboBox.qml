import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

ComboBox {
    id: control

    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent



//    Popup {
//             y: control.height - 1
//             width: control.width
//             implicitHeight: contentItem.implicitHeight
//             padding: 1

//             contentItem: ListView {
//                 clip: true
//                 implicitHeight: contentHeight
//                 model: control.popup.visible ? control.delegateModel : null
//                 currentIndex: control.highlightedIndex

//                 ScrollIndicator.vertical: ScrollIndicator { }
//             }

//             background: Rectangle {
//                 border.color: "#21be2b"
//                 radius: 2
//             }
//         }
}

