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

        Loader {
            id: toolboxLoader
            sourceComponent: modelData

            width: scrollView.width

            onLoaded: {

                //toolboxFlickable.contentHeight = toolboxLayout.childrenRect.height
                var iconSource = toolboxLoader.item.iconSource
                var showButtonText = toolboxLoader.item.showButtonText
                toolButtonModel.append({
                                           "iconSource": iconSource,
                                           "showButtonText": showButtonText,
                                           "toolbox": toolboxLoader
                                       })
            }

            Binding {
                target: toolboxLoader.item
                when: toolboxLoader.status === Loader.Ready
                property: "height"
                value: toolboxLoader.item.implicitHeight
                restoreMode: Binding.RestoreBindingOrValue
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
    toolButtonListView.model: toolButtonModel

    toolButtonListView.delegate: toolButtonComponent

    toolButtonListView.onContentXChanged: {
        if(toolButtonListView.contentX === 0){
            goingAtTheBeginningOfToolButtonListView = false
        }

    }

    Behavior on toolButtonListView.contentX {
        enabled: SkrSettings.ePaperSettings.animationEnabled
        SpringAnimation {
            spring: 2
            mass: 0.2
            damping: 0.2
        }
    }

    showTheBeginningButton.onClicked: {
        goingAtTheBeginningOfToolButtonListView = true
        toolButtonListView.contentX = 0
    }

    showTheEndButton.onClicked: {
        toolButtonListView.contentX = toolButtonListView.contentWidth - toolButtonListView.width + showTheEndButton.width
    }


    Component {
        id: toolButtonComponent
        SkrToolButton {
            id: toolButton
            checkable: true
            icon.source: model.iconSource
            text: model.showButtonText

            onCheckedChanged: {
                model.toolbox.visible = toolButton.checked
            }

            Binding on checked {
                value: model.toolbox.visible
                delayed: true
                restoreMode: Binding.RestoreBindingOrValue
            }


            WheelHandler {
                onWheel: function(event) {
                    toolButtonListView.flick((event.angleDelta.y * 4), 0)
                }
            }
        }
    }

    //------------------------------------------------------------------------
    //-----toolboxes------------------------------------------------------------
    //-----------------------------------------------------------------------
    property var toolboxes: []

    function determineAndAddToolboxPlugins() {
        var toolboxUrlList = skrTreeManager.findToolboxUrlsForProject()

        for (var i in toolboxUrlList) {
            var url = toolboxUrlList[i]
            var pluginComponent = Qt.createComponent(
                        url, Component.PreferSynchronous, root)
            toolboxes.push(pluginComponent)
        }
    }

    //-----------------------------------------------------------
    //-----------------------------------------------------------
    //-----------------------------------------------------------
    signal hideDockCalled
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-previous-view.svg"
        onTriggered: {
            hideDockCalled()
        }
    }
}
