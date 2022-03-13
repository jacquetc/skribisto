import QtQuick
import QtQml.Models
import QtQuick.Controls
import QtQuick.Layouts

DocumentListViewForm {
    id: root
    property int minimumHeight: 300

    property var documentModel
    property var model
    onModelChanged: {
        visualModel.model = model
    }

    signal openDocument(int projectId, int paperId)
    signal closeDocument(int projectId, int paperId)

    listView.model: visualModel
    DelegateModel {
        id: visualModel

        //delegate: dragDelegate
    }
}
