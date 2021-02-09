import QtQuick 2.15
import QtQml.Models 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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
