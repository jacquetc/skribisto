import QtQuick 2.9
import "."

WritePageForm {

    writingZone.textAreaWidth: 800
    writingZone.stretch: false
    writingZone.minimapVisibility: false
    writingZone.text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas et eros porta, dictum mi non, gravida massa. Quisque molestie, tellus ac aliquam tincidunt, lorem mauris bibendum quam, vitae posuere augue ligula sit amet eros. Maecenas sed neque vestibulum est congue laoreet. Quisque ac massa ullamcorper, dapibus lorem et, lobortis libero. Sed tempor mauris quis efficitur imperdiet. Phasellus eget nulla at mauris finibus varius. Nunc porta laoreet ante, sed mattis turpis elementum ut. Ut et erat et odio interdum vestibulum quis nec mi. Aliquam malesuada nisi eget elit blandit, nec aliquet ex vulputate. Aenean consequat quam diam, id tincidunt ligula porta sodales. \nNam viverra scelerisque lectus eu dapibus. Sed sed neque leo. Suspendisse ultricies libero justo, a porta erat mollis eu. Morbi sed quam eget odio vulputate venenatis. Integer a justo et dui consequat vehicula. Vestibulum quis facilisis risus. Donec pretium commodo ultrices. Vivamus rhoncus erat nisl, pharetra dictum ipsum ultricies at. Nulla at lectus et sem egestas finibus. Nunc augue arcu, bibendum ac est nec, laoreet facilisis augue. Vivamus id cursus ex. Aenean placerat, quam eget mattis tincidunt, lacus velit consectetur enim, vel scelerisque urna dolor sodales dui. \nFusce semper fermentum dolor quis venenatis. Vestibulum scelerisque, sem eget luctus pulvinar, lacus tellus facilisis erat, vitae sagittis lectus risus ut nibh. Vivamus sagittis dui ligula, id pulvinar libero blandit ut. Donec massa risus, commodo sit amet lectus ut, porta consectetur lacus. Donec sit amet ipsum volutpat, accumsan mauris bibendum, volutpat est. Nullam egestas purus at mauris fermentum mollis. Donec sit amet rutrum dui. Nulla elementum vulputate est quis porta. Nunc id lacus eget tellus efficitur aliquam quis et augue. Maecenas finibus fringilla elementum. Maecenas suscipit, quam sed consectetur egestas, augue augue euismod nulla, sed sollicitudin enim dui eget tellus. Nulla accumsan faucibus volutpat. Nullam lacinia pharetra condimentum. Donec maximus vel dolor id molestie. Morbi varius, dui malesuada pretium semper, ligula nulla pharetra erat, a hendrerit ante ante non felis. Nunc molestie cursus erat, id semper leo luctus vitae. \nIn consectetur dictum ornare. Curabitur luctus ligula magna, nec feugiat magna lobortis eu. Sed aliquam et urna sed sodales. Proin quis condimentum urna, vitae convallis metus. Phasellus dictum lectus non erat porta, eget fermentum velit lobortis. Nunc vitae lorem lectus. Quisque velit lacus, pretium eu iaculis sed, dapibus vitae elit. Integer imperdiet odio in orci porta, nec scelerisque justo euismod. Aliquam enim libero, porta in risus sit amet, aliquet mollis augue. Nullam sed augue facilisis, volutpat metus eu, volutpat nisl. \nNulla bibendum neque vitae turpis vestibulum rhoncus. Pellentesque ut ligula libero. Duis felis ante, pretium at interdum ac, varius a ante. Fusce venenatis dolor ut posuere molestie. Morbi in suscipit tellus. Cras pellentesque sapien velit. Aliquam erat volutpat. Vivamus consectetur augue nunc, sit amet facilisis turpis dapibus at. Integer finibus tincidunt quam sit amet tempor. Morbi hendrerit mollis eros quis hendrerit. Ut erat nisl, sodales at enim nec, viverra tincidunt erat. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed id porttitor risus. Vivamus quis enim nulla. Phasellus sed posuere purus. Ut sit amet auctor erat. "

    Component.onCompleted: {
        //utreedock.createBody("qrc:/qml/WriteTreeView.qml")

    }




    resizeHandleMouseArea.onContainsMouseChanged: resizeHandleMouseArea.containsMouse ? resizeHandleMouseArea.cursorShape = Qt.SizeHorCursor : resizeHandleMouseArea.cursorShape = Qt.ArrowCursor

    resizeHandleMouseArea.drag.target: resizeHandle
    resizeHandleMouseArea.drag.axis: Drag.XAxis
    resizeHandleMouseArea.drag.minimumX: 0
    resizeHandleMouseArea.drag.maximumX: base.width / 2
    resizeHandleMouseArea.drag.smoothed: false
    resizeHandleMouseArea.onPressed: {
        resizeHandle.anchors.left = undefined
        resizeHandle.anchors.right = undefined
        //resizeHandle.color = "#ffa369"
    }
    resizeHandleMouseArea.onReleased:  {

        writingZone.textAreaWidth = (base.width/2 - (resizeHandle.x + resizeHandle.width)) * 2
        resizeHandle.anchors.left = resizeHandle.parent.left
        resizeHandle.anchors.right = resizeHandle.parent.left
    }

}

