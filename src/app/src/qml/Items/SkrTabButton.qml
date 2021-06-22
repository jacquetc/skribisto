import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import ".."

SkrTabButtonForm {
    id: control
    Material.background: SkrTheme.buttonBackground
    Material.foreground: SkrTheme.buttonForeground
    Material.accent: SkrTheme.accent

    SkrFocusIndicator {
        parent: control.background
        anchors.fill: control.background
        visible: control.activeFocus & Globals.focusVisible

    }
    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Tab){
            Globals.setFocusTemporarilyVisible()
        }
    }

    property string tip
    hoverEnabled: true

    SkrToolTip {
        text: control.tip ? control.tip : control.text
        visible: control.hovered && text.length !== 0
    }


    property string pageType : "undefined"
    property int projectId : -2
    property int paperId : -2
    readonly property string tabId: {return pageType + "_" +  projectId + "_" + paperId }

    function setTitle(newTitle) {

        control.text = newTitle
    }

    Accessible.name: control.text === "" ? action.text : control.text


    signal onCloseCalled(int index)
    closeButton.onClicked:  onCloseCalled(TabBar.index)



    readonly property bool isCurrent:  {
        if (TabBar.tabBar !== null) {
            return TabBar.index === TabBar.tabBar.currentIndex
        }
        return false
    }


    property bool textVisible: true
    onTextVisibleChanged: {
        tabLabel.visible = textVisible
    }

    Item{
        id: clickZone
        z:1
        anchors.fill: parent
        TapHandler{
            onSingleTapped: function(eventPoint) {
                control.toggle()
                control.clicked()
            }
        }
    }


}
