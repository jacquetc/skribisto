import QtQuick
import Skribisto
import QtQuick.Layouts
import QtQuick.Window
import QtQuick.Controls 6.4
Window {
    id: window

    visible: true
    title: "Skribisto"
    minimumHeight: 800
    minimumWidth: 800



    RowLayout {
        id: columnLayout
        anchors.fill: parent
        spacing: 0

        SideChapterList{
            Layout.fillHeight: true
            Layout.maximumWidth: 100

        }


        Item {
            id: mainScreenHolder
            Layout.fillHeight: true
            Layout.preferredWidth: window.width / 2
            MainScreen {
                id: mainScreen
                anchors.fill: parent
            }
        }

        Rectangle {
            id: verticalSeparator
            color: "#000000"
            Layout.fillHeight: true
            Layout.preferredWidth: 2
        }

        Item {
            id: sideScreenHolder
            Layout.fillWidth: true
            Layout.fillHeight: true

            SideScreen {
                id: sideScreen
                anchors.fill: parent

            }
        }
        SwipeView {
            id: swipeView
            visible: false
            currentIndex: 0
            Layout.fillWidth: true
            Layout.fillHeight: true
            Item {
                id: mainScreenHolderInSwipeView
            }
            Item {
                id: sideScreenHolderInSwipeView
            }

        }
    }


    StateGroup {
        id: stateGroup
        states: [
            State {
                name: "Portrait"
                when: window.width < 600
                ParentChange { target: mainScreen; parent: mainScreenHolderInSwipeView;}
                ParentChange { target: sideScreen; parent: sideScreenHolderInSwipeView;}
                PropertyChanges {
                    target: mainScreenHolder
                    visible: false

                }
                PropertyChanges {
                    target: sideScreenHolder
                    visible: false

                }
                PropertyChanges {
                    target: swipeView
                    visible: true

                }
                PropertyChanges {
                    target: verticalSeparator
                    visible: false

                }
            }
        ]
    }
}

