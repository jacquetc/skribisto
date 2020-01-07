import QtQuick 2.12
import QtQml.Models 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

DocumentListViewForm {
    id: root

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
