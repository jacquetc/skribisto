import QtQuick
import QtQuick.Controls
import Qt.labs.settings 1.1
import QtQml
import Qt.labs.platform 1.1 as LabPlatform
import eu.skribisto.searchtreelistproxymodel 1.0
import eu.skribisto.exporter 1.0

ExporterPageForm {
    id: root


    property bool printEnabled: false
    property int projectId: -2



    Connections {
        target: skrData.projectHub()
        function onActiveProjectChanged(projectId){
            root.projectId = projectId
            treeProxyModel.projectIdFilter = projectId
            treeProxyModel.checkAllButNonPrintable()
            projectLabel.text = skrData.projectHub().getProjectName(projectId)
        }
    }

    Component.onCompleted: {
        var activeProjectId = skrData.projectHub().getActiveProject()
        root.projectId = activeProjectId
        treeProxyModel.projectIdFilter = activeProjectId
        treeProxyModel.checkAllButNonPrintable()
        projectLabel.text = skrData.projectHub().getProjectName(activeProjectId)

        initTypes()
        loadFontFamily()
    }

    Component.onDestruction: {
        saveType()
    }

    Settings{
        id: settings
        category: "exporter"
        property string type: "odt"
        property bool includeSynopsis: false
        property int indentWithTitle: 1
        property int textPointSize: Qt.application.font.pointSize
        property string textFontFamily: skrRootItem.defaultFontFamily()
        property real textIndent: 2
        property real textTopMargin: 2
        property url folder
        property bool tagsEnabled: false
        property bool numbered: false
    }


    //----------------------------------------------------------------------------
    //-------Items  -------------------------------------------------------------
    //----------------------------------------------------------------------------

    SKRSearchTreeListProxyModel {
        id: treeProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
    }

    tree.model: treeProxyModel

    Action {
        id: selectAllTreeItemsAction
        text: selectAllTreeItemsAction.checked ? qsTr("Select none") : qsTr("Select all")
        //enabled: navigationMenu.opened
        //shortcut: "Ctrl+Shift+Del"
        icon.source: selectAllTreeItemsAction.checked ? "qrc:///icons/backup/edit-select-none.svg" : "qrc:///icons/backup/edit-select-all.svg"
        checkable: true
        onCheckedChanged: {

            if(selectAllTreeItemsAction.checked){
                treeProxyModel.checkAll()
            }
            else {
                treeProxyModel.checkNone()

            }

        }



    }

    selectAllTreeItemsToolButton.action: selectAllTreeItemsAction



    //----------------------------------------------------------------------------
    //---------- Types --------------------------------------------------------
    //----------------------------------------------------------------------------


    function initTypes(){
        switch(settings.type){
        case "odt":
            odtTypeAction.checked = true
            break;
        case "txt":
            txtTypeAction.checked = true
            break;
        case "md":
            mdTypeAction.checked = true
            break;
        case "html":
            htmlTypeAction.checked = true
            break;
        case "pdf":
            pdfTypeAction.checked = true
            break;
        }
    }

    function saveType(){
        if(odtTypeAction.checked){
            settings.type = "odt"
        }
        else if(txtTypeAction.checked){
            settings.type = "txt"
        }
        else if(mdTypeAction.checked){
            settings.type = "md"
        }
        else if(htmlTypeAction.checked){
            settings.type = "html"
        }
        else if(pdfTypeAction.checked){
            settings.type = "pdf"
        }
    }

    Action {
        id: odtTypeAction
        text: qsTr(".odt")
        checkable: true
        ActionGroup.group: typeActionGroup
        onCheckedChanged: {
            if(checked){
                settings.type = "odt"
                exporter.outputType = SKRExporter.Odt
            }

        }
    }

    odtButton.action: odtTypeAction


    Action {
        id: txtTypeAction
        text: qsTr(".txt")
        checkable: true
        ActionGroup.group: typeActionGroup
        onCheckedChanged: {
            if(checked){
                settings.type = "txt"
                exporter.outputType = SKRExporter.Txt
            }
        }
    }

    txtButton.action: txtTypeAction


    Action {
        id: mdTypeAction
        text: qsTr(".md")
        checkable: true
        ActionGroup.group: typeActionGroup
        onCheckedChanged: {
            if(checked){
                settings.type = "md"
                exporter.outputType = SKRExporter.Md
            }
        }
    }

    mdButton.action: mdTypeAction


    Action {
        id: htmlTypeAction
        text: qsTr(".html")
        checkable: true
        ActionGroup.group: typeActionGroup
        onCheckedChanged: {
            if(checked){
                settings.type = "html"
                exporter.outputType = SKRExporter.Html
            }
        }
    }

    htmlButton.action: htmlTypeAction


    Action {
        id: pdfTypeAction
        text: qsTr(".pdf")
        checkable: true
        ActionGroup.group: typeActionGroup
        onCheckedChanged: {
            if(checked){
                settings.type = "pdf"
                exporter.outputType = SKRExporter.Pdf
            }
        }
    }

    pdfButton.action: pdfTypeAction




    ActionGroup{
        id: typeActionGroup
        exclusive: true

        onTriggered: {

            if(!action.checked){
                action.checked = true
            }

        }
    }



    //----------------------------------------------------------------------------
    //-------Options--------------------------------------------------
    //----------------------------------------------------------------------------


    includeSynopsisSwitch.checked: settings.includeSynopsis
    Binding {
        target: settings
        property: "includeSynopsis"
        value: includeSynopsisSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }


    tagsSwitch.checked: settings.tagsEnabled
    Binding {
        target: settings
        property: "tagsEnabled"
        value: tagsSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    numberedSwitch.checked: settings.numbered
    Binding {
        target: settings
        property: "numbered"
        value: numberedSwitch.checked
        restoreMode: Binding.RestoreBindingOrValue
    }

    indentWithTitleLabel.text: qsTr("Levels with titles: %1").arg(indentWithTitleSlider.value)

    indentWithTitleSlider.value: settings.indentWithTitle
    Binding {
        target: settings
        property: "indentWithTitle"
        value: indentWithTitleSlider.value
        restoreMode: Binding.RestoreBindingOrValue
    }


    // textPointSizeSlider :

    textPointSizeSlider.value: settings.textPointSize


    Binding {
        target: settings
        property: "textPointSize"
        value: textPointSizeSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Font family combo :
    fontFamilyComboBox.model: skrFonts.fontFamilies()

    Binding {
        target: settings
        property: "textFontFamily"
        value: fontFamilyComboBox.currentText
        when:  fontFamilyLoaded
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    property bool fontFamilyLoaded: false

    function loadFontFamily(){
        var fontFamily = settings.textFontFamily
        console.log("fontFamily", fontFamily)
        console.log("application fontFamily", skrRootItem.defaultFontFamily())

        var index = fontFamilyComboBox.find(fontFamily, Qt.MatchFixedString)
        //console.log("index", index)
        if(index === -1){
            index = fontFamilyComboBox.find("Liberation Serif", Qt.MatchFixedString)
        }
        if(index === -1){
            index = fontFamilyComboBox.find(skrRootItem.defaultFontFamily(), Qt.MatchContains)
        }
        //console.log("index", index)

        fontFamilyComboBox.currentIndex = index
        fontFamilyLoaded = true
    }

    // Indent :
    textIndentSlider.value: settings.textIndent

    Binding {
        target: settings
        property: "textIndent"
        value: textIndentSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }

    // Margins :
    textTopMarginSlider.value: settings.textTopMargin

    Binding {
        target: settings
        property: "textTopMargin"
        value: textTopMarginSlider.value
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }



    //----------------------------------------------------------------------------
    //-------Select file--------------------------------------------------
    //----------------------------------------------------------------------------

    property url targetFile

    selectFileToolButton.onClicked: {
        var activeProjectId = skrData.projectHub().getActiveProject()
        exportFileDialog.projectId = activeProjectId

        exportFileDialog.projectName = skrData.projectHub().getProjectName(activeProjectId)
        exportFileDialog.type = settings.type
        exportFileDialog.open()
    }


    LabPlatform.FileDialog{
        property int projectId: -2
        property string projectName: ""
        property string type: ""

        id: exportFileDialog
        title: qsTr("Export \"%1\" project as …").arg(projectName)
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)
        fileMode: LabPlatform.FileDialog.SaveFile
        selectedNameFilter.index: 0
        nameFilters: ["*.%1".arg(type)]
        onAccepted: {

            var file = exportFileDialog.file.toString()

            if(file.indexOf(".%1".arg(type)) === -1){ // not found
                file = file + ".%1".arg(type)
            }
            if(projectId === -2){
                return
            }


            var fileUrl = Qt.resolvedUrl(file)


            fileTextField.text = skrQMLTools.translateURLToLocalFile(fileUrl)


        }
        onRejected: {

        }
    }


    fileTextField.onTextChanged: {
        targetFile = skrQMLTools.getURLFromLocalFile(fileTextField.text)


    }

    //----------------------------------------------------------------------------
    //--------Export -------------------------------------------------
    //----------------------------------------------------------------------------

    SKRExporter{
        id: exporter
        projectId: root.projectId
        outputUrl: targetFile
        includeSynopsis: includeSynopsisSwitch.checked
        numbered: numberedSwitch.checked
        tagsEnabled: tagsSwitch.checked
        indentWithTitle: indentWithTitleSlider.value
        fontFamily: fontFamilyComboBox.currentText
        fontPointSize: textPointSizeSlider.value
        textIndent: textIndentSlider.value
        textTopMargin: textTopMarginSlider.value
    }

    Connections{
        target: exporter
        function onProgressMaximumChanged(value){
            progressBar.to = value
        }
    }
    Connections{
        target: exporter
        function onProgressChanged(value){
            progressBar.value += value
        }
    }




    previewButton.onClicked: {
        if(projectId === -2 || (fileTextField.text.length === 0 && !printEnabled)){
            return
        }


        exporter.outputType = SKRExporter.PrinterPreview
        exporter.treeItemIdList = treeProxyModel.getCheckedIdsList()
        exporter.run()

    }

    exportProjectButton.onClicked: {
        if(projectId === -2 || (fileTextField.text.length === 0 && !printEnabled)){
            return
        }

        if(printEnabled){
            exporter.outputType = SKRExporter.Printer
        }

        exporter.treeItemIdList = treeProxyModel.getCheckedIdsList()
        exporter.run()

        var ok = Qt.openUrlExternally(targetFile)
        //console.log("ok", ok)
    }


    //----------------------------------------------------------------------------
    //------------------------------------------------------------------
    //----------------------------------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            selectAllTreeItemsToolButton.forceActiveFocus()
        }
    }


}
