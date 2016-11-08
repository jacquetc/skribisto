from . import cfg
from PyQt5.QtWidgets import QListWidget

def test_close_database(qtbot, data_object):
    assert len(data_object.database_manager.database_for_int_dict) == 1

def test_close_and_load_the_same_database(qtbot, data_object):

    # window = QListWidget()
    # window.show()
    # qtbot.addWidget(window)
    print("test_close_and_load_the_same_database")
    print(data_object.database_manager.database_for_int_dict.keys())
    # closing already opened project :
    with qtbot.waitSignal(data_object.signal_hub.item_value_returned, timeout=1000) as blocker:
        data_object.project_hub.close_project(0)
    project_id, paper_id, type, value = blocker.args
    assert blocker.signal_triggered
    assert project_id == 0

    # window.clear()
    # window.addItems(cfg.data.task_manager._task_list)

    # loading new project :
    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.project_hub.load_project(cfg.test_database_path)
    project_id, paper_id, type, value = blocker.args
    print(data_object.database_manager.database_for_int_dict.keys())

    assert blocker.signal_triggered
    assert project_id == 1

def test_error_when_loading_locked_database(qtbot, data_object):
    # loading project again :
    with qtbot.waitSignal(data_object.signal_hub.error_sent, timeout=1000) as blocker:
        data_object.project_hub.load_project(cfg.test_database_path)
    error_string = blocker.args[0]
    assert error_string == "E_LOCKEDDATABASE"