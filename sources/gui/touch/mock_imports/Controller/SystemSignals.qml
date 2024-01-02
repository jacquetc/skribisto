pragma Singleton

import QtQuick

QtObject {

    signal loadSystemProgressFinished
    signal loadSystemProgressRangeChanged(int minimum, int maximum)
    signal loadSystemProgressTextChanged(QString progressText)
    signal loadSystemProgressValueChanged(int progressValue)
    signal systemLoaded
    signal systemSaved
    signal systemClosed

        }
