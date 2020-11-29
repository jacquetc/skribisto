import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml.Models 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.usersettings 1.0
import eu.skribisto.searchtaglistproxymodel 1.0
import eu.skribisto.taghub 1.0
import eu.skribisto.skr 1.0
import "../Items"
import ".."


TagPadForm {
    id: root


    property int projectId: -2
    // we use the term itemId instead of paperId to not be constrained if we want to tag more than papers in the future
    property int itemId: -2
    property var itemType: SKR.Sheet
    property var tagListModel: undefined

    signal callAddTagRelationship(int projectId, int itemId, string tagName, string colorCode, string textColorCode)
    signal callRemoveTagRelationship(int projectId,int itemId, int tagId)
    signal callAddTag(int projectId, string tagName, string colorCode, string textColorCode)
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

        onCountChanged:{
            hideCurrentTagsInAddPopup()
        }


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
    //needed to center vertically in overview tree


    Binding on tagFlickable.contentY {
        value: tagFlow.height < tagFlickable.height ? -(tagFlickable.height / 2  - tagFlow.height / 2) : 0
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }



    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallRemoveTagRelationship(projectId, itemId, tagId){
            if(projectId !== root.projectId){
                return
            }
            plmData.tagHub().removeTagRelationship(projectId, itemType , itemId, tagId)
        }
    }
    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallAddTagRelationship(projectId, itemId, tagName, colorCode, textColorCode){
            if(projectId !== root.projectId){
                return
            }

            var result;
            // verify if name doesn't already exist :
            var tagId = plmData.tagHub().getTagIdWithName(projectId, tagName)

            if(tagId === -2){
                //if not, create tag
                result = plmData.tagHub().addTag(projectId, tagName)
                tagId = plmData.tagHub().getLastAddedId()
            }
            // set color if different
            if(colorCode !== plmData.tagHub().getTagColor(projectId, tagId)){
                plmData.tagHub().setTagColor(projectId, tagId, colorCode)
                plmData.tagHub().setTagTextColor(projectId, tagId, textColorCode)
            }

            // set relationship
            result = plmData.tagHub().setTagRelationship(projectId, itemType, itemId, tagId)
            if (!result.success){
                console.log("result onCallAddTagRelationship")
                //TODO: add notification
                return
            }
        }
    }

    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallAddTag(projectId, tagName, colorCode, textColorCode){
            if(projectId !== root.projectId){
                return
            }
            var result = plmData.tagHub().addTag(projectId, tagName)
            var tagId = result.getData("tagId", -2)

            if(tagId !== -2){
                plmData.tagHub().setTagColor(projectId, tagId, colorCode)
                plmData.tagHub().setTagTextColor(projectId, tagId, textColorCode)
            }
        }
    }

    //----------------------------------------------------------------

    Connections{
        target: root
        function onCallRemoveTag(projectId, tagId){
            if(projectId !== root.projectId){
                return
            }
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
            border.color: isSelected ? SkrTheme.accent :  SkrTheme.buttonBackground
            border.width: 2
            radius : height / 2
            property int projectId: model.projectId
            property int itemId: root.itemId
            property int tagId: model.tagId
            property bool isSelected: false
            property bool isFocused: root.focusedIndex === model.index


            color: model.color


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
                        //console.log(" root.selectedList",  root.selectedList)
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


            Action{
                id: renameAction
                text: qsTr("Rename")
                onTriggered: {

                    if(loader_renamePopup.active){
                        loader_renamePopup.active = false
                        loader_renamePopup.item.close()
                        return
                    }

                    loader_renamePopup.active = true
                    loader_renamePopup.item.tagId = model.tagId
                    loader_renamePopup.item.projectId = projectId
                    loader_renamePopup.item.open()
                }
            }

            SkrMenu{
                id: rightClickMenu

                SkrMenuItem {
                    id: renameMenuItem
                    action: renameAction

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
                if( event.key === Qt.Key_F2){
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

                if (event.key === Qt.Key_F2){
                    renameAction.triggered()
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

                    color: model.textColor

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

            if(loader_addPopup.active){
                loader_addPopup.item.close()
                loader_addPopup.active = false
                return
            }


            loader_addPopup.active = true
            loader_addPopup.item.open()
        }
    }
    addTagMenuToolButton.action: addTagAction


    //---------------------------------------------

    QtObject{
        id: priv
        property var idToHideList: []

    }



    //proxy model for search :

    function  hideCurrentTagsInAddPopup(){

        var idToHideList = []
        for(var i = 0  ; i < visualModel.items.count; i++){

            idToHideList.push(visualModel.items.get(i).model.tagId)
        }

        priv.idToHideList = idToHideList

    }



    Component {
        id: component_addPopup
        SkrPopup {
            property alias titleTextField: inner_titleTextField
            property alias searchListScrollView: inner_searchListScrollView
            property alias searchResultList: inner_searchResultList
            property alias colorChooser: inner_colorChooser


            SKRSearchTagListProxyModel {
                id: searchProxyModel
                projectIdFilter: projectId
                hideTagIdListFilter: priv.idToHideList
            }



            id: addPopup
            x: addTagMenuToolButton.x - 400
            y: addTagMenuToolButton.y + addTagMenuToolButton.height
            width: 400
            height: 400
            modal: false
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
            padding: 0

            onOpened: {
                inner_titleTextField.clear()
                colorChooser.selectRandomColor()


            }
            onClosed: {
                loader_addPopup.active = false
            }

            RowLayout{
                anchors.fill: parent

                ColumnLayout {
                    Layout.fillHeight: true
                    Layout.preferredWidth: addPopup.width / 2

                    ColorChooser {
                        id: inner_colorChooser

                        Layout.fillHeight: true
                        Layout.fillWidth: true


                        onColorCodeChanged: inner_titleTextField.forceActiveFocus()

                    }
                }
                ColumnLayout {
                    Layout.fillHeight: true
                    Layout.preferredWidth: addPopup.width / 2

                    SkrTextField {
                        id: inner_titleTextField
                        Layout.fillWidth: true

                        selectByMouse: true

                        placeholderText: qsTr("Tag name")


                        onVisibleChanged: {
                            if (visible){
                                inner_titleTextField.forceActiveFocus()
                                inner_titleTextField.selectAll()
                            }
                        }

                        onAccepted: {
                            if(itemId === -2){
                                callAddTag(projectId, inner_titleTextField.text, inner_colorChooser.colorCode, inner_colorChooser.textColorCode)
                            }
                            else {
                                callAddTagRelationship(projectId, itemId, inner_titleTextField.text, inner_colorChooser.colorCode, inner_colorChooser.textColorCode)
                            }
                            addPopup.close()
                        }

                        onTextChanged: {
                            searchProxyModel.textFilter = text
                        }


                        Keys.priority: Keys.BeforeItem

                        Keys.onPressed: {
                            if (event.key === Qt.Key_Down){
                                if(inner_searchResultList.count > 0){
                                    inner_searchResultList.itemAtIndex(0).forceActiveFocus()
                                }
                            }

                        }


                    }

                    ScrollView {
                        id: inner_searchListScrollView
                        focusPolicy: Qt.StrongFocus
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded

                        ListView {
                            id: inner_searchResultList
                            smooth: true
                            focus: true
                            boundsBehavior: Flickable.StopAtBounds

                            model: searchProxyModel
                            interactive: true
                            spacing: 1
                            delegate: Component {
                                id: inner_itemDelegate

                                Item {
                                    id: inner_delegateRoot


                                    Accessible.name: model.name
                                    Accessible.role: Accessible.ListItem


                                    height: inner_itemBase.height
                                    width: inner_itemBase.width

                                    anchors {
                                        left: Qt.isQtObject(parent) ? parent.left : undefined
                                        right: Qt.isQtObject(parent) ? parent.right : undefined
                                        leftMargin: 5
                                        rightMargin: 5
                                    }

                                    TapHandler {
                                        id: inner_tapHandler
                                        //                                    onSingleTapped: {
                                        //                                        searchResultList.currentIndex = model.index
                                        //                                        delegateRoot.forceActiveFocus()
                                        //                                        colorChooser.selectColor(model.color)

                                        //                                        eventPoint.accepted = true
                                        //                                    }
                                        onSingleTapped: {

                                            if(itemId === -2){
                                                callAddTag(model.projectId, model.name, model.color, model.textColor)
                                            }
                                            else {
                                                //create relationship with tag
                                                callAddTagRelationship(model.projectId, itemId, model.name, model.color, model.textColor)

                                            }
                                            addPopup.close()
                                            eventPoint.accepted = true
                                        }

                                        onGrabChanged: {
                                            point.accepted = false
                                        }

                                    }

                                    //                        Shortcut {
                                    //                            sequences: ["Return", "Space"]
                                    //                            onActivated: {

                                    //                                //create relationship with tag

                                    //                                var tagId = model.paperId
                                    //                                var result = plmData.tagHub().setPaperTagRelationship(model.projectId, paperId, tagId )

                                    //                                if (!result.success){
                                    //                                    //TODO: add notification
                                    //                                    return
                                    //                                }

                                    //                                addPopup.close()

                                    //                            }

                                    //                            //enabled: listView.activeFocus
                                    //                        }

                                    Keys.onPressed: {
                                        if (event.key === Qt.Key_Return || event.key === Qt.Key_Space){
                                            console.log("Return key pressed title")

                                            //create relationship with tag
                                            callAddTagRelationship(model.projectId, itemId, model.name, model.color, model.textColor)
                                            addPopup.close()


                                            event.accepted = true
                                        }

                                    }

                                    Rectangle {
                                        id: inner_itemBase
                                        width: childrenRect.width + 10
                                        height: childrenRect.height + 10
                                        border.color: inner_searchResultList.currentIndex === model.index ? SkrTheme.accent : SkrTheme.buttonBackground
                                        border.width: 2
                                        radius : height / 2
                                        color: model.color

                                        RowLayout{
                                            id: inner_tagLayout
                                            anchors.left: parent.left
                                            anchors.top: parent.top

                                            anchors.margins : 5
                                            SkrLabel {
                                                text: model.name
                                                horizontalAlignment: Qt.AlignHCenter
                                                verticalAlignment: Qt.AlignHCenter
                                                Layout.minimumWidth: 20
                                                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                                                Layout.fillWidth: true
                                                Layout.fillHeight: true

                                                color: model.textColor
                                            }
                                        }

                                    }
                                }
                            }

                            highlight:  Component {
                                id: inner_highlight
                                Rectangle {

                                    radius: 5
                                    border.color:  SkrTheme.accent
                                    border.width: 2
                                    visible: inner_searchResultList.activeFocus
                                    Behavior on y {
                                        SpringAnimation {
                                            spring: 5
                                            mass: 0.2
                                            damping: 0.2
                                        }
                                    }
                                }
                            }


                            section.property: "projectId"
                            section.criteria: ViewSection.FullString
                            section.labelPositioning: ViewSection.CurrentLabelAtStart |
                                                      ViewSection.InlineLabels
                            section.delegate: inner_sectionHeading

                            // The delegate for each section header
                            Component {
                                id: inner_sectionHeading
                                Rectangle {
                                    width: inner_searchResultList.width
                                    height: childrenRect.height
                                    color: SkrTheme.buttonBackground

                                    required property string section

                                    SkrLabel {
                                        text: qsTr("Existing tags")
                                        font.bold: true
                                        color: SkrTheme.buttonForeground
                                        //font.pixelSize: 20
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }
    }
    Loader {
        id: loader_addPopup
        sourceComponent: component_addPopup
        active: false
    }




    //--------------------------------------------
    //------- Rename note -----------------------
    //--------------------------------------------


    //---------------------------------------------



    Component {
        id: component_renamePopup
        SkrPopup {

            property alias titleTextField: inner_titleTextField
            property int projectId: -2
            property int tagId: -2


            id: renamePopup
            x: addNoteMenuToolButton.x - width
            y: addNoteMenuToolButton.y + addNoteMenuToolButton.height
            width: inner_titleTextField.implicitWidth
            height: inner_titleTextField.implicitHeight
            modal: false
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
            padding: 0

            onOpened: {
                inner_titleTextField.clear()
                var name = plmData.tagHub().getTagName(projectId, tagId)
                inner_titleTextField.text = name
                inner_titleTextField.selectAll()
            }

            onClosed: {
                loader_renamePopup.active = false
            }

            ColumnLayout {
                anchors.fill: parent
                SkrTextField {
                    id: inner_titleTextField
                    Layout.fillWidth: true

                    selectByMouse: true
                    placeholderText: qsTr("Note name")


                    onVisibleChanged: {
                        if (visible){
                            inner_titleTextField.forceActiveFocus()
                            inner_titleTextField.selectAll()
                        }
                    }

                    onAccepted: {
                        if(inner_titleTextField.text.length === 0){
                            return
                        }

                        //create basic note
                        var result = plmData.tagHub().setTagName(renamePopup.projectId, renamePopup.tagId, inner_titleTextField.text)
                        if (!result.success){
                            //TODO: add notification
                            return
                        }


                        renamePopup.close()
                    }

                }


            }
        }
    }
    Loader {
        id: loader_renamePopup
        sourceComponent: component_renamePopup
        active: false
    }

    //--------------------------------------------------------------------------
    //--------------------------------------------------------------------------
    //--------------------------------------------------------------------------



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

