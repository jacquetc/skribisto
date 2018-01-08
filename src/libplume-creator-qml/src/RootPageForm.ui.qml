import QtQuick 2.9
import QtQuick.Controls 2.3

Item {
    property variant window: none

    StackView {
        WelcomePage {
            id: welcomePage
        }

        WritePage {
            id: writePage
        }
    }
}
