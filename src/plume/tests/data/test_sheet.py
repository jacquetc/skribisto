from . import cfg


def test_get_title(qtbot, data_object):

    with qtbot.waitSignal(data_object.signal_hub.item_value_returned, timeout=1000) as blocker:
        data_object.sheet_hub.get_title(0, 1)
    project_id, paper_id, type, value = blocker.args

    assert value == "first_title"


def test_set_title(qtbot, data_object):

    with qtbot.waitSignal(data_object.signal_hub.item_value_changed, timeout=1000) as blocker:
        data_object.sheet_hub.set_title(0, 1, "new_first_title")
    project_id, paper_id, type, value = blocker.args

    assert value == "new_first_title"

