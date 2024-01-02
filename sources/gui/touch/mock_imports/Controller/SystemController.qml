pragma Singleton

import QtQuick

QtObject {

        function saveSystemAs(dto) {
            EventDispatcher.system().systemSaved()
        }
        function saveSystem() {
            EventDispatcher.system().systemSaved()
        }
    
        function loadSystem(dto) {
            EventDispatcher.system().systemLoaded()
        }

        function closeSystem() {
            EventDispatcher.system().systemClosed()
        }


    
}
