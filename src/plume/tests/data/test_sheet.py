from . import cfg


# def test_get_title(qtbot, data_object):
#
#     with qtbot.waitSignal(data_object.signal_hub.item_value_returned, timeout=1000) as blocker:
#         data_object.sheet_hub.get_title(0, 1)
#     project_id, paper_id, type, value = blocker.args
#
#     assert value == "first_title"

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

    assert value == 2
