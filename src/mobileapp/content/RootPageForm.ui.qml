

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SkrControls
import Skribisto

Item {

    id: rootPageBase
    property alias viewManager: multiViewArea.viewManager
    property alias mainMenuButton: mainMenuButton
    property alias showWelcomeButton: showWelcomeButton
    property alias baseForDrawers: baseForDrawers
    property alias distractionFreeButton: distractionFreeButton
    property alias themeColorButton: themeColorButton
    property alias showLeftDockButton: showLeftDockButton
    property alias showRightDockButton: showRightDockButton
    property alias topToolBarRepeater: topToolBarRepeater
    property alias headerRowLayout: headerRowLayout
    property alias headerRowLayoutPreferredHeight: headerRowLayout.layoutPreferredHeight
    property alias headerStayVisibleHoverHandler: headerStayVisibleHoverHandler
    property alias headerStayVisibleTapHandler: headerStayVisibleTapHandler
    property int showLeftDockButtonWidth: 30 * SkrSettings.interfaceSettings.zoom
    property int showRightDockButtonWidth: 30 * SkrSettings.interfaceSettings.zoom

    ColumnLayout {
        id: baseColumnLayout
        anchors.fill: parent

        RowLayout {
            id: headerRowLayout
            spacing: 0

            property int layoutPreferredHeight: 30 * SkrSettings.interfaceSettings.zoom
            Layout.preferredHeight: layoutPreferredHeight
            Layout.fillWidth: true

            HoverHandler {
                id: headerStayVisibleHoverHandler
                acceptedDevices: PointerDevice.Mouse
            }

            TapHandler {
                id: headerStayVisibleTapHandler
                acceptedDevices: PointerDevice.TouchScreen | PointerDevice.Stylus
            }
            SkrToolButton {
                id: showLeftDockButton

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: showLeftDockButtonWidth

                focusPolicy: Qt.NoFocus
            }

            SkrToolButton {
                id: mainMenuButton
                text: qsTr("Main menu")
                visible: SkrSettings.accessibilitySettings.showMenuButton
                focusPolicy: Qt.NoFocus
                padding: 2
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
                checkable: true
            }

            SkrToolButton {
                id: showWelcomeButton
                icon.color: "transparent"

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            }

            SkrToolButton {
                id: saveButton
                action: saveAction

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            }

            Breadcrumb {
                id: breadcrumb
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.alignment: Qt.AlignCenter
                Layout.fillWidth: true
                Layout.minimumWidth: 300
            }

            RowLayout {
                Repeater {
                    id: topToolBarRepeater
                }
            }

            Item {
                id: stretcher
                Layout.fillWidth: true
                Layout.minimumWidth: rootPageBase.width / 6 > 300 ? 300 : 0
            }

            SkrToolButton {
                id: distractionFreeButton

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            }

            SkrToolButton {
                id: themeColorButton

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 30 * SkrSettings.interfaceSettings.zoom
            }

            SkrToolButton {
                id: showRightDockButton

                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: showRightDockButtonWidth

                focusPolicy: Qt.NoFocus
            }
        }

        Item {
            id: baseForDrawers
            Layout.fillHeight: true
            Layout.fillWidth: true

            Item {
                id: columnLayout
                anchors.fill: parent
                anchors.leftMargin: ApplicationWindow.window.compactMode ? 0 : leftDrawer.width * leftDrawer.position
                anchors.rightMargin: ApplicationWindow.window.compactMode ? 0 : rightDrawer.width * rightDrawer.position

                Item {
                    id: middleBase
                    anchors.fill: parent

                    MultiViewArea {
                        id: multiViewArea

                        anchors.fill: parent
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

