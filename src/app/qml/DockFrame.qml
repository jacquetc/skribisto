import QtQuick 2.4
import "dockFrame.js" as DockFrameJS

DockFrameForm {


    function createBody(qmlPath) {
        DockFrameJS.createDockBody(qmlPath, dockBody)
    }



}
