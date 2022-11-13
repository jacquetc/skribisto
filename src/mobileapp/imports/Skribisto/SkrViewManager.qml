import QtQuick
import QtQml
import backend

Item {
    id: base
    visible: false

    required property var rootWindow
    required property var horizontalSplitView
    required property var leftVerticalSplitView
    required property var rightVerticalSplitView
    required property var loader_top_left
    required property var loader_bottom_left
    required property var loader_top_right
    required property var loader_bottom_right
    required property var multiViewArea

    //---------------------------------------------------------------
    SKRViewManager {
        id: viewManagerCpp
        windowManager: skrWindowManager
        rootWindow: base.rootWindow

        //function openBottomLeftView()
    }

    //---------------------------------------------------------------
    function openBottomLeftView() {
        loader_bottom_left.setSource("EmptyPage.qml", {
                                         "position": Qt.BottomLeftCorner,
                                         "viewManager": viewManager
                                     })
        loader_bottom_left.item.position = Qt.BottomLeftCorner
        loader_bottom_left.visible = true
        loader_top_left.preferredHeight = leftVerticalSplitView.height / 2
    }

    //---------------------------------------------------------------
    function setTopLeftViewHeight(newHeight) {
        loader_top_left.preferredHeight = newHeight
    }

    //---------------------------------------------------------------
    function closeBottomLeftView() {
        loader_bottom_left.setSource("EmptyPage.qml", {
                                         "position": Qt.BottomLeftCorner,
                                         "viewManager": viewManager
                                     })
        loader_bottom_left.visible = false
    }

    //---------------------------------------------------------------
    function openTopRightView() {
        loader_top_right.setSource("EmptyPage.qml", {
                                       "position": Qt.TopRightCorner,
                                       "viewManager": viewManager
                                   })
        loader_top_right.item.position = Qt.TopRightCorner
        rightVerticalSplitView.visible = true
        loader_top_right.visible = true
        leftVerticalSplitView.preferredWidth = horizontalSplitView.width / 2
    }

    //---------------------------------------------------------------
    function setLeftVerticalSplitViewWidth(newWidth) {
        leftVerticalSplitView.preferredWidth = newWidth
    }

    //---------------------------------------------------------------
    function closeTopRightView() {
        loader_top_right.setSource("EmptyPage.qml", {
                                       "position": Qt.TopRightCorner,
                                       "viewManager": viewManager
                                   })
        loader_top_right.visible = false
        if (loader_bottom_right.visible === false) {
            rightVerticalSplitView.visible = false
        }
    }

    //---------------------------------------------------------------
    function openBottomRightView() {
        loader_bottom_right.setSource("EmptyPage.qml", {
                                          "position": Qt.BottomRightCorner,
                                          "viewManager": viewManager
                                      })
        loader_bottom_right.item.position = Qt.BottomRightCorner

        rightVerticalSplitView.visible = true
        loader_bottom_right.visible = true
        if (loader_top_right.visible) {
            loader_top_right.preferredHeight = rightVerticalSplitView.height / 2
        } else {
            leftVerticalSplitView.preferredWidth = horizontalSplitView.width / 2
        }
    }

    //---------------------------------------------------------------
    function setTopRightViewHeight(newHeight) {
        loader_top_right.preferredHeight = newHeight
    }

    //---------------------------------------------------------------
    function closeBottomRightView() {
        loader_bottom_right.setSource("EmptyPage.qml", {
                                          "position": Qt.BottomRightCorner,
                                          "viewManager": viewManager
                                      })
        loader_bottom_right.visible = false
        if (loader_top_right.visible === false) {
            rightVerticalSplitView.visible = false
        }
    }

    //---------------------------------------------------------------
    function closeView(position) {
        switch (position) {
        case Qt.BottomLeftCorner:
            closeBottomLeftView()
            break
        case Qt.TopRightCorner:
            closeTopRightView()
            break
        case Qt.BottomRightCorner:
            closeBottomRightView()
            break
        }
    }

    //---------------------------------------------------------------
    function openView(position) {
        switch (position) {
        case Qt.BottomLeftCorner:
            openBottomLeftView()
            break
        case Qt.TopRightCorner:
            openTopRightView()
            break
        case Qt.BottomRightCorner:
            openBottomRightView()
            break
        }
    }

    //---------------------------------------------------------------
    function restoreViewsFromSettings() {
        var windowGroup = "window_" + skrWindowManager.getWindowId(rootWindow)
        var verticalMiddle = leftVerticalSplitView.height / 2
        var horizontalMiddle = horizontalSplitView.height / 2

        //size :
        var topLeftViewSize = skrUserSettings.getSetting(windowGroup,
                                                         "topLeftViewSize",
                                                         verticalMiddle)
        var topRightViewSize = skrUserSettings.getSetting(windowGroup,
                                                          "topRightViewSize",
                                                          verticalMiddle)
        var leftSplitViewSize = skrUserSettings.getSetting(windowGroup,
                                                           "leftSplitViewSize",
                                                           horizontalMiddle)

        //visible :
        var bottomLeftViewVisible = skrUserSettings.getSetting(
                    windowGroup, "bottomLeftViewVisible", false)
        var topRightViewVisible = skrUserSettings.getSetting(
                    windowGroup, "topRightViewVisible", false)
        var bottomRightViewVisible = skrUserSettings.getSetting(
                    windowGroup, "bottomRightViewVisible", false)

        // failsafe for if width is superior to window width
        var windowWidth = rootWindow.width
        var windowHeight = rootWindow.height

        if (leftSplitViewSize > windowWidth) {
            leftSplitViewSize = windowWidth / 2
        }
        if (topLeftViewSize > windowHeight) {
            topLeftViewSize = windowHeight / 2
        }
        if (topRightViewSize > windowHeight) {
            topRightViewSize = windowHeight / 2
        }

        if (bottomLeftViewVisible) {
            openBottomLeftView()
            setTopLeftViewHeight(topLeftViewSize)
        }
        if (topRightViewVisible) {
            openTopRightView()
            setLeftVerticalSplitViewWidth(leftSplitViewSize)
        }
        if (bottomRightViewVisible) {
            openBottomRightView()
            setTopRightViewHeight(topRightViewSize)
            if (!topRightViewVisible) {
                setLeftVerticalSplitViewWidth(leftSplitViewSize)
            }
        }
    }

    //---------------------------------------------------------------
    function saveViewsToSettings() {

        var windowGroup = "window_" + skrWindowManager.getWindowId(rootWindow)

        //reset
        skrUserSettings.removeSetting(windowGroup, "bottomLeftViewVisible")
        skrUserSettings.removeSetting(windowGroup, "topLeftViewSize")
        skrUserSettings.removeSetting(windowGroup, "topRightViewVisible")
        skrUserSettings.removeSetting(windowGroup, "leftSplitViewSize")
        skrUserSettings.removeSetting(windowGroup, "bottomRightViewVisible")
        skrUserSettings.removeSetting(windowGroup, "topRightViewSize")

        // save
        if (loader_bottom_left.visible) {
            skrUserSettings.setSetting(windowGroup, "bottomLeftViewVisible",
                                       loader_bottom_left.visible)
            skrUserSettings.setSetting(windowGroup, "topLeftViewSize",
                                       loader_top_left.height)
        }
        if (loader_top_right.visible) {
            skrUserSettings.setSetting(windowGroup, "topRightViewVisible",
                                       loader_top_right.visible)
            skrUserSettings.setSetting(windowGroup, "leftSplitViewSize",
                                       leftVerticalSplitView.width)
        }
        if (loader_bottom_right.visible) {
            skrUserSettings.setSetting(windowGroup, "bottomRightViewVisible",
                                       loader_top_right.visible)
            skrUserSettings.setSetting(windowGroup, "topRightViewSize",
                                       loader_top_right.height)
            if (loader_top_left.visible) {
                skrUserSettings.setSetting(windowGroup, "leftSplitViewSize",
                                           leftVerticalSplitView.width)
            }
        }
    }

    //--------------------------------------------------------------
    function restoreProjectOpenedItems(projectId) {

        var keyPrefix = "window_" + skrWindowManager.getWindowId(
                    rootWindow) + "_"

        var topLeftTreeId = skrUserSettings.getFromProjectSettingHash(
                    projectId, "openedItems", keyPrefix + "top_left", -2)
        var topRightTreeId = skrUserSettings.getFromProjectSettingHash(
                    projectId, "openedItems", keyPrefix + "top_right", -2)
        var bottomLeftTreeId = skrUserSettings.getFromProjectSettingHash(
                    projectId, "openedItems", keyPrefix + "bottom_left", -2)
        var bottomRightTreeId = skrUserSettings.getFromProjectSettingHash(
                    projectId, "openedItems", keyPrefix + "bottom_right", -2)

        var top_left_additionalProperties = skrUserSettings.getFromProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "top_left", -2)
        var top_right_additionalProperties = skrUserSettings.getFromProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "top_right", -2)
        var bottom_left_additionalProperties = skrUserSettings.getFromProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "bottom_left", -2)
        var bottom_right_additionalProperties = skrUserSettings.getFromProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "bottom_right", -2)

        //load tree items
        if (loader_top_left.visible && topLeftTreeId !== -2) {
            insertAdditionalPropertyDict(top_left_additionalProperties)
            if (typeof topLeftTreeId === "string") {
                loadProjectDependantPageAt(projectId, topLeftTreeId,
                                           Qt.TopLeftCorner)
            } else {
                loadTreeItemAt(projectId, topLeftTreeId, Qt.TopLeftCorner)
            }
        }
        if (loader_top_right.visible && topRightTreeId !== -2) {
            insertAdditionalPropertyDict(top_right_additionalProperties)
            if (typeof topRightTreeId === "string") {
                loadProjectDependantPageAt(projectId, topRightTreeId,
                                           Qt.TopRightCorner)
            } else {
                loadTreeItemAt(projectId, topRightTreeId, Qt.TopRightCorner)
            }
        }
        if (loader_bottom_left.visible && bottomLeftTreeId !== -2) {
            insertAdditionalPropertyDict(bottom_left_additionalProperties)
            if (typeof bottomLeftTreeId === "string") {
                loadProjectDependantPageAt(projectId, bottomLeftTreeId,
                                           Qt.BottomLeftCorner)
            } else {
                loadTreeItemAt(projectId, bottomLeftTreeId, Qt.BottomLeftCorner)
            }
        }
        if (loader_bottom_right.visible && bottomRightTreeId !== -2) {
            insertAdditionalPropertyDict(bottom_right_additionalProperties)
            if (typeof bottomRightTreeId === "string") {
                loadProjectDependantPageAt(projectId, bottomRightTreeId,
                                           Qt.BottomRightCorner)
            } else {
                loadTreeItemAt(projectId, bottomRightTreeId,
                               Qt.BottomRightCorner)
            }
        }
    }

    //---------------------------------------------------------------
    function saveProjectOpenedItems(projectId) {

        var keyPrefix = "window_" + skrWindowManager.getWindowId(
                    rootWindow) + "_"

        //reset
        skrUserSettings.removeFromProjectSettingHash(projectId, "openedItems",
                                                     keyPrefix + "top_left")
        skrUserSettings.removeFromProjectSettingHash(projectId, "openedItems",
                                                     keyPrefix + "top_right")
        skrUserSettings.removeFromProjectSettingHash(projectId, "openedItems",
                                                     keyPrefix + "bottom_left")
        skrUserSettings.removeFromProjectSettingHash(projectId, "openedItems",
                                                     keyPrefix + "bottom_right")

        // save
        var top_left_treeItemId = -2
        var top_left_additionalProperties = ({})
        var item = loader_top_left.item
        if (item) {
            if (item.projectId === projectId) {
                top_left_treeItemId = item.treeItemId
                if (top_left_treeItemId == -1) {
                    top_left_treeItemId = item.pageType
                }
                top_left_additionalProperties = item.additionalPropertiesForSavingView
            }
        }
        var top_right_treeItemId = -2
        var top_right_additionalProperties = ({})
        item = loader_top_right.item
        if (item) {
            if (item.projectId === projectId) {
                top_right_treeItemId = item.treeItemId
                if (top_right_treeItemId == -1) {
                    top_right_treeItemId = item.pageType
                }
                top_right_additionalProperties = item.additionalPropertiesForSavingView
            }
        }
        var bottom_left_treeItemId = -2
        var bottom_left_additionalProperties = ({})
        item = loader_bottom_left.item
        if (item) {
            if (item.projectId === projectId) {
                bottom_left_treeItemId = item.treeItemId
                if (bottom_left_treeItemId == -1) {
                    bottom_left_treeItemId = item.pageType
                }
                bottom_left_additionalProperties = item.additionalPropertiesForSavingView
            }
        }
        var bottom_right_treeItemId = -2
        var bottom_right_additionalProperties = ({})
        item = loader_bottom_right.item
        if (item) {
            if (item.projectId === projectId) {
                bottom_right_treeItemId = item.treeItemId
                if (bottom_right_treeItemId == -1) {
                    bottom_right_treeItemId = item.pageType
                }
                bottom_right_additionalProperties = item.additionalPropertiesForSavingView
            }
        }

        skrUserSettings.insertInProjectSettingHash(projectId, "openedItems",
                                                   keyPrefix + "top_left",
                                                   top_left_treeItemId)
        skrUserSettings.insertInProjectSettingHash(projectId, "openedItems",
                                                   keyPrefix + "top_right",
                                                   top_right_treeItemId)
        skrUserSettings.insertInProjectSettingHash(projectId, "openedItems",
                                                   keyPrefix + "bottom_left",
                                                   bottom_left_treeItemId)
        skrUserSettings.insertInProjectSettingHash(projectId, "openedItems",
                                                   keyPrefix + "bottom_right",
                                                   bottom_right_treeItemId)

        skrUserSettings.insertInProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "top_left", top_left_additionalProperties)
        skrUserSettings.insertInProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "top_right", top_right_additionalProperties)
        skrUserSettings.insertInProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "bottom_left", bottom_left_additionalProperties)
        skrUserSettings.insertInProjectSettingHash(
                    projectId, "additionalPropertiesForSavingView",
                    keyPrefix + "bottom_right",
                    bottom_right_additionalProperties)
    }

    //---------------------------------------------------------------
    function isViewVisible(position) {
        var isVisible = false

        switch (position) {
        case Qt.BottomLeftCorner:
            isVisible = loader_bottom_left.visible
            break
        case Qt.BottomRightCorner:
            isVisible = loader_bottom_right.visible
            break
        case Qt.TopLeftCorner:
            isVisible = loader_top_left.visible
            break
        case Qt.TopRightCorner:
            isVisible = loader_top_right.visible
            break
        }

        return isVisible
    }

    //---------------------------------------------------------------
    function getLoaderFrom(position) {
        var loader

        switch (position) {
        case Qt.BottomLeftCorner:
            loader = loader_bottom_left
            break
        case Qt.BottomRightCorner:
            loader = loader_bottom_right
            break
        case Qt.TopLeftCorner:
            loader = loader_top_left
            break
        case Qt.TopRightCorner:
            loader = loader_top_right
            break
        }

        return loader
    }

    //---------------------------------------------------------------
    property int focusedPosition: Qt.TopLeftCorner

    signal focusedChanged(int position, int projectId, int treeItemId, string pageType)

    onFocusedPositionChanged: {
        if (focusedPosition === -1) {
            return
        }

        prepareAndCallFocusedChangedSignal()
    }

    function prepareAndCallFocusedChangedSignal() {

        var loader = getLoaderFrom(focusedPosition)
        var projectId = loader.item.projectId
        var treeItemId = loader.item.treeItemId
        var pageType = loader.item.pageType

        focusedPrivate.focusedPageType = pageType
        focusedPrivate.focusedTreeItemId = treeItemId
        focusedPrivate.focusedProjectId = projectId
        focusedPage = loader.item

        focusedChanged(focusedPosition, projectId, treeItemId, pageType)
    }

    readonly property string focusedPageType: focusedPrivate.focusedPageType
    readonly property int focusedTreeItemId: focusedPrivate.focusedTreeItemId
    readonly property int focusedProjectId: focusedPrivate.focusedProjectId
    property var focusedPage

    QtObject {
        id: focusedPrivate
        property string focusedPageType: ""
        property int focusedTreeItemId: -1
        property int focusedProjectId: -1
    }

    //---------------------------------------------------------------
    QtObject {
        id: priv
        property var additionalProperties: ({})
    }

    function insertAdditionalProperty(key, value) {
        priv.additionalProperties[key] = value
    }
    function insertAdditionalPropertyDict(dict) {
        priv.additionalProperties = dict
    }

    function loadTreeItemAt(projectId, treeItemId, position) {

        var pageType = ""
        if (projectId === -1 && treeItemId !== -1) {
            pageType = "EMPTY"
        } else {
            pageType = skrData.treeHub().getType(projectId, treeItemId)
        }

        if (!isViewVisible(position)) {
            return
        }

        var qmlUrl = viewManagerCpp.getQmlUrlFromPageType(pageType)

        var properties = {
            "position": position,
            "viewManager": viewManager,
            "projectId": projectId,
            "treeItemId": treeItemId,
            "pageType": pageType
        }

        for (var additionalProperty in priv.additionalProperties) {
            properties[additionalProperty] = priv.additionalProperties[additionalProperty]
        }
        priv.additionalProperties = ({})

        var loader = getLoaderFrom(position)
        loader.setSource(qmlUrl, properties)

        if (loader.status === Loader.Error) {
            console.log("Error while loading", pageType)
            return
        }

        if (position === focusedPosition) {
            prepareAndCallFocusedChangedSignal()
        }
    }

    //---------------------------------------------------------------
    function loadTreeItem(projectId, treeItemId) {
        loadTreeItemAt(projectId, treeItemId, focusedPosition)
    }

    //---------------------------------------------------------------
    function loadTreeItemAtAnotherView(projectId, treeItemId) {

        var otherPosition = getAnotherPosition()
        openView(otherPosition)
        loadTreeItemAt(projectId, treeItemId, otherPosition)
    }

    //---------------------------------------------------------------
    function getAnotherPosition() {

        var otherPosition
        switch (focusedPosition) {
        case Qt.BottomLeftCorner:
            otherPosition = Qt.TopRightCorner
            break
        case Qt.BottomRightCorner:
            otherPosition = Qt.BottomLeftCorner
            break
        case Qt.TopLeftCorner:
            otherPosition = Qt.TopRightCorner
            break
        case Qt.TopRightCorner:
            otherPosition = Qt.BottomRightCorner
            break
        }

        return otherPosition
    }

    //---------------------------------------------------------------
    function loadProjectIndependantPageAt(pageType, position) {

        if (!isViewVisible(position)) {
            return
        }

        var qmlUrl = viewManagerCpp.getQmlUrlFromPageType(pageType)

        var properties = {
            "position": position,
            "viewManager": viewManager,
            "projectId": -1,
            "treeItemId": -1,
            "pageType": pageType
        }

        for (var additionalProperty in priv.additionalProperties) {
            properties[additionalProperty] = priv.additionalProperties[additionalProperty]
        }
        priv.additionalProperties = ({})

        var loader = getLoaderFrom(position)
        var object = loader.setSource(qmlUrl, properties)

        if (loader.status === Loader.Error) {
            console.log("Error while loading", pageType)
            return
        }

        if (position === focusedPosition) {
            prepareAndCallFocusedChangedSignal()
        }
    }

    //---------------------------------------------------------------
    function loadProjectIndependantPage(pageType) {

        loadProjectIndependantPageAt(pageType, focusedPosition)
    }

    //---------------------------------------------------------------
    function loadProjectIndependantPageAtAnotherView(pageType) {

        var otherPosition = getAnotherPosition()
        openView(otherPosition)
        loadProjectIndependantPageAt(pageType, otherPosition)
    }

    //---------------------------------------------------------------
    function loadProjectDependantPageAt(projectId, pageType, position) {

        if (!isViewVisible(position)) {
            return
        }

        var qmlUrl = viewManagerCpp.getQmlUrlFromPageType(pageType)

        var properties = {
            "position": position,
            "viewManager": viewManager,
            "projectId": projectId,
            "treeItemId": -1,
            "pageType": pageType
        }

        for (var additionalProperty in priv.additionalProperties) {
            properties[additionalProperty] = priv.additionalProperties[additionalProperty]
        }
        priv.additionalProperties = ({})

        var loader = getLoaderFrom(position)
        var object = loader.setSource(qmlUrl, properties)

        if (loader.status === Loader.Error) {
            console.log("Error while loading", pageType)
            return
        }

        if (position === focusedPosition) {
            prepareAndCallFocusedChangedSignal()
        }
    }

    //---------------------------------------------------------------
    function loadProjectDependantPage(projectId, pageType) {

        loadProjectDependantPageAt(projectId, pageType, focusedPosition)
    }

    //---------------------------------------------------------------
    function loadProjectDependantPageAtAnotherView(projectId, pageType) {
        var otherPosition = getAnotherPosition()
        openView(otherPosition)
        loadProjectIndependantPageAt(projectId, pageType, otherPosition)
    }

    //---------------------------------------------------------------
    function getToolboxesFrom(position) {

        var loader = getLoaderFrom(position)

        if (loader.status !== Loader.Ready) {
            return
        }

        var item = loader.item
        return item.toolboxes
    }

    //---------------------------------------------------------------
    Connections {
        target: skrData.projectHub()
        function onProjectLoaded(_projectId) {
            if (skrData.projectHub().getProjectCount() === 1) {

                restoreProjectOpenedItems(_projectId)
                postProjectLoadedTimer.start()
            }
        }
    }

    Timer {
        id: postProjectLoadedTimer
        interval: 100
        onTriggered: {
            focusedPosition = Qt.TopLeftCorner
        }
    }

    //---------------------------------------------------------------
    Connections {
        target: skrData.projectHub()
        function onProjectToBeClosed(_projectId) {
            saveProjectOpenedItems(_projectId)
            closePagesByProject(_projectId)
        }
    }

    //---------------------------------------------------------
    function closePagesByProject(projectId) {
        var allPositions = [Qt.TopLeftCorner, Qt.TopRightCorner, Qt.BottomLeftCorner, Qt.BottomRightCorner]

        var i
        for (i = 0; i < allPositions.length; i++) {
            var loader = getLoaderFrom(allPositions[i])
            if (loader.visible) {
                if (loader.item) {
                    if (loader.item.projectId === projectId) {
                        loadProjectIndependantPageAt("EMPTY", allPositions[i])
                    }
                }
            }
        }
    }

    //---------------------------------------------------------------
    function closePagesWithTreeItemId(projectId, treeItemId) {

        var allPositions = [Qt.TopLeftCorner, Qt.TopRightCorner, Qt.BottomLeftCorner, Qt.BottomRightCorner]

        var i
        for (i = 0; i < allPositions.length; i++) {
            var loader = getLoaderFrom(allPositions[i])
            if (loader.visible) {
                if (loader.item) {
                    if (loader.item.projectId === projectId
                            && loader.item.treeItemId === treeItemId) {
                        loadProjectIndependantPageAt("EMPTY", allPositions[i])
                    }
                }
            }
        }
    }

    //---------------------------------------------------------------
    Connections {
        target: skrData.treeHub()
        function onTreeItemRemoved(_projectId, _treeItemId) {

            closePagesWithTreeItemId(_projectId, _treeItemId)
        }
    }

    //---------------------------------------------------------------
    Connections {
        target: skrData.treeHub()
        function ontrashedChanged(_projectId, _treeItemId, newTrashedState) {
            if (newTrashedState) {
                closePagesWithTreeItemId(_projectId, _treeItemId)
            }
        }
    }

    //---------------------------------------------------------------
}
