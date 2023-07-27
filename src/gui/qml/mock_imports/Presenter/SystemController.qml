pragma Singleton

import QtQuick

QtObject {

        signal saveSystemAsReplied(var dto)
        function saveSystemAs(dto) {
            saveSystemAsReplied(dto)
        }
    
        signal loadSystemReplied(var dto)
        function loadSystem(dto) {
            loadSystemReplied(dto)
        }
    
}