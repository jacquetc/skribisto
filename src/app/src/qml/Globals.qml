pragma Singleton

import QtQuick 2.12

QtObject {
    property color mainbg: 'red'
    property bool compactSize: false

    property int height
    property int width
    //readonly property int  compactHeightLimit: 700
    readonly property int compactWidthLimit: 800

    //onHeightChanged: height < 700 ? compactSize = true : compactSize = false
    onWidthChanged: {
        width < compactWidthLimit ? compactSize = true : compactSize = false
        // width < compactWidthLimit ? console.log("compact = true") : console.log("compact = false")
    }
    onCompactSizeChanged: console.log("compact = " + compactSize)



    //Write :
    signal openSheetCalled(int projectId, int paperId)

    //FullScreen
    signal fullScreenCalled(bool value)
}
