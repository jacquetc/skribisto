import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

ExamplePageForm {


    repeater.model: ListModel{
        ListElement{
            title: "Starforgers"
            author: "Ken McConnell"
            imageSource: "qrc:///examples/Starforgers.jpg"
            skribistoFile: "qrc:///examples/Starforgers.skrib"
        }

    }


    repeater.delegate:
        SkrListItemPane {
            id: noteSearchItem
            padding: 5
            elevation: 5
            Layout.preferredWidth: 300
            Layout.preferredHeight: 400

            focus: true

            Accessible.role: Accessible.Button
            Accessible.name: model.title + " " + model.author

            TapHandler {
                onTapped: {
                    //TODO: temporary until async is done
                    Globals.loadingPopupCalled()
                    //plmData.projectHub().loadProject(model.skribistoFile)
                    loadProjectTimer.start()
                }

                onGrabChanged: {
                    point.accepted = false
                }
            }

            //TODO: temporary until async is done
            Timer {
                id: loadProjectTimer
                repeat: false
                interval: 100
                onTriggered: {
                    plmData.projectHub().loadProject(model.skribistoFile)

                }

            }

            contentItem: ColumnLayout{
                anchors.fill: parent

                Image {
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    id: name
                    source: model.imageSource
                    fillMode: Image.PreserveAspectFit
                }



                SkrLabel{
                    text: model.title
                    Layout.fillWidth: true
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignVCenter
                    font.bold: true
                }

                SkrLabel{
                    text: model.author
                    Layout.fillWidth: true
                    horizontalAlignment: Qt.AlignHCenter
                    verticalAlignment: Qt.AlignVCenter

                }


            }



        }

}
