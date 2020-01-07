import QtQuick 2.12
import QtQml 2.12
import QtQuick.Controls 2.12
import eu.skribisto.sheethub 1.0
import ".."

NotesPageForm {
    property int textAreaFixedWidth: 400

    writingZone.textAreaWidth: 400
    writingZone.stretch: Globals.compactSize
    writingZone.minimapVisibility: true

    //writingZone.text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas et eros porta, dictum mi non, gravida massa. Quisque molestie, tellus ac aliquam tincidunt, lorem mauris bibendum quam, vitae posuere augue ligula sit amet eros. Maecenas sed neque vestibulum est congue laoreet. Quisque ac massa ullamcorper, dapibus lorem et, lobortis libero. Sed tempor mauris quis efficitur imperdiet. Phasellus eget nulla at mauris finibus varius. Nunc porta laoreet ante, sed mattis turpis elementum ut. Ut et erat et odio interdum vestibulum quis nec mi. Aliquam malesuada nisi eget elit blandit, nec aliquet ex vulputate. Aenean consequat quam diam, id tincidunt ligula porta sodales. \nNam viverra scelerisque lectus eu dapibus. Sed sed neque leo. Suspendisse ultricies libero justo, a porta erat mollis eu. Morbi sed quam eget odio vulputate venenatis. Integer a justo et dui consequat vehicula. Vestibulum quis facilisis risus. Donec pretium commodo ultrices. Vivamus rhoncus erat nisl, pharetra dictum ipsum ultricies at. Nulla at lectus et sem egestas finibus. Nunc augue arcu, bibendum ac est nec, laoreet facilisis augue. Vivamus id cursus ex. Aenean placerat, quam eget mattis tincidunt, lacus velit consectetur enim, vel scelerisque urna dolor sodales dui. \nFusce semper fermentum dolor quis venenatis. Vestibulum scelerisque, sem eget luctus pulvinar, lacus tellus facilisis erat, vitae sagittis lectus risus ut nibh. Vivamus sagittis dui ligula, id pulvinar libero blandit ut. Donec massa risus, commodo sit amet lectus ut, porta consectetur lacus. Donec sit amet ipsum volutpat, accumsan mauris bibendum, volutpat est. Nullam egestas purus at mauris fermentum mollis. Donec sit amet rutrum dui. Nulla elementum vulputate est quis porta. Nunc id lacus eget tellus efficitur aliquam quis et augue. Maecenas finibus fringilla elementum. Maecenas suscipit, quam sed consectetur egestas, augue augue euismod nulla, sed sollicitudin enim dui eget tellus. Nulla accumsan faucibus volutpat. Nullam lacinia pharetra condimentum. Donec maximus vel dolor id molestie. Morbi varius, dui malesuada pretium semper, ligula nulla pharetra erat, a hendrerit ante ante non felis. Nunc molestie cursus erat, id semper leo luctus vitae. \nIn consectetur dictum ornare. Curabitur luctus ligula magna, nec feugiat magna lobortis eu. Sed aliquam et urna sed sodales. Proin quis condimentum urna, vitae convallis metus. Phasellus dictum lectus non erat porta, eget fermentum velit lobortis. Nunc vitae lorem lectus. Quisque velit lacus, pretium eu iaculis sed, dapibus vitae elit. Integer imperdiet odio in orci porta, nec scelerisque justo euismod. Aliquam enim libero, porta in risus sit amet, aliquet mollis augue. Nullam sed augue facilisis, volutpat metus eu, volutpat nisl. \nNulla bibendum neque vitae turpis vestibulum rhoncus. Pellentesque ut ligula libero. Duis felis ante, pretium at interdum ac, varius a ante. Fusce venenatis dolor ut posuere molestie. Morbi in suscipit tellus. Cras pellentesque sapien velit. Aliquam erat volutpat. Vivamus consectetur augue nunc, sit amet facilisis turpis dapibus at. Integer finibus tincidunt quam sit amet tempor. Morbi hendrerit mollis eros quis hendrerit. Ut erat nisl, sodales at enim nec, viverra tincidunt erat. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed id porttitor risus. Vivamus quis enim nulla. Phasellus sed posuere purus. Ut sit amet auctor erat. "

    //writingZone.text: "<body><table>  <tr>
    //    <th>Month</th>
    //    <th>Savings</th>
    //  </tr>
    //  <tr>
    //    <td>January</td>
    //    <td>$100</td>
    //  </tr></table></body>"
    Component.onCompleted: {

    }

    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width < textAreaFixedWidth
        value: middleBase.width
    }
    Binding on writingZone.textAreaWidth {
        when: !Globals.compactSize && middleBase.width >= textAreaFixedWidth
        value: textAreaFixedWidth
        //only on 5.14 restoreMode: Binding.RestoreBindingOrValue
    }
    Binding on minimap.text {
        value: writingZone.textArea.text
        delayed: true
    }

    //Scrolling minimap
    writingZone.internalScrollBar.position: minimap.position
    minimap.position: writingZone.internalScrollBar.position

    // ScrollBar invisible while minimap is on
    writingZone.scrollBarVerticalPolicy: writingZone.minimapVisibility ? ScrollBar.AlwaysOff : ScrollBar.AsNeeded

    //minimap width depends on writingZone width
    //minimapFlickable.width: 300

    //    {
    //        //console.log("writingZone.flickable.width :" + writingZone.flickable.width)
    //        console.log("ratio :" + minimapFlickable.height / writingZone.flickable.contentHeight)
    //        return writingZone.flickable.width * (1 - minimapRatio())
    //    }

    //minimap.height: minimapRatio() >= 1 ? minimapFlickable.height : writingZone.flickable.contentHeight * minimapRatio()
    leftDock.onFoldedChanged: {
        if (leftDock.folded) {
            leftDockMenuGroup.visible = false
            leftDockMenuButton.checked = false
            leftDockMenuButton.visible = false
        } else {
            leftDockMenuButton.visible = true
        }
    }

    leftDockShowButton.onClicked: leftDock.folded ? leftDock.unfold(
                                                        ) : leftDock.fold()
    leftDockShowButton.icon {
        name: leftDock.folded ? "go-next" : "go-previous"
        height: 50
        width: 50
    }

    leftDockMenuButton.onCheckedChanged: leftDockMenuButton.checked ? leftDockMenuGroup.visible = true : leftDockMenuGroup.visible = false
    leftDockMenuButton.checked: false
    leftDockMenuButton.icon {
        name: "overflow-menu"
        height: 50
        width: 50
    }

    //leftDockResizeButton.onVisibleChanged: leftDock.folded = false
    //leftDockResizeButton.onClicked:
    leftDockMenuGroup.visible: false
    leftDockResizeButton.icon {
        name: "resizecol"
        height: 50
        width: 50
    }

    // compact mode :
    compactHeaderPane.visible: Globals.compactSize

    compactLeftDockShowButton.onClicked: leftDrawer.open()
    compactLeftDockShowButton.icon {
        name: "go-next"
        source: "qrc:/pics/go-next.svg"
        color: "transparent"
        height: 50
        width: 50
    }

    Connections {
        target: Globals

        // @disable-check M16
        onCompactSizeChanged: {
            if (Globals.compactSize === true) {

            } else {
                leftDrawer.close()
                rightDrawer.close()
            }
        }
    }

    Drawer {
        id: leftDrawer
        enabled: Globals.compactSize
        width: if (base.width * 0.6 > 400) {
                   return 400
               } else {
                   return base.width * 0.6
               }
        height: base.height
        modal: Globals.compactSize ? true : false
        edge: Qt.LeftEdge

        //        interactive: Globals.compactSize ? true : false
        //        visible:true
        //        position: Globals.compactSize ? 0 : 1
        NotesLeftDock {
            anchors.fill: parent
        }
    }

    Drawer {
        id: rightDrawer
        enabled: Globals.compactSize
        width: if (base.width * 0.6 > 400) {
                   return 400
               } else {
                   return base.width * 0.6
               }
        height: base.height
        modal: Globals.compactSize ? true : false
        edge: Qt.RightEdge

        //        interactive: Globals.compactSize ? true : false
        //        visible:true
        //        position: Globals.compactSize ? 0 : 1
        background: Rectangle {
            Rectangle {
                x: parent.width - 1
                width: 0
                height: parent.height
                color: "transparent"
            }
        }
    }

    // projectLoaded :
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectLoaded: function (projectId) {
            writingZone.text = plmData.sheetHub().getContent(1, 1)
        }
    }

    // projectClosed :
    Connections {
        target: plmData.projectHub()
        // @disable-check M16
        onProjectClosed: function (projectId) {
            writingZone.text = ""
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

