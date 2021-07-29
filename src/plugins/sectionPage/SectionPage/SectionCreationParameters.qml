import QtQuick 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

import "../../Items"
import "../../Commons"
import "../.."

Item {
    implicitHeight: layout.childrenRect.height
    implicitWidth: layout.childrenRect.width


    ColumnLayout{
        id: layout
        anchors.fill: parent

        SkrComboBox {
            id: sectionTypeComboBox
            Layout.fillWidth: true

            wheelEnabled: true
            //visible: SkrSettings.accessibilitySettings.accessibilityEnabled
            model: [
                { value: "book-beginning ", text: qsTr("Beginning of a book") },
                { value: "chapter", text: qsTr("Chapter") },
                { value: "separator", text: qsTr("Separator") },
                { value: "book-end", text: qsTr("End of a book") }
            ]
            textRole: "text"
            valueRole: "value"
            currentIndex: 2

            onCurrentValueChanged: {
                skrTreeManager.setCreationParametersForPageType("SECTION", {"section_type": sectionTypeComboBox.currentValue})
            }
        }

    }

}
