import QtQuick 2.4
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12

TreeViewForm {

    property var model
    onModelChanged: {
        listView.model = model
    }

    signal open(int projectId, int paperId)
    signal remove(int projectId, int paperId)
    signal clearBin
    signal addAfter(int projectId, int paperId)
}
