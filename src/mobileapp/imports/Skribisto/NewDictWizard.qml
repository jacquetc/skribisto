import QtQuick
import QtQml
import eu.skribisto.spellchecker 1.0
import eu.skribisto.download 1.0

NewDictWizardForm {

    id: root

    Component.onCompleted: {
        populateDictComboBox()
    }





    closeButton.onClicked: root.close()

    Binding{
        id: progressBarValueBinding
        target: progressBar
        property: "value"
        value: downloader.progress
        when: dictComboBoxModel.count > 0
    }


    Download {
        id: downloader
        onError: function(errorCode, errorString){
            infoLabel.text = errorString
        }


    }


    installButton.onClicked: {


        infoLabel.text = qsTr("Downloading from %1 (%2/%3)").arg("https://github.com/wooorm/dictionaries/").arg(1).arg(2)

        var lang = dictComboBox.currentValue


        downloader.onFinished.connect(downloadAff)
        downloader.url = "https://github.com/wooorm/dictionaries/blob/main/dictionaries/" + lang + "/index.dic?raw=true"

        skrRootItem.createPath(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/")
        var file = skrQMLTools.getURLFromLocalFile(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/" + lang + ".dic")

        downloader.destination = file
        downloader.start()



    }

    function downloadAff(){
        downloader.onFinished.disconnect(downloadAff)
        infoLabel.text = qsTr("Downloading from %1 (%2/%3)").arg("https://github.com/wooorm/dictionaries/").arg(2).arg(2)

        downloader.onFinished.connect(finish)
        var lang = dictComboBox.currentValue

        downloader.url = "https://github.com/wooorm/dictionaries/blob/main/dictionaries/" + lang + "/index.aff?raw=true"

        skrRootItem.createPath(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/")
        var file = skrQMLTools.getURLFromLocalFile(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/" + lang + ".aff")

        downloader.destination = file
        downloader.start()



    }

    function finish(){
        downloader.onFinished.disconnect(finish)

        infoLabel.text = qsTr("Dictionary \"%1\" successfully installed").arg(dictComboBox.currentValue)
        Globals.newDictInstalled(dictComboBox.currentValue)
    }



    // spell checking combo box :

    SKRSpellChecker {
        id : spellChecker
    }

    ListModel {
        id: dictComboBoxModel
    }
    dictComboBox.model: dictComboBoxModel
    dictComboBox.valueRole: "dictCode"
    dictComboBox.textRole: "text"

    function populateDictComboBox(){
        skrRootItem.removeFile(skrRootItem.getTempPath() + "/dict_tree")
        downloader.onFinished.connect(findDictsFromGitHubTree)
        downloader.url =  "https://api.github.com/repos/wooorm/dictionaries/git/trees/main?recursive=1"
        var file = skrQMLTools.getURLFromLocalFile(skrRootItem.getTempPath() + "/dict_tree")
        downloader.destination = file
        downloader.start()


    }

    function findDictsFromGitHubTree(){
        downloader.onFinished.disconnect(findDictsFromGitHubTree)
        var dictList = skrRootItem.getDictFoldersFromGitHubTree(skrRootItem.getTempPath() + "/dict_tree")

        var i;
        for(i = 0 ; i < dictList.length ; i++){
            dictComboBoxModel.append({"text": skrRootItem.getNativeLanguageNameFromLocale(dictList[i]) + " (" + dictList[i] + ")", "dictCode": dictList[i]})
        }


        var value = SkrSettings.spellCheckingSettings.spellCheckingLangCode
        var index = dictComboBox.indexOfValue(value)
        if(index >= 0){
            dictComboBox.currentIndex = index
        }
        else {
            value = skrRootItem.getOnlyLanguageFromLocale(value);
            index = dictComboBox.indexOfValue(value)
            dictComboBox.currentIndex = index
        }



    }

    viewLicenseButton.onClicked: {


        var lang = dictComboBox.currentValue

        infoLabel.text = qsTr("Downloading license from %1 for %2").arg("https://github.com/wooorm/dictionaries/").arg(lang)



        downloader.onFinished.connect(displayLicense)
        downloader.url = "https://github.com/wooorm/dictionaries/blob/main/dictionaries/" + lang + "/license?raw=true"

        skrRootItem.createPath(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/")
        var file = skrQMLTools.getURLFromLocalFile(skrRootItem.getWritableAddonsPathsListDir() + "/dicts/" + lang + "_license.txt")
        priv.licenseFileName = file
        downloader.destination = file
        downloader.start()

    }

    QtObject{
        id: priv
        property url licenseFileName
    }

    function displayLicense(){
        downloader.onFinished.disconnect(displayLicense)
        infoLabel.text = ""

        Qt.openUrlExternally(priv.licenseFileName)

    }


}
