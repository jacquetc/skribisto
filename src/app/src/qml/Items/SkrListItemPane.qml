import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

Item {
    property int elevation: 0
    property int borderWidth: 0
    property int padding: 0
    property string borderColor: "transparent"
    property alias contentItem : mainRectangle.data

    Material.elevation: elevation

    Rectangle {
        id: mainRectangle
        anchors.fill: parent
        anchors.margins: padding
        color: SkrTheme.listItemBackground
        border.width: borderWidth
        border.color: borderColor



    }

    Rectangle {
        id: bottomShadow
        anchors.top: mainRectangle.bottom
        anchors.left: mainRectangle.left
        anchors.right: mainRectangle.right
        height: 3 + elevation

        gradient: Gradient {
            orientation: Qt.Vertical
            GradientStop {
                position: 1.00;
                color: "transparent";
            }
            GradientStop {
                position: 0.20;
                color: "#d6d6d6";
            }
            GradientStop {
                position: 0.10;
                color: "#b5b5b5";
            }
            GradientStop {
                position: 0.00;
                color: "#b5b5b5";
            }

        }
    }

    Rectangle {
        id: leftShadow
        anchors.top: mainRectangle.top
        anchors.right: mainRectangle.left
        anchors.bottom: mainRectangle.bottom
        width: 3 + elevation

        gradient: Gradient {
            orientation: Qt.Horizontal
            GradientStop {
                position: 0.00;
                color: "transparent";
            }
            GradientStop {
                position: 0.80;
                color: "#d6d6d6";
            }
            GradientStop {
                position: 0.90;
                color: "#b5b5b5";
            }
            GradientStop {
                position: 1.00;
                color: "#b5b5b5";
            }

        }

    }

    Rectangle {
        id: rightShadow
        anchors.top: mainRectangle.top
        anchors.left: mainRectangle.right
        anchors.bottom: mainRectangle.bottom
        width: 3 + elevation


        gradient: Gradient {
            orientation: Qt.Horizontal
            GradientStop {
                position: 1.00;
                color: "transparent";
            }
            GradientStop {
                position: 0.20;
                color: "#d6d6d6";
            }
            GradientStop {
                position: 0.10;
                color: "#b5b5b5";
            }
            GradientStop {
                position: 0.00;
                color: "#b5b5b5";
            }

        }

    }
}
