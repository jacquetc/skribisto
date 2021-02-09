import QtQuick 2.0
import QtTest 1.0

TestCase {
    name: "writetreecase"
    id: testCase
    when: windowShown

    Component {
        id: component

        Item {

        }

    }

    function initTestCase() {
    }

    function cleanupTestCase() {
    }

    function test_case1() {
        compare(1 + 1, 2, "sanity check");
        verify(true);
    }
    function test_loadProject(){
        var item = createTemporaryObject(component, testCase);
                  verify(item);


    }

}
