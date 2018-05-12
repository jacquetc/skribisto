/*
 *   Copyright 2016 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2 or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU Library General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

import QtQuick 2.1
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.0 as QQC2
import org.kde.kirigami 2.4 as Kirigami

Kirigami.ApplicationWindow {
    id: root
    property string currentFile


    header: Kirigami.ToolBarApplicationHeader {
    }

    pageStack.initialPage: iconView

    Kirigami.ScrollablePage {
        id: iconView
        title: "Notes"
        actions {
            contextualActions: [
                Kirigami.Action {
                    id: sortAction
                    iconName: "view-sort-ascending-symbolic"
                    tooltip: "Sort Ascending"
                }
            ]
        }
        background: Rectangle {
            color: Kirigami.Thmeme.backgroundColor
        }

        GridView {
            id: view
            model: 100
            cellWidth: Kirigami.Units.gridUnit * 9
            cellHeight: cellWidth
            currentIndex: -1
            highlightMoveDuration: 0
            highlight: Rectangle {
                color: Kirigami.Theme.highlightColor
            }
            delegate: MouseArea {
                width: view.cellWidth
                height: view.cellHeight
                Kirigami.Icon {
                    source: "text-plain"
                    anchors {
                        fill: parent
                        margins: Kirigami.Units.gridUnit
                    }
                    QQC2.Label {
                        anchors {
                            top: parent.bottom
                            horizontalCenter: parent.horizontalCenter
                        }
                        text: "File " + modelData
                    }
                }
                onClicked: {
                    view.currentIndex = index;
                    root.currentFile = "File " + modelData;
                    if (root.pageStack.depth < 2) {
                        root.pageStack.push(editorComponent);
                    }
                    root.pageStack.currentIndex = 1
                }
            }
        }
    }

    Component {
        id: editorComponent
        Kirigami.ScrollablePage {
            id: editor
            title: root.currentFile
            actions {
                main: Kirigami.Action {
                        id: shareAction
                        iconName: "document-share"
                        text: "Share..."
                        tooltip: "Share this document with your device"
                        checkable: true
                        onCheckedChanged: sheet.sheetOpen = checked;
                    }
                contextualActions: [
                    Kirigami.Action {
                        iconName: "format-text-bold-symbolic"
                        tooltip: "Bold"
                    },
                    Kirigami.Action {
                        iconName: "format-text-underline-symbolic"
                        tooltip: "Underline"
                    },
                    Kirigami.Action {
                        iconName: "format-text-italic-symbolic"
                        tooltip: "Italic"
                    }
                ]
            }
            background: Rectangle {
                color: Kirigami.Theme.backgroundColor
                Rectangle {
                    anchors.fill: parent
                    color: "yellow"
                    opacity: 0.2
                }
            }

            Kirigami.OverlaySheet {
                id: sheet
                onSheetOpenChanged: shareAction.checked = sheetOpen
                ListView {
                    implicitWidth: Kirigami.Units.gridUnit * 30
                    model: ListModel {
                        ListElement {
                            title: "Share with phone \"Nokia 3310\""
                            description: "You selected this phone 12 times before. It's currently connected via bluetooth"
                            buttonText: "Push Sync"
                        }
                        ListElement {
                            title: "Share with phone \"My other Nexus5\""
                            description: "You selected this phone 0 times before. It's currently connected to your laptop via Wifi"
                            buttonText: "push sync"
                        }
                        ListElement {
                            title: "Share with NextCloud"
                            description: "You currently do not have a server set up for sharing and storing notes from Katie. If you want to set one up click here"
                            buttonText: "Setup..."
                        }
                        ListElement {
                            title: "Send document via email"
                            description: "This will send the document as an attached file to your own email for later sync"
                            buttonText: "Send As Email"
                        }
                    }
                    header: Kirigami.AbstractListItem {
                        height: Kirigami.Units.gridUnit * 6
                        hoverEnabled: false
                        RowLayout {
                            Kirigami.Icon {
                                source: "documentinfo"
                                width: Kirigami.Units.iconSizes.large
                                height: width
                            }
                            QQC2.Label {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                wrapMode: Text.WordWrap
                                text: "This document has already automatically synced with your phone \"Dancepartymeister 12\". If you want to sync with another device or do further actions you can do that here"
                            }
                        }
                    }
                    delegate: Kirigami.AbstractListItem {
                        height: Kirigami.Units.gridUnit * 6
                        hoverEnabled: false
                        //TODO: bug in overlaysheet
                        rightPadding: Kirigami.Units.gridUnit * 1.5
                        RowLayout {
                            ColumnLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                QQC2.Label {
                                    wrapMode: Text.WordWrap
                                    text: model.title
                                }
                                QQC2.Label {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    wrapMode: Text.WordWrap
                                    text: model.description
                                }
                            }
                            QQC2.Button {
                                text: model.buttonText
                                onClicked: sheet.close()
                            }
                        }
                    }
                }
            }
            QQC2.TextArea {
                background: Item {}
                wrapMode: TextEdit.WordWrap 
                selectByMouse: true
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec sollicitudin, lorem at semper pretium, tortor nisl pellentesque risus, eget eleifend odio ipsum ac mi. Donec justo ex, elementum vitae gravida vel, pretium ac lacus. Duis non metus ac enim viverra auctor in non nunc. Sed sit amet luctus nisi. Proin justo nulla, vehicula eget porta sit amet, aliquet vitae dolor. Mauris sed odio auctor, tempus ipsum ac, placerat enim. Ut in dolor vel ante dictum auctor.

    Praesent blandit rhoncus augue. Phasellus consequat luctus pulvinar. Pellentesque rutrum laoreet dolor, sit amet pellentesque tellus mattis sed. Sed accumsan cursus tortor. Morbi et risus dolor. Nullam facilisis ipsum justo, nec sollicitudin mi pulvinar ac. Nulla facilisi. Donec maximus turpis eget mollis laoreet. Phasellus vel mauris et est mattis auctor eget sit amet turpis. Aliquam dignissim euismod purus, eu efficitur neque fermentum eu. Suspendisse potenti. Praesent mattis ex vitae neque rutrum tincidunt. Etiam placerat leo viverra pulvinar tincidunt.

    Proin vel rutrum massa. Proin volutpat aliquet dapibus. Maecenas aliquet elit eu venenatis venenatis. Ut elementum, lacus vel auctor auctor, velit massa elementum ligula, quis elementum ex nisi aliquam mauris. Nulla facilisi. Pellentesque aliquet egestas venenatis. Donec iaculis ultrices laoreet. Vestibulum cursus rhoncus sollicitudin.

    Proin quam libero, bibendum eget sodales id, gravida quis enim. Duis fermentum libero vitae sapien hendrerit, in tincidunt tortor semper. Nullam quam nisi, feugiat sed rutrum vitae, dignissim quis risus. Ut ultricies pellentesque est, ut gravida massa convallis sed. Ut placerat dui non felis interdum, id malesuada nulla ornare. Phasellus volutpat purus placerat velit porta tristique. Donec molestie leo in turpis bibendum pharetra. Fusce fermentum diam vitae neque laoreet, sed aliquam leo sollicitudin.

    Ut facilisis massa arcu, eu suscipit ante varius sed. Morbi augue leo, mattis eu tempor vitae, condimentum sed urna. Curabitur ac blandit orci. Vestibulum quis consequat nunc. Proin imperdiet commodo imperdiet. Aenean mattis augue et imperdiet ultricies. Ut id feugiat nulla, et sollicitudin dui. Etiam scelerisque ligula ac euismod hendrerit. Integer in quam nibh. Pellentesque risus massa, porttitor quis fermentum eu, dictum varius magna. Morbi euismod bibendum lacus efficitur pretium. Phasellus elementum porttitor enim nec dictum. Morbi et augue laoreet, convallis quam quis, egestas quam."
            }
        }
    }
}
