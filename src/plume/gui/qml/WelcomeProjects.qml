import QtQuick 2.5

WelcomeProjectsForm {


    function init(){
        //leftBase.onBaseWidthChanged.connect(changeLeftBaseWidth)
        //rightBase.onBaseWidthChanged.connect(changeRightBaseWidth)
    }

    recent_list_view.model: project_list_model
    recent_list_view.delegate: listDelegate



    Component {
                 id: listDelegate
                 Rectangle {
                         id:wrapper
                         width: recent_list_view.width
                         height: 70
                         color: /*wrapper.ListView.isCurrentItem |*/ mouseArea.containsMouse? "#ff7b00" : "transparent"

                         Text {
                             text: title
                             anchors.top: parent.top
                             anchors.topMargin: 8
                             anchors.left: parent.left
                             anchors.leftMargin: 21
                             font.bold: true
                         }

                         Text {

                             text: path
                             elide: Text.ElideLeft
                             anchors.bottom: parent.bottom
                             anchors.bottomMargin: 8
                             anchors.left: parent.left
                             anchors.leftMargin: 19
                             anchors.right: parent.right
                             anchors.rightMargin: 19
                         }

                         Text {

                             text: Qt.formatDate(last_opened_date, Qt.DefaultLocaleShortDate)
                             anchors.top: parent.top
                             anchors.topMargin: 8
                             anchors.right: parent.right
                             anchors.rightMargin: 8
                         }
                         MouseArea { id: mouseArea;
                             anchors.fill: parent;
                             hoverEnabled: true;
                             onClicked: recent_list_view.currentIndex = index }

                 }

    }



    Component.onCompleted: init()
}
