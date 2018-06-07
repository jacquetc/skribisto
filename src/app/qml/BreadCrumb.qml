import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Universal 2.2
import QtQml.Models 2.11

BreadCrumbForm {

    view.model: ListModel {
        id: crumbModel
    }
    view.delegate: Button {
        y:5
        height: parent.height-15
        //property bool hovered: false

        text: location

        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            //hoveredChanged: {hoveredChanged() ?  parent.hovered = true :  parent.hovered = false}
        }
    }

    function loadLocation(path) {
        var pathArray = path

        view.model.clear()
        for (var index in pathArray) {
            var data = {'location': pathArray[index]};
            view.model.append(data)
        }

        view.positionViewAtEnd()
    }
    Component.onCompleted: {
        loadLocation(["un",  "deux", "trois"])
    }

}

