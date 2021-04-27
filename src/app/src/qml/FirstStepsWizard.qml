import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15

import "Items"
import "Commons"
import "WelcomePage"

SkrPopup {

    id: root
    parent: Overlay.overlay
    width: Overlay.overlay.width >= 1000 ? 1000 : Overlay.overlay.width
    height: Overlay.overlay.height >= 1000 ? 1000 : Overlay.overlay.height
    anchors.centerIn: Overlay.overlay

    modal: true
    visible: true

    closePolicy: Popup.CloseOnEscape

    background: Rectangle {

        radius: 10
        color: SkrTheme.pageBackground

    }


    contentItem: SkrPane {
        anchors.fill: parent
        clip: true
        ColumnLayout {
            anchors.fill: parent

            SkrToolButton {
                id: goBackButton
                text: qsTr("Go Back")
                focusPolicy: Qt.NoFocus
                display: AbstractButton.IconOnly
                Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                icon {
                    source: "qrc:///icons/backup/go-previous.svg"
                }

                onClicked: root.close()
            }

            SwipeView {
                id: stackView
                Layout.fillHeight: true
                Layout.fillWidth: true


                onCurrentIndexChanged: {
                    var i = 0
                    for(i = 0; i < stackView.count; i++){

                        stackView.itemAt(i).enabled = i === stackView.currentIndex
                    }
                }

                ColumnLayout {
                    id: welcome

                    SkrLabel{
                        Layout.alignment: Qt.AlignHCenter
                        text: qsTr("<h1>First steps with Skribisto</h1>")
                    }

                    Image {
                        id: skribistoImage
                        Layout.alignment: Qt.AlignHCenter
                        source: "qrc:///pics/skribisto.png"
                        Layout.preferredHeight: 200
                        Layout.preferredWidth: 200
                    }

                    SkrLabel{
                        text: qsTr("Welcome to Skribisto !")
                    }

                    Item {
                        Layout.fillHeight: true
                    }
                }


                ColumnLayout {
                    id: shortcutsPage

                }


                ColumnLayout {
                    id: pluginsPage

                }

                NewProjectPage {
                    id: newProjectPage

                }



            }

            RowLayout{

                Layout.fillWidth: true

                SkrToolButton {
                    id: previousButton
                    text: qsTr("Previous")
                    display: AbstractButton.IconOnly
                    Layout.alignment: Qt.AlignLeft
                    icon {
                        source: "qrc:///icons/backup/go-previous.svg"
                    }

                    onClicked: {
                        stackView.decrementCurrentIndex()
                    }

                }
                Item{
                    Layout.fillWidth: true

                PageIndicator {
                    count: stackView.count
                    currentIndex: stackView.currentIndex
                    interactive: true
                    anchors.centerIn: parent
                    onCurrentIndexChanged: {
                        stackView.currentIndex = currentIndex
                    }
                }

}


                SkrToolButton {
                    id: nextButton
                    text: qsTr("Next")
                    display: AbstractButton.TextBesideIcon
                    Layout.alignment: Qt.AlignRight
                    icon {
                        source: "qrc:///icons/backup/go-next.svg"
                    }

                    onClicked: {
                        stackView.incrementCurrentIndex()
                    }

                }

            }


        }

    }

}
