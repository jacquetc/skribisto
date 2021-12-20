import QtQuick
import ".."

Item {
    id: root

    Canvas{
        anchors.fill: parent
        onPaint:{
            var ctx = getContext("2d");
            ctx.setLineDash([2, 5]);
            ctx.strokeStyle = SkrTheme.buttonForeground
            ctx.beginPath();
            ctx.roundedRect(0, 0, root.width, root.height, 4, 4)
            ctx.stroke();
         }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
