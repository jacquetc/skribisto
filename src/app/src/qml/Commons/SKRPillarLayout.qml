import QtQuick
import QtQuick.Layouts
import QtQml.Models

Item {
    id:root

    default property list<Item> childrenList
    property int columns: 1
    property int maxColumns: 1
    property int columnSpacing: 5

    function placeChildren(){

        var repeaterCount = repeater.count

        var numberPerPillar = (childrenList.length / repeaterCount) | 0
        var leftOver = childrenList.length % repeaterCount

        var distributionOfNumbers = []

        var k
        for(k = 0 ; k < repeaterCount; k++){
            var number = numberPerPillar

            if(leftOver !== 0){
                //give one from leftOver
                number++
                leftOver--
            }

            distributionOfNumbers.push(number)
        }

        //console.log("distributionOfNumbers", distributionOfNumbers)

        var distributionOfChildren = []


        var c
        for(c = 0 ; c < distributionOfNumbers.length; c++){

            var childrenSubList = []

            var sumOfNumbersForBegining = 0

            var s
            for(s = 0; s < c ; s++){

                sumOfNumbersForBegining += distributionOfNumbers[s]

            }

            var sumOfNumbersForEnding = 0
            for(s = 0; s <= c ; s++){

                sumOfNumbersForEnding += distributionOfNumbers[s]

            }


            var n
            for(n = sumOfNumbersForBegining; n < sumOfNumbersForEnding; n++){
                childrenSubList.push(childrenList[n])
            }
            distributionOfChildren.push(childrenSubList)

        }

        var columnHeightList = []

        var repeaterIndex;
        for(repeaterIndex = 0; repeaterIndex < repeaterCount ; repeaterIndex++){

            var columnLayout
            if(repeater.itemAt(repeaterIndex)){
                columnLayout = repeater.itemAt(repeaterIndex)
            }
            else {
                console.warn("repeater.itemAt(repeaterIndex) : no item")
                break;
            }



            var childrenForThisColumn = distributionOfChildren[repeaterIndex]

            //console.log("childrenForThisColumn", childrenForThisColumn)

            columnLayout.data = childrenForThisColumn

            // take heights


            var columnHeight = 0
            var m
            for(m = 0; m < columnLayout.data.length; m++){
                    var child = columnLayout.data[m]
                columnHeight += child.height + columnSpacing
            }


            columnHeightList.push(columnHeight)

            columnLayout.data.push(stretcherComponent.createObject(columnLayout))


        }



        var tallerHeight = 0
        var t
        for(t = 0 ; t < columnHeightList.length ; t++){
            if( columnHeightList[t] > tallerHeight){
                tallerHeight = columnHeightList[t]
            }
        }

        root.implicitHeight = tallerHeight

//        console.log("columnHeightList", columnHeightList)
//        console.log("implicitHeight", implicitHeight)


    }



    Component {
        id: stretcherComponent
        Item {
            Layout.fillHeight: true
        }
    }


    Component.onCompleted: {
        if(repeater.count === 0){
            return;
        }
        placeChildrenTimer.start()


    }

    Timer{
        id: placeChildrenTimer
        interval: 50
        repeat: false
        onTriggered: placeChildren()
    }


    RowLayout {
        anchors.fill: parent

        Repeater {
            id: repeater
            model: columns === 0 ? 1 : (columns <= maxColumns ? columns : maxColumns)
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true

            }


            onCountChanged:  {
                if(count === 0){
                    return;
                }


                placeChildrenTimer.start()


            }

        }

    }



}
