import QtQuick
import QtQuick.Controls
import Models
import QtQuick.Shapes

BinderPageForm {
    // -------------- Tab bar :

    tabBarReapeater.model: DividerListModelFromBinderDividers{}
    tabBarReapeater.delegate:
        Rectangle{
        id: backgroundOfNextTab
        width: tabLabel.contentWidth + 30
        height: 30
        // use rectColor of the next irem in the model
        z: tabBarReapeater.model.count - model.index


        readonly property color backgoundColor: model.backgroundColor
        readonly property color foregroundColor: model.foregroundColor
        readonly property int leftMargin: model.index === 0 ? 5 :  40

        Shape {
            id: tabButton
            anchors.fill: parent


            ShapePath {

                fillColor: backgroundColor
                strokeColor: "black"
                strokeWidth: 1
                startX: 0
                startY: 0
                property real radius : backgroundOfNextTab.height / 1.2

                PathLine { x: backgroundOfNextTab.width ; y: 0 }  // Draw to the start of the curve
                PathArc { x: backgroundOfNextTab.width + 30; y:  backgroundOfNextTab.height / 1.3; radiusX: 31; radiusY: 25; useLargeArc: false }  // Rounded corner
                PathLine { x: backgroundOfNextTab.width + 30; y: backgroundOfNextTab.height }  // Down to the bottom
                PathLine { x: 0; y: backgroundOfNextTab.height }  // All the way to the left
                PathLine { x: 0; y: 0 }  // Close the shape by returning to the top-left
            }


            Label {
                id: tabLabel
                anchors.fill: parent
                anchors.leftMargin: leftMargin
                anchors.topMargin: 5
                text: model.title
                color: foregroundColor
            }
        }
    }

    // -------------- Notes :


    noteListView.model: NoteListModelFromDividerNotes{ dividerId: tabBarReapeater.currentItem ? tabBarReapeater.currentItem.model.itemId : 0}

    noteListView.delegate: ItemDelegate {
        text: model.title
        width: ListView.width
    }

    Component {
        id: sectionHeading
        Rectangle {
            width: container.width
            height: childrenRect.height
            color: "lightsteelblue"

            required property string section

            Text {
                text: parent.section
                font.bold: true
                font.pixelSize: 20
            }
        }
    }
    noteListView.section.property: "group"
    noteListView.section.criteria: ViewSection.FullString
    noteListView.section.delegate: sectionHeading
}
