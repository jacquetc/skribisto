import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

TabForm {
    id: root
    property string pageType : "undefined"
    property int projectId : -2
    property int paperId : -2
    readonly property string tabId: {return pageType + "_" +  projectId + "_" + paperId }

    function setTitle(newTitle) {

        root.text = newTitle
    }

    Accessible.name: root.text === "" ? action.text : root.text


    signal onCloseCalled(int index)
    closeButton.onClicked:  onCloseCalled(TabBar.index)



    readonly property bool isCurrent:  {
        if (TabBar.tabBar !== null) {
            return TabBar.index === TabBar.tabBar.currentIndex
        }
        return false
    }




//    tagText.layer.enabled: true
//      tagText.layer.smooth: true
//      tagText.layer.samplerName: "maskSource"
//    tagText.layer.effect: ShaderEffect {
//        property variant source: tagText
//        fragmentShader: "
//                varying highp vec2 qt_TexCoord0;
//                uniform highp float qt_Opacity;
//                uniform lowp sampler2D source;
//                uniform lowp sampler2D maskSource;
//                void main(void) {
//                    gl_FragColor = texture2D(source, qt_TexCoord0.st) * (1.0-texture2D(maskSource, qt_TexCoord0.st).a) * qt_Opacity;
//                }
//            "
//    }

}
