import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

TabForm {
    id: root
    property string pageType : "undefined"
    property int projectId : -2
    property int paperId : -2
    readonly property string tabId: {return pageType + "_" +  projectId + "_" + paperId}

    function setTitle(newTitle) {

        root.text = newTitle
    }

    signal onCloseCalled(int index)
    closeButton.onClicked:  onCloseCalled(TabBar.index)



    readonly property bool isCurrent:  {
        if (TabBar.tabBar !== null) {
            return TabBar.index === TabBar.tabBar.currentIndex
        }
        return false
    }

}
