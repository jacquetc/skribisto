import QtQuick

QtObject {

     property QtObject priv: QtObject {
         property QtObject awaitObject: QtObject{
            property var value
         }

        property int delay: 50
        property var signalFn: function(){}

     }

    function setValue(value){
        priv.awaitObject.value = value
    }

    function setDelay(delay){
        priv.delay = delay
    }

    function setSignalFn(fn){
        priv.signalFn = fn
    }

    function await(){
        return awaitObject
    }

    function then(fn){
        var promise = new Promise((resolve, reject) => {
                                       var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                                       timer.interval = 50; // delay
                                       timer.repeat = false;
                                       timer.triggered.connect(() => {
                                                                   fn(priv.awaitObject.value)
                                                                   priv.signalFn()
                                                                   resolve();
                                                                   timer.destroy(); // Clean up the timer
                                                               });
                                       timer.start();
                                   })

    }
}