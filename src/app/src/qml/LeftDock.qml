import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.settings 1.1
import QtQml 2.15
import "Items"
import "Commons"

LeftDockForm {
    id: root


    required property var viewManager
    //-----------------------------------------------------------
    //--------------- toolboxes Behavior------------------------
    //-----------------------------------------------------------

    //    Settings {
    //        id: settings
    //        category: "leftDock"

    //        property bool navigationViewVisible: true
    //        property bool documentViewVisible: true
    //    }

    //    function loadConf(){

    //        navigationViewToolButton.checked = settings.navigationViewVisible
    //        documentViewToolButton.checked = settings.documentViewVisible

    //    }

    //    function resetConf(){
    //        settings.navigationViewVisible = true
    //        settings.documentViewVisible = true
    //    }

    //    Component.onCompleted: {
    //        loadConf()
    //        initNavigationView()
    //        Globals.resetDockConfCalled.connect(resetConf)

    //    }

    //    Component.onDestruction: {
    //        settings.navigationViewVisible = navigationViewToolButton.checked
    //        settings.documentViewVisible = documentViewToolButton.checked

    //    }
    Component.onCompleted: {
        toolButtonModel.clear()
        determineAndAddToolboxPlugins()
        toolboxRepeater.model = toolboxes
    }

    Component {
        id: toolboxLoaderComponent

        Loader{
            id: toolboxLoader
            sourceComponent: modelData


            width: scrollView.width

            onLoaded: {

                //toolboxFlickable.contentHeight = toolboxLayout.childrenRect.height

                var iconSource = toolboxLoader.item.iconSource
                var showButtonText = toolboxLoader.item.showButtonText
                toolButtonModel.append({"iconSource": iconSource, "showButtonText": showButtonText,
                                           "toolbox": toolboxLoader})

            }

            Binding {
                target: toolboxLoader.item
                when: toolboxLoader.status === Loader.Ready
                property: "height"
                value: toolboxLoader.item.implicitHeight

            }

        }
    }
    toolboxRepeater.delegate: toolboxLoaderComponent


    //--------------------------------------------------------------
    //--------------------------------------------------------------
    //--------------------------------------------------------------




    ListModel {
        id: toolButtonModel
    }
    toolButtonRepeater.model: toolButtonModel


    Component {
        id: toolButtonLoaderComponent


        Loader {
            id: toolButtonLoader
            sourceComponent: toolButtonComponent

            onLoaded: {
                toolButtonLoader.item.iconSource = model.iconSource
                toolButtonLoader.item.text = model.showButtonText
                toolButtonLoader.item.toolbox =  model.toolbox
            }


        }
    }
    toolButtonRepeater.delegate: toolButtonLoaderComponent


    Component {
        id: toolButtonComponent
        SkrToolButton {
            id: toolButton
            checkable: true
            icon.source: iconSource

            property var toolbox
            property string iconSource

            onCheckedChanged: {
                toolbox.visible = toolButton.checked
            }

            Binding on checked{
                value: toolbox.visible
                delayed: true
                restoreMode: Binding.RestoreBindingOrValue
            }

        }

    }



    //------------------------------------------------------------------------
    //-----toolboxes------------------------------------------------------------
    //-----------------------------------------------------------------------


    property var toolboxes: []

    function determineAndAddToolboxPlugins(){
        var toolboxUrlList = skrTreeManager.findToolboxUrlsForProject()

        for(var i in toolboxUrlList){
            var url = toolboxUrlList[i]
            var pluginComponent = Qt.createComponent(url, Component.PreferSynchronous, root)
            toolboxes.push(pluginComponent)
        }

    }


    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------

    signal hideDockCalled()
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-previous-view.svg"
        onTriggered: {
            hideDockCalled()
        }

    }


}
