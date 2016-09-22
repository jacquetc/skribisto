from . import cfg

def test_set_title(qtbot):

    cfg.data.write.set_title(0, 1, "first_title_test")

    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.write.get_title(0, 1)
    project_id, paper_id, type, value = blocker.args

    assert value == "first_title_test"

def test_get_title(qtbot):

    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.write.get_title(0, 1)
    project_id, paper_id, type, value = blocker.args

    assert value == "first_title"