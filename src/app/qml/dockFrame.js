var component;
var body;
var parent;

function createDockBody(qmlPath, bodyParent) {
    parent = bodyParent
    component = Qt.createComponent(qmlPath);
    if (component.status === Component.Ready)
        finishCreation();
    else
        component.statusChanged.connect(finishCreation);
}

function finishCreation() {
    if (component.status === Component.Ready) {
        body = component.createObject(parent, {"anchors.fill": parent});
        if (body === null) {
            // Error Handling
            console.log("Error creating object");
        }
    } else if (component.status === Component.Error) {
        // Error Handling
        console.log("Error loading component:", component.errorString());
    }
}
