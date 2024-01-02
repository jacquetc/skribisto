import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Models

ColumnLayout{
    id: root
    spacing: 0
    ToolBar{
        id: toolbar
        Layout.fillWidth: true


        Row{
            ToolButton{
                text: "+"
                onClicked: {
                    stackView.pop()
                }
            }
        }
    }

    ListView {
        id: listView
        Layout.fillWidth: true
        Layout.fillHeight: true

        model: BookChaptersListModel {}
        delegate: ItemDelegate {
            text: model.title
            width: parent.width
            height: 30
            highlighted: listView.currentIndex == index
            onClicked: {
                listView.currentIndex = index
                UiSignals.chapterCalledFromSideList(model.itemId)
            }
        }

    }
}

