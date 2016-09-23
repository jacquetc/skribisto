import pytest
import sys
from . import cfg
from ...data.data import Data
from PyQt5.QtCore import QCoreApplication


@pytest.fixture(scope="function", autouse=True)
def my_own_class_run_at_beginning(request):
    print('\nOpening')
    cfg.app = QCoreApplication.instance()


    def my_own_class_run_at_end():
        print('Closing')


    request.addfinalizer(my_own_class_run_at_end)


def test_tasks_run_false(status):
    """Return true if tasks_run signal is False."""
    return status == False

@pytest.fixture()
def data_object(request, qtbot):
    print('[setup] Data')
    # code to connect to your db
    cfg.data = Data(cfg.app)
    with qtbot.waitSignal(cfg.data.signal_hub.item_value_returned, timeout=1000) as blocker:
        cfg.data.project_hub.load_project(cfg.test_database_path)

    def end():
        print('\n[teardown] Data')
        # ensure the task list is finished before closing
        with qtbot.waitSignal(cfg.data.signal_hub.tasks_run, timeout=1000, check_params_cb=test_tasks_run_false) as blocker:
            cfg.data.project_hub.close_all_projects()
        #cfg.data.deleteLater()

    request.addfinalizer(end)

    return cfg.data