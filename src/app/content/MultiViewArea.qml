import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml
import SkrControls
import Skribisto
import backend

MultiViewAreaForm {
    id: root

    property alias viewManager: viewManager

    QtObject {
        id: priv
        property real multiViewAreaPreviousHeight: multiViewArea.height
        property real multiViewAreaPreviousHeight2: multiViewArea.height
        property real multiViewAreaPreviousWidth: multiViewArea.width
    }

    SkrViewManager {
        id: viewManager
        rootWindow: ApplicationWindow.window
        horizontalSplitView: root.horizontalSplitView
        leftVerticalSplitView: root.leftVerticalSplitView
        rightVerticalSplitView: root.rightVerticalSplitView
        loader_top_left: root.loader_top_left
        loader_bottom_left: root.loader_bottom_left
        loader_top_right: root.loader_top_right
        loader_bottom_right: root.loader_bottom_right
        multiViewArea: root.multiViewArea
    }

    //horizontalSplitView:
    Component.onCompleted: {
        //console.log("onCompleted");
        loader_top_left.setSource("EmptyPage.qml", {
                                      "position": Qt.TopLeftCorner,
                                      "viewManager": viewManager
                                  })
        restoreViewsFromSettingsTimer.start()
    }

    Timer {
        id: restoreViewsFromSettingsTimer
        interval: 30
        onTriggered: {
            viewManager.restoreViewsFromSettings()
        }
    }

    Connections {
        target: multiViewArea
        enabled: loader_bottom_left.visible
        function onHeightChanged() {
            var delta = multiViewArea.height - priv.multiViewAreaPreviousHeight
            loader_top_left.preferredHeight = loader_top_left.SplitView.preferredHeight + delta / 2
            priv.multiViewAreaPreviousHeight = multiViewArea.height

            if (multiViewArea.height < minimum * 2) {
                viewManager.closeBottomLeftView()
            }
        }
    }
    Connections {
        target: multiViewArea
        enabled: loader_bottom_right.visible
        function onHeightChanged() {
            var delta = multiViewArea.height - priv.multiViewAreaPreviousHeight2
            loader_top_right.preferredHeight
                    = loader_top_right.SplitView.preferredHeight + delta / 2
            priv.multiViewAreaPreviousHeight2 = multiViewArea.height

            if (multiViewArea.height < minimum * 2) {
                viewManager.closeBottomRightView()
            }
        }
    }

    Connections {
        target: multiViewArea
        enabled: loader_top_right.visible || loader_bottom_right.visible
        function onWidthChanged() {
            var delta = multiViewArea.width - priv.multiViewAreaPreviousWidth
            leftVerticalSplitView.preferredWidth
                    = leftVerticalSplitView.SplitView.preferredWidth + delta / 2
            priv.multiViewAreaPreviousWidth = multiViewArea.width

            if (multiViewArea.width < minimum * 2) {
                viewManager.closeBottomRightView()
                viewManager.closeTopRightView()
            }
        }
    }

    //    Connections {
    //        target: baseForDrawers
    //        enabled: loader_top_right.visible === true || loader_bottom_right.visible === true
    //        function onWidthChanged(){
    //            console.log("parent.width", baseForDrawers.width)
    //            console.log("horizontalSplitView.width", horizontalSplitView.width)
    //            if(baseForDrawers.width < horizontalSplitView.width){
    //                var delta = baseForDrawers.width - root.width
    //                leftVerticalSplitView.preferredWidth = leftVerticalSplitView.SplitView.preferredWidth - delta

    //            }
    //        }

    //    }
}
