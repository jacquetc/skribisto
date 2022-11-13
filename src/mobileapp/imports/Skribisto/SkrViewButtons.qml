import QtQuick
import QtQuick.Layouts
import QtQml
import QtQuick.Controls

import SkrControls

Item {
    id: base


    required property var position
    required property var viewManager

    signal openInNewWindowCalled()
    signal splitCalled(int position)

    Component.onCompleted: {
        determineButtonsAvailability()
    }

    onPositionChanged: {
        determineButtonsAvailability()
    }

    function determineButtonsAvailability(){
        switch(position){
        case Qt.TopLeftCorner:
            closeSplitButton.visible = false
            break
        case Qt.TopRightCorner:
            splitVerticallyMenuItem.visible = false
            break
        case Qt.BottomLeftCorner:
            splitHorizontallyMenuItem.visible = false
            break
        case Qt.BottomRightCorner:
            splitVerticallyMenuItem.visible = false
            break

        }
    }

    function determineAndRunSplitHorizontallyButtonFuntion(){
        switch(position){
        case Qt.TopLeftCorner:
            viewManager.openBottomLeftView()
            splitCalled(Qt.BottomLeftCorner)
            break
        case Qt.BottomLeftCorner:
            break
        case Qt.TopRightCorner:
            viewManager.openBottomRightView()
            splitCalled(Qt.BottomRightCorner)
            break
        case Qt.BottomRightCorner:
            viewManager.openTopRightView()
            splitCalled(Qt.TopRightCorner)
            break

        }
    }

    function determineAndRunSplitVerticallyButtonFuntion(){
        switch(position){
        case Qt.TopLeftCorner:
            viewManager.openTopRightView()
            splitCalled(Qt.TopRightCorner)
            break
        case Qt.BottomLeftCorner:
            viewManager.openTopRightView()
            splitCalled(Qt.TopRightCorner)
            break
        case Qt.TopRightCorner:
            break
        case Qt.BottomRightCorner:
            break

        }
    }




    implicitHeight: rowLayout.height
    implicitWidth: rowLayout.width



    RowLayout {
        id: rowLayout

        //        implicitWidth: rowLayout.childrenRect.width + rowLayout.spacing
        //        implicitHeight: rowLayout.childrenRect.height


        //    implicitWidth: {return rowLayout.childrenRect.width + rowLayout.spacing}
        //    implicitHeight: {return rowLayout.childrenRect.height}


        SkrToolButton {
            id: splitMenuButton
            text: qsTr("Split")
            Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
            Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            checkable: true
            padding: 0
            icon{
                source: "icons/3rdparty/backup/view-split-top-bottom.svg"

            }

            onCheckedChanged: {
                if(checked){
                    splitMenu.popup(splitMenuButton, splitMenuButton.x, splitMenuButton.height)
                }
                else {
                    if(splitMenu.visible){
                        splitMenu.close()
                    }
                }



            }

        }

        SkrToolButton {
            id: closeSplitButton
            text: qsTr("Close view")
            Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
            Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            padding: 0
            icon{
                source: "icons/3rdparty/backup/view-close.svg"
            }
            onClicked: {
                viewManager.closeView(position)

            }
        }


    }



    SkrMenu{
        id: splitMenu

        onClosed: {
            splitMenuButton.checked = false
        }

        SkrMenuItem{
            id: splitHorizontallyMenuItem
            height: splitHorizontallyMenuItem.visible ? undefined : 0
            text: qsTr("Split")
            icon{
                source: "icons/3rdparty/backup/view-split-top-bottom.svg"
            }

            onClicked: {
                determineAndRunSplitHorizontallyButtonFuntion()

            }
        }
        SkrMenuItem{
            id: splitVerticallyMenuItem
            height: splitVerticallyMenuItem.visible ? undefined : 0
            text: qsTr("Split vertically")
            icon{
                source: "icons/3rdparty/backup/view-split-left-right.svg"
            }

            onClicked: {
                determineAndRunSplitVerticallyButtonFuntion()
            }
        }
        SkrMenuItem{
            id: openInNewWindowMenuItem
            text: qsTr("Open in new window")
            icon {
                source: "icons/3rdparty/backup/window-new.svg"
            }


            onClicked: {
                openInNewWindowCalled()



            }
        }
    }
}
