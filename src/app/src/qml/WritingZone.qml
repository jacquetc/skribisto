import QtQuick 2.11

WritingZoneForm {
    textAreaWidth: 400
    //property int textAreaLeftPos:
    stretch: true
    minimapVisibility: false
    readonly property int textAreaLeftPos: base.width / 2 - textAreaWidth / 2
    readonly property int textAreaRightPos: base.width / 2 + textAreaWidth / 2

}
