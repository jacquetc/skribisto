import QtQuick
import backend

Item {
    id: item
    required property string name
    required property string group
    property int priority: 0
    required property string sequence
    required property bool hasFocus
    signal activated()

    ShortcutBackend {
        id: backend
        name: item.name
        group: item.group
        sequence: item.sequence
        enabled: item.enabled
        visible: item.visible
        hasFocus: item.hasFocus
        onActivated: item.activated()
    }

    Shortcut {
        id: shortcut
        sequence: item.sequence
        onActivatedAmbiguously: backend.processAmbiguousActivation()
        onActivated: item.activated()
    }
}
