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

            var toolBoxes = viewManager.getToolBoxesFrom(position)

            toolButtonModel.clear()
            toolBoxRepeater.model = toolBoxes
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
        id: toolBoxLoaderComponent

        Loader{
            id: toolBoxLoader
            sourceComponent: modelData


            width: scrollView.width

            onLoaded: {

                //toolBoxFlickable.contentHeight = toolBoxLayout.childrenRect.height

                var iconSource = toolBoxLoader.item.iconSource
                var showButtonText = toolBoxLoader.item.showButtonText
                toolButtonModel.append({"iconSource": iconSource, "showButtonText": showButtonText,
                                           "toolBox": toolBoxLoader})

            }

            Binding {
                target: toolBoxLoader.item
                when: toolBoxLoader.status === Loader.Ready
                property: "height"
                value: toolBoxLoader.item.implicitHeight

            }

        }
    }
    toolBoxRepeater.delegate: toolBoxLoaderComponent


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
                toolButtonLoader.item.toolBox =  model.toolBox
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

            property var toolBox
            property string iconSource

            onCheckedChanged: {
                toolBox.visible = toolButton.checked
            }

            Binding on checked{
                value: toolBox.visible
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
