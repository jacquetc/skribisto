from src.plume.gui import main_window
from src.plume.gui import cfg, plugins,  pics_rc
from src.plume.plume import launch
from PyQt5 import Qt
import pytest
import sys
from . import cfg

from PyQt5.QtCore import QCoreApplication
import os

_df = os.environ.get('PLUME_DEVELOP_FROM', None)
if _df and os.path.exists(_df):
    sys.path.insert(0, _df)
import src.plume.error_handler

# for development :
_df = os.environ.get('PLUME_DEVELOP_DATA_BUILD_FROM', None)
if _df and os.path.exists(_df):
    sys.path.insert(0, _df)
    from ctypes import cdll
    cdll.LoadLibrary(_df + "/libplume-creator-data.so.1.61")

import plmdata

@pytest.fixture(scope="function", autouse=True)
def my_own_class_run_at_beginning(request):
    print('\nOpening')
    cfg.app = QCoreApplication.instance()


    def my_own_class_run_at_end():
        print('Closing')


    request.addfinalizer(my_own_class_run_at_end)


@pytest.fixture(scope="function")
def gui(request, qtbot):
    print('[setup] GUI')

    # data = plmdata.PLMData(QCoreApplication.instance())
    # cfg.data = data
    # cfg.gui_plugins = plugins.Plugins()
    # window = main_window.MainWindow()
    window = launch(QCoreApplication.instance())
    arguments = sys.argv
    del arguments[0]
    window.set_application_arguments(arguments)  # load project

    qtbot.addWidget(window)
    window.is_test = True
    window.show()

    def end():
        print('\n[teardown] Gui')
        # cfg.gui.deleteLater()

    request.addfinalizer(end)

    return window

@pytest.fixture()
def data_object(request, qtbot):
    print('[setup] Data')
    # code to connect to your db
    cfg.data = plmdata.PLMData(cfg.app)
    print(cfg.test_database_path)

    with qtbot.waitSignal(cfg.data.projectHub().projectLoaded, timeout=1000) as blocker:
        cfg.data.projectHub().loadProject(cfg.test_database_path)
        print(cfg.data.projectHub().getProjectIdList())

    def end():
        print('\n[teardown] Data')
        # ensure the task list is finished before closing
        # print(cfg.data.projectHub().getProjectIdList())

        with qtbot.waitSignal(cfg.data.projectHub().allProjectsClosed, timeout=1000) as blocker:
            cfg.data.projectHub().closeAllProjects()
        #cfg.data.deleteLater()

        base = os.path.basename(cfg.test_database_path)
        cfg.data.deleteLater()

    request.addfinalizer(end)

    return cfg.data