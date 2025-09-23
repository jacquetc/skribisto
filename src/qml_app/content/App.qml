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

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QCoro 0

// Components
import Skribisto.App
import Controllers
// import Models
// import Singles

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "FrontEndsExample"


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
                text: "Hello World"
                font.pixelSize: 20
            }

            // Spacer
            Item {
                Layout.fillWidth: true
            }

            RootController {
                id: rootController
            }
            ProjectController {
                id: projectController
            }
            // Button
            Button {
                id: createProjectButton
                text: "Create Project"
                onClicked: {
                    console.log("Create button clicked")
                    var dto = projectController.getCreateDto()
                    dto.title = "My Project "

                    projectController.create([dto]).then(function (result) {
                        console.log("Async project creation result :", result)
                    })
                }
            }
            // Button
            Button {
                id: createRootButton
                text: "Create Root"
                onClicked: {
                    console.log("Create button clicked")
                    var dto = rootController.getCreateDto()
                    dto.projects = [1]

                    rootController.create([dto]).then(function (result) {
                        console.log("Async root creation result :", result)
                    })
                }
            }
            // Button
            Button {
                id: getRootButton

                text: "Get Root 1"
                onClicked: {
                    console.log("Get Root 1 button clicked")

                    rootController.get([1]).then(function (res) {
                        console.log("Async get root result :", res)
                    })
                }
            }
            // Button
            Button {
                id: getProjectButton

                text: "Get Project 1"
                onClicked: {
                    console.log("Get Root 1 button clicked")

                    projectController.get([1]).then(function (res) {
                        console.log("Async get project result :", res)
                    })
                }
            }

            Button {
                id: removeRootButton
                text: "Remove Root 1"
                onClicked: {
                    console.log("remove root 1 clicked")

                    rootController.remove([1]).then(function (result) {
                        console.log("Async root removal result :", result)
                    })
                }
            }


        }

        // Content
        Text {
            text: "Hello, Skribisto!"
        }
    }
}