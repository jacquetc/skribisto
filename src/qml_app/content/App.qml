/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
//import QCoro 0
import QtQml
import QtQuick.Controls.Basic
// Components
import Skr.Controllers

// import Models
import Skr.Singles

ApplicationWindow {
    id: applicationWindow

    height: 480
    title: "FrontEndsExample"
    visible: true
    width: 640

    // palette: customPalette
    // ColorGroup {
    //     id: activeCG
    //     window: Qt.color("#ffffff")
    //     button: Qt.color("#dddddd")
    //     buttonText: Qt.color("#000000")
    //     text: Qt.color("#705890")
    // }
    // ColorGroup {
    //     id: inactiveCG
    //     window: Qt.color("#ffffff")
    //     button: Qt.color("#f85aaa")
    //     buttonText: Qt.color("#000000")
    //     text: Qt.color("#705890")
    // }
    // Palette {
    //     id: customPalette
    //     active: activeCG
    //     inactive: inactiveCG
    // }
    palette {
        active {
            buttonText: "#3700B3"
        }
        inactive {
            buttonText: "yellow"
        }
        disabled {
            buttonText: colors.buttonText
        }
    }
    QtObject {
        id: colors

        property string buttonText: "salmon"
        property string primaryVariant: "#3700B3"

        // onButtonTextChanged: {
        //     console.log("Button text color changed to:", buttonText)
        //     applicationWindow.palette.disabled.butonText = buttonText
        // }
    }

    // Main layout
    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        // Header
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            // Title
            Text {
                font.pixelSize: 20
                text: "Hello World"
            }

            // Spacer
            Item {
                Layout.fillWidth: true
            }
            RootController {
                id: rootController

            }
            WorkManagementController {
                id: workManagementController

            }
            WorkController {
                id: workController

            }
            SingleBinderItem {
                id: binderItem1

                itemId: 1
            }
            // Button
            Button {
                id: savekButton

                text: "Save"

                onClicked: {
                    console.log("Save button clicked");
                    let dto = workManagementController.getSaveWorkDto();
                    dto.fileName = "/tmp/mywork.skr";

                    workManagementController.saveWork(dto).then(function (result) {
                        console.log("Async save result :", result);
                    });
                }
            }
            // Button
            Button {
                id: createWorkButton

                text: "Create Work"

                onClicked: {
                    colors.buttonText = "blue";
                    console.log("Create button clicked");
                    var dto = workController.getCreateDto();
                    dto.title = "My Work ";

                    workController.create([dto]).then(function (result) {
                        console.log("Async work creation result :", result);
                    });
                }
            }
            // Button
            Button {
                id: createRootButton

                enabled: true
                text: "Create Root"

                onClicked: {
                    console.log("Create button clicked");

                    var dto = rootController.getCreateDto();
                    dto.works = [1];

                    rootController.create([dto]).then(function (result) {
                        console.log("Async root creation result :", result);
                    });
                }
            }
            // Button
            Button {
                id: getRootButton

                text: "Get Root 1"

                onClicked: {
                    console.log("Get Root 1 button clicked");

                    rootController.get([1]).then(function (res) {
                        console.log("Async get root result :", res);
                    });
                }
            }
            // Button
            Button {
                id: getWorkButton

                text: "Get Work 1"

                onClicked: {
                    console.log("Get Work 1 button clicked");

                    workController.get([1]).then(function (res) {
                        console.log("Async get work result :", res);
                    });
                }
            }
            Button {
                id: removeRootButton

                text: "Remove Root 1"

                onClicked: {
                    console.log("remove root 1 clicked");

                    rootController.remove([1]).then(function (result) {
                        console.log("Async root removal result :", result);
                    });
                }
            }
        }

        // Content
        Text {
            text: "Hello, Skribisto!"
        }
    }
}
