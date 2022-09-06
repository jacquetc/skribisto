import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import SkrControls



Dialog {
    id: dialog

    anchors.centerIn: Overlay.overlay

    property int projectId: -1
    property int treeItemId: -1

    background: Rectangle {
        color: SkrTheme.menuBackground
    }

    Material.background: SkrTheme.menuBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    modal: true
    implicitWidth: Math.max(Math.max(contentItem.implicitWidth, footer.implicitWidth), header.implicitWidth) + 30

    header: SkrLabel {
        id: headerLabel
        visible: true
        text: "Set a goal"
        font.bold: true
        font.pointSize: Qt.application.font.pointSize * 1.2
        horizontalAlignment: Qt.AlignHCenter
        verticalAlignment: Qt.AlignVCenter
    }

    contentItem: ColumnLayout {

        SkrLabel {
            id: contentLabel
            Layout.alignment: Qt.AlignCenter
            wrapMode: Text.WordWrap
            text: SkrSettings.interfaceSettings.wordCountVisible ? qsTr("Please set the number of words to reach for this item:") : qsTr("Please set the number of characters to reach for this item:")
        }
        SkrTextField {
            id: spinBox
            Layout.alignment: Qt.AlignCenter
            inputMethodHints: Qt.ImhDigitsOnly | Qt.ImhPreferNumbers | Qt.ImhExclusiveInputMask
            selectByMouse: true
            maximumLength: 7
            inputMask: "0000000"

            Keys.onReturnPressed: dialog.accept()
            Keys.onEnterPressed: dialog.accept()
            Keys.onEscapePressed: dialog.reject()

        }

        //TODO: add quick values

    }


    standardButtons: Dialog.Ok | Dialog.Cancel


    onAccepted: {
        var propertyId = "-2"
        if(spinBox.text === "0" || spinBox.text === ""){
            if(SkrSettings.interfaceSettings.wordCountVisible){
               propertyId = skrData.treePropertyHub().findPropertyId(dialog.projectId, dialog.treeItemId, "word_count_goal")
            }
            else {
                propertyId = skrData.treePropertyHub().findPropertyId(dialog.projectId, dialog.treeItemId, "char_count_goal")
            }
            if(propertyId !== "-2"){
                skrData.treePropertyHub.removeProperty(dialog.projectId, propertyId)
            }
        }
        else {

            if(SkrSettings.interfaceSettings.wordCountVisible){
               skrData.treePropertyHub().setProperty(dialog.projectId, dialog.treeItemId, "word_count_goal", spinBox.text, true, false, true)
            }
            else {
                skrData.treePropertyHub().setProperty(dialog.projectId, dialog.treeItemId, "char_count_goal", spinBox.text, true, false, true)
            }
        }



    }

    onAboutToShow: {
        var propertyValue = "0"
        if(SkrSettings.interfaceSettings.wordCountVisible){
            propertyValue = skrData.treePropertyHub().getProperty(dialog.projectId, dialog.treeItemId, "word_count_goal", "0")
        }
        else {
            propertyValue = skrData.treePropertyHub().getProperty(dialog.projectId, dialog.treeItemId, "char_count_goal", "0")
        }

        if(propertyValue === "0"){
            spinBox.text = "2000"
        }
        else {
            spinBox.text = propertyValue
        }
        spinBox.forceActiveFocus()
        spinBox.selectAll()
    }

    onRejected: {
        var propertyValue = "0"
        if(SkrSettings.interfaceSettings.wordCountVisible){
            propertyValue = skrData.treePropertyHub().getProperty(dialog.projectId, dialog.treeItemId, "word_count_goal", "0")
        }
        else {
            propertyValue = skrData.treePropertyHub().getProperty(dialog.projectId, dialog.treeItemId, "char_count_count", "0")
        }

        if(propertyValue === "0"){
            spinBox.text = "2000"
        }
        else {
            spinBox.text = propertyValue
        }
    }


    TapHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus | PointerDevice.TouchScreen
        grabPermissions: PointerHandler.CanTakeOverFromAnything

        onGrabChanged: function(transition, point) {
            point.accepted = true
        }
    }

    onActiveFocusChanged: {
        if(activeFocus){
            spinBox.forceActiveFocus()
        }
    }



}
