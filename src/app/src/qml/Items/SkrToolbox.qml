import QtQuick 2.15

Item {
    id: control
    clip: true

    property string iconSource
    property string showButtonText
    property string name
    property bool visibleByDefault: true

    implicitHeight: children[0].implicitHeight
}
