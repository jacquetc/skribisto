import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import "Items"
import "Commons"

RightDockForm {
    id: root
    required property var viewManager

    Connections{
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

    //--------------------------------------------------------------
    //--------------------------------------------------------------
    //--------------------------------------------------------------
    signal hideDockCalled()
    hideDockToolButton.action: hideDockAction

    Action {
        id: hideDockAction
        icon.source: "qrc:///icons/backup/go-next-view.svg"
        onTriggered: {
            hideDockCalled()
        }

    }

}
