import QtQuick 2.15
import QtQuick.Controls 2.15
import "../Items"

ImporterPageForm {

    signal closeCalled()


    function goBackToImporterMainPage(){
        stackView.pop()
    }

//    onActiveFocusChanged: {
//        if (activeFocus) {
//            importFromPlumeToolButton.forceActiveFocus()
//        }
//    }

Component.onCompleted: {
    populateImporterButtonModel()
}

    //--------------------------------------------------------------
    //----- Top toolbar-----------------------------------------------
    //--------------------------------------------------------------


    ListModel {
        id: importerButtonModel
    }


    function populateImporterButtonModel(){
        importerButtonModel.clear()

        var list = skrPluginGetter.findImporterNames()

        for(var i in list){
            var name = list[i]
            var iconSource = skrPluginGetter.findImporterIconSource(name)
            var buttonText = skrPluginGetter.findImporterButtonText(name)
            var qmlUrl = skrPluginGetter.findImporterUrl(name)

            importerButtonModel.append({ "name": name, "qmlUrl": qmlUrl, "iconSource": iconSource, "buttonText": buttonText})
        }
    }


    importerButtonRepeater.model: importerButtonModel

    importerButtonRepeater.delegate: SkrButton {
        property string name: model.name
        text: model.buttonText

        icon.source: model.iconSource
        icon.color: "transparent"
        icon.height: 90
        icon.width: 90


        onClicked: {
            var item = stackView.push(model.qmlUrl, StackView.Immediate)
            item.closeCalled.connect(closeCalled)
            item.forceActiveFocus()

            item.onGoBackButtonClicked.connect(goBackToImporterMainPage)

        }


    }
}
