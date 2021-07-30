import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import "Items"
import "Commons"

RightDockForm {
    id: root
    required property var viewManager

    Connections {
        target: viewManager
        function onFocusedChanged(position, projectId, treeItemId, pageType) {

            var toolboxes = viewManager.getToolboxesFrom(position)

            toolButtonModel.clear()
            toolboxRepeater.model = toolboxes
            // var pageType = viewManager.focusedPositionPageType
        }
    }

    //    property int treeItemId : -1
    //    property int paperId : -1

    //    viewManager.onFocusedPositionChanged: {
    //        var pageType = viewManager.focusedPositionPageType
    //    }

    //--------------------------------------------------------
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

    //--------------------------------------------------------------
    //--------------------------------------------------------------
    //--------------------------------------------------------------
    signal hideDockCalled
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-next-view.svg"
        onTriggered: {
            hideDockCalled()
        }
    }
}
