import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15

import "Items"

Item {
    id: base
    width: 400
    height: 400




    property alias horizontalSplitView: horizontalSplitView
    property alias leftVerticalSplitView: leftVerticalSplitView
    property alias rightVerticalSplitView: rightVerticalSplitView
    property alias loader_top_left: loader_top_left
    property alias loader_bottom_left: loader_bottom_left
    property alias loader_top_right: loader_top_right
    property alias loader_bottom_right: loader_bottom_right

    property real minimum: 300


    SplitView {
        id: horizontalSplitView
        orientation: Qt.Horizontal
        anchors.fill: parent

        handle: SkrSplitViewHandler {
            orientation: Qt.Vertical
        }

        SplitView {
            id: leftVerticalSplitView
            orientation: Qt.Vertical
            property var preferredWidth: undefined

            SplitView.minimumHeight: minimum
            SplitView.minimumWidth: minimum
            SplitView.preferredWidth: preferredWidth

            handle: SkrSplitViewHandler {
                orientation: Qt.Horizontal
            }

            Loader {
                id: loader_top_left
                property var preferredHeight: undefined
                SplitView.minimumHeight: minimum
                SplitView.minimumWidth: minimum
                SplitView.preferredHeight: preferredHeight
            }



            Loader {
                id: loader_bottom_left
                SplitView.minimumHeight: minimum
                SplitView.minimumWidth: minimum

                visible: false
            }

        }



        SplitView {
            id: rightVerticalSplitView
            orientation: Qt.Vertical

            SplitView.minimumHeight: minimum

            visible: false


            handle: SkrSplitViewHandler {
                orientation: Qt.Horizontal
            }


            Loader {
                id: loader_top_right
                property var preferredHeight: undefined
                SplitView.minimumHeight: minimum
                SplitView.minimumWidth: minimum
                SplitView.preferredHeight: preferredHeight
                visible: false
            }



            Loader {
                id: loader_bottom_right
                SplitView.minimumHeight: minimum
                SplitView.minimumWidth: minimum
                visible: false
            }
        }
    }






}
