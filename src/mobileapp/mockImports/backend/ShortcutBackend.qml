import QtQuick

QtObject {
    property string name: ""
    property string group: ""
    property string sequence: ""
    property string finalSequence: ""
    property int priority: 0
    property bool enabled: true
    property bool visible: true
    signal activated()

    function processAmbiguousActivation(){}
}
