import QtQuick
import QtQuick.Controls

FocusScope {
    id: control

    required property var viewManager
    required property int position
    property int projectId: -1
    property int treeItemId: -1
    property string pageType: ""
    property var additionalPropertiesForSavingView: ({})
    property bool dropAreaEnabled: true
    property list<Component> toolboxes

    clip: true

    signal closeViewCalled
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

