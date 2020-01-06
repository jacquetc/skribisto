import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ListView {
    id: listView
    antialiasing: true
    clip: true
    smooth: true
    boundsBehavior: Flickable.StopAtBounds

    //    highlightMoveVelocity: 2000
    //    highlight: Rectangle {
    //        width: 200
    //        height: 40
    //        radius: 7
    //        y: listView.currentItem.y
    //        //        Behavior on y {
    //        //            SpringAnimation {
    //        //                spring: 2
    //        //                damping: 0.1
    //        //            }
    //        //        }
    //        border.color: "black"
    //        border.width: 2
    //        color: "red"
    //    }
    //    highlightFollowsCurrentItem: true
    //    Keys.onDownPressed: listView.incrementCurrentIndex()
    //    Keys.onUpPressed: listView.decrementCurrentIndex()
    //    Shortcut {
    //        sequence: "Down"
    //        onActivated: listView.incrementCurrentIndex()
    //    }

}
