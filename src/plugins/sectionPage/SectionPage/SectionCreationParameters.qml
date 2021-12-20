import QtQuick
import QtQml
import QtQuick.Layouts
import QtQuick.Controls

import "../../Items"
import "../../Commons"
import "../.."

Item {
    implicitHeight: layout.childrenRect.height
    implicitWidth: layout.childrenRect.width


    ColumnLayout{
        id: layout
        anchors.centerIn: parent

        SkrComboBox {
            id: sectionTypeComboBox
            //Layout.fillWidth: true

            wheelEnabled: true
            //visible: SkrSettings.accessibilitySettings.accessibilityEnabled
            model: ListModel {
                ListElement{ value: "book-beginning"; text: qsTr("Beginning of a book") }
                ListElement{ value: "chapter"; text: qsTr("Chapter") }
                ListElement{ value: "separator"; text: qsTr("Separator") }
                ListElement{ value: "book-end"; text: qsTr("End of a book") }
}

            textRole: "text"
            valueRole: "value"
            currentIndex: 2

            onCurrentValueChanged: {
                skrTreeManager.setCreationParametersForPageType("SECTION", {"section_type": sectionTypeComboBox.currentValue})
            }
        }

    }

}
