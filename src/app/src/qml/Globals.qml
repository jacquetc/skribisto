pragma Singleton
import QtQuick 2.9

QtObject {
    property color mainbg: 'red'
    property bool compactSize: false

    property int height
    property int  width
    //readonly property int  compactHeightLimit: 700
    readonly property int  compactWidthLimit: 700


    //onHeightChanged: height < 700 ? compactSize = true : compactSize = false


    onWidthChanged: {width < compactWidthLimit ? compactSize = true : compactSize = false
       // width < compactWidthLimit ? console.log("compact = true") : console.log("compact = false")
    }
    onCompactSizeChanged: console.log("compact = " +  compactSize )



}
