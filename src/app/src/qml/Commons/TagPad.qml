import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.skrusersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
//import ".."


TagPadForm {
    id: root
    property int minimumHeight: 300


    property int projectId: -2
    property int itemId: -2
    property var tagListModel: undefined

    signal callAddTagRelationship(int projectId, int itemId, string tagName)
    signal callRemoveTagRelationship(int projectId,int itemId, int tagId)



    tagRepeater.model: tagListModel

    // force focus on first child
    tagFlow.activeFocusOnTab: true
    tagFlow.onActiveFocusChanged: {
        if(tagFlow.children.length > 1){// means there is no children
            var first = tagFlow.children[0]
            first.forceActiveFocus()
            first.isSelected = true
            return
        }

    }


    Component {
        id: tagFlowComponent
        Rectangle {
            id: itemBase
            width: childrenRect.width + 10
            height: childrenRect.height + 10
            //color: isOpened ? "cyan" : "lightskyblue"
            border.color: isSelected ? "blue" : "lightskyblue"
            border.width: 1
            radius : height / 2
            property int projectId: model.projectId
            property int itemId: root.itemId
            property int tagId: model.tagId
            property bool isSelected: false


            focus: true

            TapHandler {
                id: tapHandler
                onSingleTapped: {



                }
            }
            //                onDoubleTapped: {
            //                    //reset other tags :
            //                    var i;
            //                    for(i = 0; i < tagRepeater.count; i++) {
            //                        tagRepeater.itemAt(i).isOpened = false
            //                    }

            //                    itemBase.isOpened = true
            //                    Globals.openTagInNewTabCalled(itemBase.projectId, itemBase.tagId)


            //                }
            //            }


            //            TapHandler {
            //                id: shiftTapHandler
            //                acceptedModifiers: Qt.ShiftModifier
            //                onSingleTapped: {
            //                    //reset other tags :
            //                    var i;
            //                    for(i = 0; i < tagRepeater.count; i++) {
            //                        tagRepeater.itemAt(i).isOpened = false
            //                    }
            //                    itemBase.isOpened = true
            //                    Globals.openTagInNewTabCalled(itemBase.projectId, itemBase.tagId)
            //                }
            //            }


            Keys.onPressed: {
                if (event.key === Qt.Key_Delete){
                    console.log("Delete key pressed ")
                    // remove
                    callRemoveTagRelationship(projectId, itemId, tagId)

                }
                if ((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_Delete){
                    console.log("Shift delete key pressed ")
                    // remove completely the tag

                    //TODO: ask confirmation before erasing

                    //plmData.tagHub().removePaperTagRelationship(projectId, itemId, model.itemTagId)

                }

                if (event.key === Qt.Key_Right || event.key === Qt.Key_Down ){

                    itemBase.isSelected = false
                    if(model.index === tagRepeater.count - 1){
                        tagRepeater.itemAt(0).forceActiveFocus()
                        tagRepeater.itemAt(0).isSelected = true

                    }
                    else{
                        tagRepeater.itemAt(model.index + 1).forceActiveFocus()
                        tagRepeater.itemAt(model.index + 1).isSelected = true
                    }

                }
                if (event.key === Qt.Key_Left || event.key === Qt.Key_Up ){

                    itemBase.isSelected = false
                    if(model.index === 0){
                        tagRepeater.itemAt(tagRepeater.count - 1).forceActiveFocus()
                        tagRepeater.itemAt(tagRepeater.count - 1).isSelected = true
                    }
                    else{
                        tagRepeater.itemAt(model.index - 1).forceActiveFocus()
                        tagRepeater.itemAt(model.index - 1).isSelected = true
                    }

                }
            }

            Accessible.role: Accessible.ListItem
            Accessible.name: model.name
            Accessible.description: qsTr("tag related to the current paper")

            RowLayout{
                id: tagLayout
                anchors.left: parent.left
                anchors.top: parent.top

                anchors.margins : 5

                Text{
                    id: tagTitle
                    text: model.name
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignHCenter
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                    Layout.fillWidth: true
                    Layout.fillHeight: true
                }

                RoundButton {
                    id: removeRelationshipButton
                    Layout.preferredWidth: 0
                    Layout.maximumHeight: tagTitle.height
                    padding:1
                    icon.name: "list-remove"
                    onReleased:{
                     callRemoveTagRelationship(projectId, itemId, tagId)
                    }

                }


                HoverHandler {
                    id: hoverHandler

                }
                state: hoverHandler.hovered ? "visible_removeRelationshipButton": ""

                states:[
                    State {
                        name: "visible_removeRelationshipButton"
                        PropertyChanges { target: removeRelationshipButton; Layout.preferredWidth: tagTitle.height}
                    }
                ]

                transitions: [
                    Transition {
                        from: ""
                        to: "visible_removeRelationshipButton"
                        NumberAnimation {target: removeRelationshipButton; property: "Layout.preferredWidth";duration: 300; easing.type: Easing.OutCubic }
                    },
                    Transition {
                        from: "visible_removeRelationshipButton"
                        to: ""
                        NumberAnimation { target: removeRelationshipButton; property: "Layout.preferredWidth";duration: 300; easing.type: Easing.OutCubic }

                    }
                ]
            }

        }
    }





    //--------------------------------------------
    //---------- Add Tag------------------------
    //--------------------------------------------

    Action {
        id: addTagAction
        text: qsTr("Add tag")
        icon.name: "list-add"
        onTriggered: {

            titleEditPopup.open()
        }
    }
    addTagMenuToolButton.action: addTagAction




    //proxy model for search :

    SKRSearchTagListProxyModel {
        id: searchProxyModel
        projectIdFilter: projectId
    }




    Popup {
        id: titleEditPopup
        x: addTagMenuToolButton.x - 200
        y: addTagMenuToolButton.y + addTagMenuToolButton.height
        width: 200
        height: 200
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        padding: 0

        ColumnLayout {
            anchors.fill: parent
            TextField {
                id: titleTextField
                Layout.fillWidth: true

                selectByMouse: true


                onVisibleChanged: {
                    if (visible){
                        titleTextField.text = "test"
                        titleTextField.forceActiveFocus()
                        titleTextField.selectAll()
                    }
                }

                onAccepted: {

                    //create basic tag
                    //var error = plmData.tagHub().addTagRelationship(projectId, itemId)

                    callAddTagRelationship(projectId, itemId, titleTextField.text)

                    // add to model
                    //tagListModel.append({title: title, itemProjectId: projectId, itemPaperId: paperId, itemTagId: tagId})

                    titleEditPopup.close()
                }

                onTextChanged: {
                    searchProxyModel.textFilter = text
                }


            }

            ScrollView {
                id: searchListScrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                ListView {
                    id: searchResultList
                    anchors.fill: parent
                    clip: true
                    smooth: true
                    focus: true
                    boundsBehavior: Flickable.StopAtBounds


                    model: searchProxyModel
                    interactive: true
                    spacing: 1
                    delegate: Component {
                        id: itemDelegate

                        Item {
                            id: delegateRoot
                            height: 30


                            anchors {
                                left: Qt.isQtObject(parent) ? parent.left : undefined
                                right: Qt.isQtObject(parent) ? parent.right : undefined
                                leftMargin: 5
                                rightMargin: 5
                            }

                            TapHandler {
                                id: tapHandler
                                onSingleTapped: {
                                    listView.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
                                }
                            }

                            //                        Shortcut {
                            //                            sequences: ["Return", "Space"]
                            //                            onActivated: {

                            //                                //create relationship with tag

                            //                                var tagId = model.paperId
                            //                                var error = plmData.tagHub().setPaperTagRelationship(model.projectId, paperId, tagId )

                            //                                if (!error.success){
                            //                                    //TODO: add notification
                            //                                    return
                            //                                }

                            //                                titleEditPopup.close()

                            //                            }

                            //                            //enabled: listView.activeFocus
                            //                        }

                            Keys.priority: Keys.BeforeItem

                            Keys.onPressed: {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")

                                    //create relationship with tag

                                    callAddTagRelationship(model.projectId, itemId, model.name)


                                    titleEditPopup.close()


                                }

                            }
                            Text {
                                text: model.name
                                anchors.fill: parent
                                horizontalAlignment: Qt.AlignLeft
                                verticalAlignment: Qt.AlignVCenter
                            }
                        }
                    }

                    highlight:  Component {
                        id: highlight
                        Rectangle {
                            //                            x: 0
                            //                            y: searchResultList.currentItem.y + 1
                            //                            width: searchResultList.width
                            //                            height: searchResultList.currentItem.height - 1
                            //                            color: "transparent"
                            radius: 5
                            border.color:  "lightsteelblue"
                            border.width: 2
                            visible: searchResultList.focus
                            Behavior on y {
                                SpringAnimation {
                                    spring: 3
                                    damping: 0.2
                                }
                            }
                        }
                    }


                    section.property: "projectId"
                    section.criteria: ViewSection.FullString
                    section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                              ViewSection.InlineLabels
                    section.delegate: sectionHeading

                    // The delegate for each section header
                    Component {
                        id: sectionHeading
                        Rectangle {
                            width: searchResultList.width
                            height: childrenRect.height
                            color: "lightsteelblue"

                            required property string section

                            Text {
                                text: qsTr("Existing tags")
                                font.bold: true
                                //font.pixelSize: 20
                            }
                        }
                    }
                }
            }
        }
    }



}

