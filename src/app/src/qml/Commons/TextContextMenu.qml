import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

TextContextMenuForm {
    id: root


    signal suggestionChosen(string original, string suggestion)
    signal suggestionToBeLearned(string word)


    stackView.initialItem: editPanelComponent

    Component.onCompleted: {
        skrEditMenuSignalHub.subscribe(root.objectName)
    }

    property int currentIndex: 0
    onCurrentIndexChanged: {


        switch(currentIndex){
        case 0:
            showEditPanel()
            break
        case 1:
            showDictPanel()
            break
        }


    }

    onOpened: {
        if(currentIndex === 0){
            showEditPanel()
        }

        forceActiveFocus()
    }

    onClosed: {
        suggestionList = []
        currentIndex = 0
    }



    function showEditPanel(){
        editToolButton.checked = true
    }
    editToolButton.onCheckedChanged: {
        if(editToolButton.checked){
            stackView.pop()
            currentIndex = 0
            dictToolButton.checked = false
        }
    }
    editToolButton.onHoveredChanged: {
        if(editToolButton.hovered){
            editToolButton.checked = true
        }
    }



    function showDictPanel(){
        dictToolButton.checked = true
    }
    dictToolButton.onCheckedChanged: {
        if(dictToolButton.checked){
            showDictPanel()
            currentIndex = 1
            stackView.push(dictPanelComponent)
            editToolButton.checked = false
        }
    }
    dictToolButton.onHoveredChanged: {
        if(dictToolButton.hovered){
            dictToolButton.checked = true
        }


    }

    property var suggestionList: []
    property string suggestionOriginalWord: ""

    QtObject{
        id: priv
        property int buttonHeight: 40

    }

    Component {
        id: editPanelComponent

        SkrPane{
            id: editPane
            padding: 1

            onActiveFocusChanged: {
                if(activeFocus){
                    italicToolButton.forceActiveFocus()
                }

            }


            Component.onCompleted:{
                skrEditMenuSignalHub.subscribe(italicToolButton.objectName)
                skrEditMenuSignalHub.subscribe(boldToolButton.objectName)
                skrEditMenuSignalHub.subscribe(strikeToolButton.objectName)
                skrEditMenuSignalHub.subscribe(underlineToolButton.objectName)
                skrEditMenuSignalHub.subscribe(cutToolButton.objectName)
                skrEditMenuSignalHub.subscribe(copyToolButton.objectName)
                skrEditMenuSignalHub.subscribe(pasteToolButton.objectName)


            }

            ColumnLayout{
                anchors.fill: parent

                GridLayout{
                    columns: 4

                    SkrToolButton{
                        id: italicToolButton
                        action: italicAction
                        objectName: "italicItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }
                    SkrToolButton{
                        id: boldToolButton
                        action: boldAction
                        objectName: "boldItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }
                    SkrToolButton{
                        id: strikeToolButton
                        action: strikeAction
                        objectName: "strikeItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }

                    SkrToolButton {
                        id: underlineToolButton
                        action: underlineAction
                        objectName: "underlineItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }

                    }
                }


                GridLayout{
                    columns: 4

                    SkrToolButton{
                        id: cutToolButton
                        action: cutTextAction
                        objectName: "cutItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                    }
                    SkrToolButton {
                        id: copyToolButton
                        action: copyTextAction
                        objectName: "copyItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                    }
                    SkrToolButton{
                        id: pasteToolButton
                        action: pasteTextAction
                        objectName: "pasteItem"
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly
                    }


                }
                GridLayout{
                    columns: 4

                    SkrToolButton{
                        id: fullScreenToolButton
                        action: fullscreenAction
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }
                    SkrToolButton {
                        id: themesToolButton
                        action: themesColorAction
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }
                    SkrToolButton{
                        id: checkSpellingToolButton
                        action: checkSpellingAction
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }
                    SkrToolButton{
                        id: centerTextCursorToolButton
                        action: centerTextCursorAction
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.preferredHeight: priv.buttonHeight
                        Layout.preferredWidth: priv.buttonHeight
                        display: AbstractButton.IconOnly

                        onToggled: {
                            root.close()
                        }
                    }

                }

                Item{
                    id: stretcher
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                }
            }

        }

    }


    Component {
        id: dictPanelComponent
        SkrPane{
            id: dictPane
            //property var suggestionList: root.suggestionList
            padding: 1
            onActiveFocusChanged: {
                if(activeFocus){
                    suggestionListView.forceActiveFocus()
                }

            }

            implicitHeight: 200
            implicitWidth: 200

            ColumnLayout{
                anchors.fill: parent

                ListView{
                    id: suggestionListView
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    model: root.suggestionList


//                    onModelChanged: {
//                        console.log("model:", model)
//                    }

                    clip: true
                    smooth: true
                    focus: true
                    boundsBehavior: Flickable.StopAtBounds

                    interactive: true
                    spacing: 1

                    onActiveFocusChanged: {
                        if(activeFocus){
                            if(suggestionListView.count > 0){
                                suggestionListView.itemAtIndex(0).forceActiveFocus()
                            }
                        }

                    }



                    delegate: Component {
                        id: itemDelegate

                        SkrListItemPane {
                            id: delegateRoot
                            height: 30
                            focus: true


                            Accessible.name: model.modelData
                            Accessible.role: Accessible.ListItem

                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                leftMargin: 5
                                rightMargin: 5
                            }

                            TapHandler {
                                id: tapHandler
                                //                                onSingleTapped: function(eventPoint) {
                                //                                    searchResultList.currentIndex = model.index
                                //                                    delegateRoot.forceActiveFocus()
                                //                                    eventPoint.accepted = true
                                //                                }
                                onSingleTapped: function(eventPoint) {
                                    //create relationship with note

                                    root.suggestionChosen(root.suggestionOriginalWord, model.modelData)
                                    root.close()

                                    eventPoint.accepted = true
                                }


                                onGrabChanged: function(transition, point) {
                                    point.accepted = false
                                }


                            }


                            HoverHandler {
                                id: hoverHandler

                                onHoveredChanged: {
                                    if(hovered){
                                        suggestionListView.currentIndex = model.index
                                        delegateRoot.forceActiveFocus()
                                    }
                                }
                            }



                            Keys.onPressed: function(event) {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")


                                    root.suggestionChosen(root.suggestionOriginalWord, model.modelData)
                                    root.close()

                                    event.accepted = true

                                }




                            }




                            SkrLabel {
                                text: model.modelData
                                anchors.fill: parent
                                horizontalAlignment: Qt.AlignLeft
                                verticalAlignment: Qt.AlignVCenter
                            }
                        }
                    }

                    highlight:  Component {
                        id: highlight
                        Rectangle {
                            radius: 5
                            border.color:   SkrTheme.accent
                            border.width: 2
                            visible: suggestionListView.activeFocus
                            Behavior on y {
                                enabled: SkrSettings.ePaperSettings.animationEnabled
                                SpringAnimation {
                                    spring: 5
                                    mass: 0.2
                                    damping: 0.2
                                }
                            }
                        }
                    }


                }


                SkrToolButton {
                    id: addWordButton
                    Layout.fillWidth: true


                    text: qsTr("Learn \"%1\"").arg(root.suggestionOriginalWord)

                    onClicked: {
                        root.suggestionToBeLearned(root.suggestionOriginalWord)

                        root.close()
                    }

                    contentItem: SkrLabel {
                        text: addWordButton.text

                    }
                }

            }


        }
    }

    onActiveFocusChanged: {
        if (activeFocus) {


            stackView.currentItem.forceActiveFocus()
        }
    }

}
