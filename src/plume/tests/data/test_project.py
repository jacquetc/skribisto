from . import cfg


def test_close_and_load_the_same_database(qtbot):
    print(cfg.data.database_manager.database_for_int_dict.keys())
    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.project_hub.close_project(0)
    project_id, paper_id, type, value = blocker.args
    assert blocker.signal_triggered
    assert project_id == 0

    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.project_hub.load_project(cfg.test_database_path)
    project_id, paper_id, type, value = blocker.args

    assert blocker.signal_triggered
    assert project_id == 1


def test_error_load_locked_database(qtbot):
    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.project_hub.load_project(cfg.test_database_path)
    project_id, paper_id, type, value = blocker.args

    assert 0 == 0