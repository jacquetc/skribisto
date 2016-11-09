def test_get_title(qtbot, data_object):

    value = data_object.sheet_hub.get_title(0, 1)

    assert value == "first_title"

def test_set_title(qtbot, data_object):

    with qtbot.waitSignal(data_object.signal_hub.item_value_changed, timeout=1000) as blocker:
        data_object.sheet_hub.set_title(0, 1, "new_first_title")
    project_id, paper_id, type, value = blocker.args

    assert value == "new_first_title"

def test_get_content(qtbot, data_object):

    value = data_object.sheet_hub.get_content(0, 1)

    assert value == "first content"

def test_set_content(qtbot, data_object):

    with qtbot.waitSignal(data_object.signal_hub.item_value_changed, timeout=1000) as blocker:
        data_object.sheet_hub.set_content(0, 1, "new first content")
    project_id, paper_id, type, value = blocker.args

    assert value == "new first content"

def test_get_indent(qtbot, data_object):

    value = data_object.sheet_hub.get_indent(0, 1)

    assert value == 0

def test_set_indent(qtbot, data_object):

    with qtbot.waitSignal(data_object.signal_hub.item_value_changed, timeout=1000) as blocker:
        data_object.sheet_hub.set_indent(0, 1, 2)
    project_id, paper_id, type, value = blocker.args

    value = data_object.sheet_hub.get_indent(0, 1)

    assert value == 2

def test_get_all(qtbot, data_object):
    value = data_object.sheet_hub.get_all(0)
    assert value

def test_get_all_headers(qtbot, data_object):
    value = data_object.sheet_hub.get_all(0)
    assert value
    assert type(value) is list

def test_set_property(qtbot, data_object):
    with qtbot.waitSignal(data_object.signal_hub.sheet_property_changed, timeout=1000) as blocker:
        data_object.sheet_hub.set_property(0, 1, "new test property", "new test value")
    project_id, paper_id, property_name, property_value = blocker.args

    assert project_id == 0
    assert paper_id == 1
    assert property_name == "new test property"
    assert property_value == "new test value"

    # fetch and test
    value = data_object.sheet_hub.get_property(0, 1, "new test property")
    assert value == "new test value"

    # change existing value :
    data_object.sheet_hub.set_property(0, 1, "new test property", "value 2")
    value = data_object.sheet_hub.get_property(0, 1, "new test property")
    assert value == "value 2"

def test_get_property(qtbot, data_object):
    value = data_object.sheet_hub.get_property(0, 1, "test property")
    assert value == "test value"