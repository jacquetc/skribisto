import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.propertyhub 1.0
import "../Commons"
import "../Items"
import ".."

PropertyPadForm {
    id: root

    property int projectId: -2
    property int paperId: -2

    onProjectIdChanged: {
        if(projectId === -2 || paperId === -2 ){
            return
        }

        determinePrintableToolButton()
        determineModifiableToolButton()
        determineFavoriteToolButton()
    }
    onPaperIdChanged: {
        if(projectId === -2 || paperId === -2 ){
            return
        }

        determinePrintableToolButton()
        determineModifiableToolButton()
        determineFavoriteToolButton()
    }

    required property var propertyHub
    required property var paperHub

    Connections{
        target: propertyHub
        function onPropertyChanged(projectId, propertyId, paperId, name, value){
            if(projectId === root.projectId && paperId === root.paperId){
                if(name === "printable"){
                    determinePrintableToolButton()
                }
                if(name === "modifiable"){
                    determineModifiableToolButton()
                }
                if(name === "favorite"){
                    determineFavoriteToolButton()
                }
            }
        }
    }



    QtObject{
        id: switchPrivate
        property bool updateBlocked: false
    }

    // printableToolButton :

    function determinePrintableToolButton(){
        switchPrivate.updateBlocked = true

        printableToolButton.checked = propertyHub.getProperty(projectId, paperId, "printable", "true") === "true"

        switchPrivate.updateBlocked = false
    }

    printableToolButton.onCheckedChanged: {
        if(switchPrivate.updateBlocked){
            return
        }
        var checked = printableToolButton.checked ? "true" : "false"
        propertyHub.setProperty(projectId, paperId, "printable", checked)

        var childrenList = paperHub.getAllChildren(projectId, paperId)

        childrenList.forEach(function(childId, index, array) {
            propertyHub.setProperty(projectId, childId, "printable", checked)
        })
    }
    printableToolButton.icon.source: "qrc:///icons/backup/document-print.svg"



    // modifiableToolButton :

    function determineModifiableToolButton(){
        switchPrivate.updateBlocked = true

        modifiableToolButton.checked = propertyHub.getProperty(projectId, paperId, "modifiable", "true") === "true"

        switchPrivate.updateBlocked = false
    }

    modifiableToolButton.onCheckedChanged: {
        if(switchPrivate.updateBlocked){
            return
        }
        var checked = modifiableToolButton.checked ? "true" : "false"
        propertyHub.setProperty(projectId, paperId, "modifiable", checked)

        var childrenList = paperHub.getAllChildren(projectId, paperId)

        if(!modifiableToolButton.checked){
            paperHub.addAttribute(projectId, paperId, "locked")
        }
        else {
            paperHub.removeAttribute(projectId, paperId, "locked")
        }

        childrenList.forEach(function(childId, index, array) {
            propertyHub.setProperty(projectId, childId, "modifiable", checked)
        })

    }
    modifiableToolButton.icon.source: "qrc:///icons/backup/edit-rename.svg"



    // favoriteToolButton :


    Action{
        id: favoriteAction
        checkable: true
        icon.source: "qrc:///icons/backup/favorite.svg"
        onCheckedChanged: {
            if(switchPrivate.updateBlocked){
                return
            }
            var checked = favoriteAction.checked ? "true" : "false"
            propertyHub.setProperty(projectId, paperId, "favorite", checked)

            if(favoriteAction.checked){
                paperHub.addAttribute(projectId, paperId, "favorite")
            }
            else {
                paperHub.removeAttribute(projectId, paperId, "favorite")
            }

        }
    }
    favoriteToolButton.action: favoriteAction

    function determineFavoriteToolButton(){
        switchPrivate.updateBlocked = true

        favoriteAction.checked = propertyHub.getProperty(projectId, paperId, "favorite", "false") === "true"

        switchPrivate.updateBlocked = false
    }




}
