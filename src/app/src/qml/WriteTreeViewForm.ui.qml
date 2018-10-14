import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3

Item {
    id: item1
    width: 400
    height: 400



    ColumnLayout {
        id: column1
        anchors.fill: parent

//        BreadCrumb {
//            id: breadCrumb
//            Layout.fillWidth: true
//            Layout.fillHeight: false
//            Layout.preferredHeight: 40
//            //Layout.maximumHeight: 40
//        }

        ListView {
            id: listView
            Layout.fillWidth: false
            Layout.fillHeight: true
            delegate: ItemDelegate {
                x: 5
                width: 80
                height: 40
                Row {
                    id: row1
                    spacing: 10
                    Image {
                        id: image
                        width: 40
                        height: 40
                        fillMode: Image.PreserveAspectFit
                        source: "qrc:/pics/author.svg"
                    }

                    Item {
                        id: item3
                        width: 200
                        height: 50

                        Column {
                            id: column
                            anchors.fill: parent

                            Text {
                                text: name
                                font.bold: true
                            }

                            Text {
                                id: text1
                                text: tag
                                font.pixelSize: 12
                            }
                        }
                    }
                }
            }
            model: ListModel {
                ListElement {
                    name: "Grey"
                    colorCode: "grey"
                    tag: 'ee'
                }

                ListElement {
                    name: "Red"
                    colorCode: "red"
                    tag: 'ee'
                }

                ListElement {
                    name: "Blue"
                    colorCode: "blue"
                    tag: 'ee'
                }

                ListElement {
                    name: "Green"
                    colorCode: "green"
                    tag: 'ee'
                }
            }
        }

    }
}
