import QtQuick 2.12
import QtQuick.Controls 2.4
import QtQuick.Controls.Universal 2.2
import Qt.labs.settings 1.1

RootPageForm {

    //    Drawer{
    //        id: drawer
    //        width: 70
    //        height: window.height
    //        modal: false
    //        interactive: true
    //        position: 0

    //        Loader{
    //            anchors.fill: parent
    //            sourceComponent: flow_comp

    //        }

    //    }
    Component {
        id: flow_comp

        Flow {
            id: menuButtonsflow
            property int buttonSize: 70

            //flow: statusBarMenuButtonsLoader.visible ? Flow.LeftToRight : Flow.TopToBottom
            ToolButton {
                id: welcome_button
                flat: true
                height: menuButtonsflow.buttonSize
                width: menuButtonsflow.buttonSize
                checkable: true
                display: AbstractButton.IconOnly
                action: welcomeWindowAction

                icon {
                    color: "transparent"
                    height: 100
                    width: 100
                }

                hoverEnabled: true
                ToolTip.delay: 1000
                ToolTip.timeout: 5000
                ToolTip.visible: hovered
                ToolTip.text: welcomeWindowAction.text

                KeyNavigation.right: welcomePage
                KeyNavigation.down: write_button

                Accessible.role: Accessible.Button
                Accessible.name: text
                Accessible.description: qsTr("Switch to the Welcome page")
                Accessible.onPressAction: {
                    welcomeWindowAction.trigger()
                    welcomePage.forceActiveFocus()
                }
            }

            ToolButton {
                id: write_button
                //flat: true
                height: menuButtonsflow.buttonSize
                width: menuButtonsflow.buttonSize
                display: AbstractButton.IconOnly
                action: writeWindowAction
                icon {
                    color: "transparent"
                    height: 100
                    width: 100
                }

                hoverEnabled: true
                ToolTip.delay: 1000
                ToolTip.timeout: 5000
                ToolTip.visible: hovered
                ToolTip.text: writeWindowAction.text
            }

            ToolButton {
                id: note_button
                //flat: true
                height: menuButtonsflow.buttonSize
                width: menuButtonsflow.buttonSize
                display: AbstractButton.IconOnly
                action: noteWindowAction
                icon {
                    color: "transparent"
                    height: 100
                    width: 100
                }

                hoverEnabled: true
                ToolTip.delay: 1000
                ToolTip.timeout: 5000
                ToolTip.visible: hovered
                ToolTip.text: noteWindowAction.text
            }

            ToolButton {
                id: gallery_button
                //flat: true
                height: menuButtonsflow.buttonSize
                width: menuButtonsflow.buttonSize
                display: AbstractButton.IconOnly
                action: galleryWindowAction
                icon {
                    color: "transparent"
                    height: 100
                    width: 100
                }

                hoverEnabled: true
                ToolTip.delay: 1000
                ToolTip.timeout: 5000
                ToolTip.visible: hovered
                ToolTip.text: galleryWindowAction.text
            }
            ToolButton {
                id: infos_button
                //flat: true
                height: menuButtonsflow.buttonSize
                width: menuButtonsflow.buttonSize
                display: AbstractButton.IconOnly
                action: infosWindowAction
                icon {
                    color: "transparent"
                    height: 100
                    width: 100
                }

                hoverEnabled: true
                ToolTip.delay: 1000
                ToolTip.timeout: 5000
                ToolTip.visible: hovered
                ToolTip.text: infosWindowAction.text
            }
        }
    }

    ActionGroup {
        id: windowGroup
        Action {
            id: welcomeWindowAction
            text: qsTr("Welcome")
            icon {
                name: "welcome-icon"
                source: "qrc:/pics/skribisto.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F4"
            checkable: true
            checked: true
            onTriggered: {
                root_stack.currentIndex = 0
                welcomePage.forceActiveFocus()
            }
        }

        Action {
            id: writeWindowAction
            text: qsTr("Write")
            icon {
                name: "author"
                source: "qrc:/pics/author.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F5"
            checkable: true
            onTriggered: root_stack.currentIndex = 1
        }

        Action {
            id: noteWindowAction
            text: qsTr("Notes")
            icon {
                name: "document-edit"
                source: "qrc:/pics/skribisto.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F6"
            checkable: true
            onTriggered: root_stack.currentIndex = 2
        }

        Action {
            id: galleryWindowAction
            text: qsTr("Gallery")
            icon {
                name: "document-edit"
                source: "qrc:/pics/skribisto.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F7"
            checkable: true
            onTriggered: root_stack.currentIndex = 1
        }
        Action {
            id: infosWindowAction
            text: qsTr("Informations")
            icon {
                name: "document-edit"
                source: "qrc:/pics/skribisto.svg"
                color: "transparent"
                height: 100
                width: 100
            }

            shortcut: "F6"
            checkable: true
            onTriggered: root_stack.currentIndex = 1
        }
    }

    sideMenuButtonsLoader.onLoaded: {
        sideMenuButtonsLoader.item.flow = Flow.TopToBottom
    }
    statusBarMenuButtonsLoader.onLoaded: {
        statusBarMenuButtonsLoader.item.flow = Flow.LeftToRight
        statusBarMenuButtonsLoader.item.buttonSize = 40
        statusBarMenuButtonsLoader.item.spacing = 3
    }

    Component.onCompleted: {

    }
    Component.onDestruction: {

    }

    statusBarMenuButtonsLoader.visible: SkrSettings.interfaceSettings.menuButtonsInStatusBar
    sideMenuButtonsLoader.visible: !SkrSettings.interfaceSettings.menuButtonsInStatusBar
}
