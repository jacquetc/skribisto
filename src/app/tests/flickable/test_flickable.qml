import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml.Models 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchtreelistproxymodel 1.0
import QtQuick.Controls.Material 2.15
import "qrc:///qml/Commons"
import "qrc:///qml/Items"
import "qrc:///qml/"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 500
    minimumWidth: 500
    visible: true


    Component.onCompleted:{

    }

    Rectangle{
        id: base
        //anchors.fill: parent
        height: 500
        width: 500

        border.color: "black"
        border.width: 1

        SkrFlickable{
            id: flickable
            anchors.fill: parent

            contentWidth: 400

            contentItem: Rectangle{
                width: flickable.contentWidth
                height: 1000

                gradient: Gradient {
                    GradientStop {
                        position: 0.00;
                        color: "#ffffff";
                    }
                    GradientStop {
                        position: 1.00;
                        color: "#000000";
                    }
                }

            }
        }

    }

}
