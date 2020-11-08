import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml.Models 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.usersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import "../Items"
import ".."


TagPadForm {
    id: root
    property int minimumHeight: 200 //mandatory for ToolFrame

    property int projectId: -2
    // we use the term itemId instead of paperId to not be constrained if we want to tag more than papers in the future
    property int itemId: -2
    property var itemType: SKRTagHub.Sheet
    property var tagListModel: undefined

    signal callAddTagRelationship(int projectId, int itemId, string tagName)
    signal callRemoveTagRelationship(int projectId,int itemId, int tagId)
    signal callAddTag(int projectId, string tagName)
    signal callRemoveTag(int projectId,int tagId)
    signal tagTapped(int projectId,int tagId)
    signal tagDoubleTapped(int projectId,int tagId)
    signal escapeKeyPressed()


    property int focusedIndex: -2
    property var selectedList: []
    signal selectedListModified(var list)

    function determineWhichItemIsSelected() {

        var count = tagFlow.children.length - 1
        var i
        for (i = 0 ; i < count ; i++ ){
            tagFlow.children[i].determineIfSelected()
        }

        selectedListModified(selectedList)

    }


    // options :
    property bool selectionEnabled: false
    property bool multipleSelectionsEnabled: false

    function clearSelection(){
        selectedList = []
        determineWhichItemIsSelected()
    }

    tagFlowFocusScope.onActiveFocusChanged: {
        if(!tagFlowFocusScope.activeFocus){
            focusedIndex = -2
        }
    }

    DelegateModel {
        id: visualModel
        model: tagListModel
        delegate: tagFlowComponent
    }

    tagRepeater.model: visualModel




    // force focus on first child
    tagFlow.activeFocusOnTab: true
    tagFlow.onActiveFocusChanged: {
        if(tagFlow.children.length > 1){// means there is no children
            var first = tagFlow.children[0]
            first.forceActiveFocus()
            first.setFocused()
            return
        }

    }

    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallRemoveTagRelationship(projectId, itemId, tagId){
            plmData.tagHub().removeTagRelationship(projectId, itemType , itemId, tagId)
        }
    }
    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallAddTagRelationship(projectId, itemId, tagName){

            var error;
            // verify if name doesn't already exist :
            var tagId = plmData.tagHub().getTagIdWithName(projectId, tagName)

            if(tagId === -2){
                //if not, create tag
                error = plmData.tagHub().addTag(projectId, tagName)
                tagId = plmData.tagHub().getLastAddedId()
            }

            // set relationship
            error = plmData.tagHub().setTagRelationship(projectId, itemType, itemId, tagId)
            if (!error.success){
                console.log("error onCallAddTagRelationship")
                //TODO: add notification
                return
            }
        }
    }

    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallAddTag(projectId, tagName){
            plmData.tagHub().addTag(projectId, tagName)
        }
    }

    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallRemoveTag(projectId, tagId){
            plmData.tagHub().removeTag(projectId, tagId)
        }
    }
    //----------------------------------------------------------------

    Component {
        id: tagFlowComponent
        Rectangle {
            id: itemBase
            width: childrenRect.width + 10
            height: childrenRect.height + 10
            border.color: isSelected ? SkrTheme.accent : "lightskyblue"
            border.width: 2
            radius : height / 2
            property int projectId: model.projectId
            property int itemId: root.itemId
            property int tagId: model.tagId
            property bool isSelected: false
            property bool isFocused: root.focusedIndex === model.index

            //TODO: adapt to tag color
            //temporary:
            color: SkrTheme.buttonBackground


            focus: true
//            onActiveFocusChanged: {
//                if(activeFocus){
//                    setFocused()
//                }
////                else {
////                    root.focusedIndex = -2
////                }
//            }

            function setFocused(){
                root.focusedIndex = model.index
            }

             function setSelected(){
                if(!multipleSelectionsEnabled && selectionEnabled){
                    root.selectedList = []
                }

                if(selectionEnabled){

                    var here = false
                    for(var i = 0; i < root.selectedList.length ; i ++){

                        if(root.selectedList[i] === itemBase.tagId){
                            here = true
                        }
                    }

                    if(!here){
                        root.selectedList.push(itemBase.tagId)
                        console.log(" root.selectedList",  root.selectedList)
                        root.determineWhichItemIsSelected()
                    }
                }

            }

             function setDeselected(){


                 var index = root.selectedList.indexOf(itemBase.tagId)
                 if(index === -1){
                     return
                 }

                 root.selectedList.splice(index, 1)
                 root.determineWhichItemIsSelected()


             }


            function toggleSelected(){
                if(itemBase.isSelected){
                    itemBase.setDeselected()

                }
                else{
                    itemBase.setSelected()
                }

            }


            function determineIfSelected(){

                var here = false
                for(var i = 0; i < root.selectedList.length ; i++){

                    if(root.selectedList[i] === itemBase.tagId){
                        here = true
                    }
                }

               itemBase.isSelected = here
            }

            TapHandler {
                id: tapHandler
                acceptedButtons: Qt.LeftButton
                onSingleTapped: {

                    itemBase.setFocused()
                    itemBase.toggleSelected()
                    itemBase.forceActiveFocus()
                    tagTapped(projectId, tagId)

                    eventPoint.accepted = true

                }
                onDoubleTapped: {
                    itemBase.setFocused()
                    itemBase.setSelected()
                    itemBase.forceActiveFocus()
                    tagDoubleTapped(projectId, tagId)

                    eventPoint.accepted = true
                }
                onLongPressed: {
                    itemBase.setFocused()
                    itemBase.forceActiveFocus()


                    if(rightClickMenu.visible){
                        rightClickMenu.close()
                        eventPoint.accepted = true
                        return
                    }

                    rightClickMenu.popup(itemBase, 0, itemBase.height)
                    eventPoint.accepted = true
                }
            }




            TapHandler {
                id: rightClickHandler
                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                acceptedButtons: Qt.RightButton
                onSingleTapped: {

                    itemBase.setFocused()
                    itemBase.forceActiveFocus()


                    if(rightClickMenu.visible){
                        rightClickMenu.close()
                        eventPoint.accepted = true
                        return
                    }

                    rightClickMenu.popup()

                    eventPoint.accepted = true

                }
            }

            SkrMenu{
                id: rightClickMenu

                SkrMenuItem {
                    id: renameMenuItem
                    text: qsTr("Rename")

                    onTriggered: {

                    }
                }


                SkrMenuItem {
                    id: addTagMenuItem
                    text: qsTr("Add")
                    action: addTagAction


                }
                SkrMenuItem {
                    id: removeTagMenuItem
                    text: qsTr("Remove")

                    onTriggered: {
                        if(itemId === -2){
                            removeTagDialog.projectId = projectId
                            removeTagDialog.tagId = tagId
                            removeTagDialog.tagName = model.name
                            removeTagDialog.open()
                        }
                        else {
                            callRemoveTagRelationship(projectId, itemId, tagId)
                        }
                    }

                }

            }

            SimpleDialog {
                property int projectId: -2
                property int tagId: -2
                property string tagName: ""
                property string relatedItemNames: ""

                id: removeTagDialog
                title: qsTr("Warning")
                text: qsTr("Do you want to delete the tag \"%1\" ?\n%2").arg(tagName).arg(relatedItemNames)
                standardButtons: Dialog.Yes  | Dialog.Cancel

                onOpened: {
                    var list = plmData.tagHub().getItemIdsFromTag(projectId, tagId, true)

                    var separator
                    var i
                    for (i = 0 ; i < list.length ; i++){


                        if(list[i] === -30){
                            relatedItemNames += "Sheets :\n"
                            separator = -30
                            continue
                        }


                        if(list[i] === -31){
                            relatedItemNames += "Notes :\n"
                            separator = -31
                            continue
                        }


                        if(separator === -30){
                            relatedItemNames += "- " + plmData.sheetHub().getTitle(projectId, list[i]) + "\n"
                        }
                        else if(separator === -31){
                            relatedItemNames += "- " + plmData.noteHub().getTitle(projectId, list[i]) + "\n"
                        }

                    }
                }

                onRejected: {

                }

                onAccepted: {
                    callRemoveTag(projectId, tagId)

                }
            }

            Keys.onShortcutOverride: {
                if( event.key === Qt.Key_Escape){
                    event.accepted = true
                }
            }

            Keys.onPressed: {
                if (event.key === Qt.Key_Delete){
                    console.log("Delete key pressed ")

                    if(itemId === -2){
                        // remove tag
                        removeTagDialog.projectId = projectId
                        removeTagDialog.tagId = tagId
                        removeTagDialog.tagName = model.name
                        removeTagDialog.open()
                    }
                    else {
                        // remove relationship
                        callRemoveTagRelationship(projectId, itemId, tagId)
                    }
                }
                if ((event.modifiers & Qt.ShiftModifier) && event.key === Qt.Key_Delete){
                    console.log("Shift delete key pressed ")
                    // remove completely the tag

                    //TODO: ask confirmation before erasing

                    //plmData.tagHub().removePaperTagRelationship(projectId, itemId, model.itemTagId)

                }

                if (event.key === Qt.Key_Space){
                    itemBase.toggleSelected()
                }

                if (event.key === Qt.Key_Escape){
                    escapeKeyPressed()
                }

                if (event.key === Qt.Key_Right || event.key === Qt.Key_Down ){

                    if(model.index === tagRepeater.count - 1){
                        tagRepeater.itemAt(0).forceActiveFocus()
                        tagRepeater.itemAt(0).setFocused()

                    }
                    else{
                        tagRepeater.itemAt(model.index + 1).forceActiveFocus()
                        tagRepeater.itemAt(model.index + 1).setFocused()
                    }

                }
                if (event.key === Qt.Key_Left || event.key === Qt.Key_Up ){

                    if(model.index === 0){
                        tagRepeater.itemAt(tagRepeater.count - 1).forceActiveFocus()
                        tagRepeater.itemAt(tagRepeater.count - 1).setFocused()
                    }
                    else{
                        tagRepeater.itemAt(model.index - 1).forceActiveFocus()
                        tagRepeater.itemAt(model.index - 1).setFocused()
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

                SkrLabel{
                    id: tagTitle
                    text: model.name
                    font.bold: itemBase.isFocused

                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignHCenter
                    Layout.minimumWidth: 20
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    color: SkrTheme.buttonForeground

                    SkrRoundButton {
                        id: removeRelationshipButton
                        width: 0
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        anchors.right: parent.right
                        padding:0
                        topInset: 1
                        bottomInset: 1
                        leftInset: 1
                        rightInset: 1
                        opacity: 0
                        icon.source: "qrc:///icons/backup/list-remove.svg"
                        onReleased:{
                            if(itemId === -2){

                                removeTagDialog.projectId = projectId
                                removeTagDialog.tagId = tagId
                                removeTagDialog.tagName = model.name
                                removeTagDialog.open()
                            }
                            else {
                                callRemoveTagRelationship(projectId, itemId, tagId)

                            }
                        }
                        activeFocusOnTab: false
                        focusPolicy: Qt.NoFocus


                    }
                }


                HoverHandler {
                    id: hoverHandler

                    onHoveredChanged: {
                        if(hovered){
                            showRemoveRelationshipButtonTimer.start()
                        }
                    }
                }

                Timer{
                    id: showRemoveRelationshipButtonTimer
                    repeat: false
                    interval: 1000

                }

                state: hoverHandler.hovered && !showRemoveRelationshipButtonTimer.running ? "visible_removeRelationshipButton": ""

                states:[
                    State {

                        name: "visible_removeRelationshipButton"
                        PropertyChanges { target: removeRelationshipButton; width: tagTitle.height}
                        PropertyChanges { target: removeRelationshipButton; opacity: 1.0}
                    }
                ]

                transitions: [
                    Transition {
                        from: ""
                        to: "visible_removeRelationshipButton"
                        reversible: true
                        ParallelAnimation {
                            NumberAnimation {target: removeRelationshipButton; property: "opacity";duration: 250; easing.type: Easing.OutCubic }
                            NumberAnimation {target: removeRelationshipButton; property: "width";duration: 250; easing.type: Easing.OutCubic }
                        }
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
        icon.source: "qrc:///icons/backup/list-add.svg"
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




    SkrPopup {
        id: titleEditPopup
        x: addTagMenuToolButton.x - 200
        y: addTagMenuToolButton.y + addTagMenuToolButton.height
        width: 200
        height: 400
        modal: false
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        padding: 0

        ColumnLayout {
            anchors.fill: parent
            SkrTextField {
                id: titleTextField
                Layout.fillWidth: true

                selectByMouse: true

                placeholderText: qsTr("Tag name")


                onVisibleChanged: {
                    if (visible){
                        titleTextField.forceActiveFocus()
                        titleTextField.selectAll()
                    }
                }

                onAccepted: {
                    if(itemId === -2){
                        callAddTag(projectId, titleTextField.text)
                    }
                    else {
                        callAddTagRelationship(projectId, itemId, titleTextField.text)
                    }
                    titleEditPopup.close()
                }

                onTextChanged: {
                    searchProxyModel.textFilter = text
                }


                Keys.priority: Keys.BeforeItem

                Keys.onPressed: {
                    if (event.key === Qt.Key_Down){
                        if(searchResultList.count > 0){
                            searchResultList.itemAtIndex(0).forceActiveFocus()
                        }
                    }

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
                                    searchResultList.currentIndex = model.index
                                    delegateRoot.forceActiveFocus()
                                    eventPoint.accepted = true
                                }
                                onDoubleTapped: {

                                    if(itemId === -2){
                                        callAddTag(model.projectId, model.name)
                                    }
                                    else {
                                        //create relationship with tag
                                        callAddTagRelationship(model.projectId, itemId, model.name)

                                    }
                                    titleEditPopup.close()
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

                            Keys.onPressed: {
                                if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                    console.log("Return key pressed title")

                                    //create relationship with tag
                                    callAddTagRelationship(model.projectId, itemId, model.name)
                                    titleEditPopup.close()


                                    event.accepted = true
                                }

                            }
                            SkrLabel {
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

                            radius: 5
                            border.color:  "lightsteelblue"
                            border.width: 2
                            visible: searchResultList.activeFocus
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

                            SkrLabel {
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




    onActiveFocusChanged: {
        if (activeFocus) {


            if(minimalMode){
                tagFlow.forceActiveFocus()

            }
            else{
                addTagMenuToolButton.forceActiveFocus()
            }
        }
    }
}

