import QtQuick

QtObject {

    function loadSystem(loadSystemDto) {
        systemLoaded()
    }
    function saveSystem() {
        systemSaved()
    }
    function saveSystemAs(saveSystemAsDto) {
        systemSaved()
    }
    function closeSystem() {
        systemClosed()
    }

    signal loadSystemProgressFinished
    signal loadSystemProgressRangeChanged(int minimum, int maximum)
    signal loadSystemProgressTextChanged(QString progressText)
    signal loadSystemProgressValueChanged(int progressValue)
    signal systemLoaded
    signal systemSaved
    signal systemClosed
}
