import QtQuick 2.4

WritingZoneForm {
    property int textAreaWidth: textAreaWidth
    property bool stretch: true


//    onTextAreaWidthChanged: {
//        if(stretch === true){
//            text_base.width = 100
//            text_base.Layout.minimumWidth = 100
//            text_base.Layout.fillWidth = true
//        }
//        if(stretch === false){
//            text_base.width = textAreaWidth
//            text_base.Layout.minimumWidth = textAreaWidth
//            text_base.Layout.fillWidth = false
//        }
//    }
}
