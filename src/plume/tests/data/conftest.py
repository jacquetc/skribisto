import pytest

from . import cfg
from ...data.data import Data
from PyQt5.QtCore import QCoreApplication


@pytest.fixture(scope="function", autouse=True)
def my_own_class_run_at_beginning(request):
    print('\nOpening db')
    cfg.app = QCoreApplication.instance()
    cfg.data = Data(cfg.app)
    cfg.data.project_hub.load_project(cfg.test_database_path)

    def my_own_class_run_at_end():
        print('Closing db')
        cfg.data.project_hub.close_all_projects()
        cfg.data.deleteLater()

    request.addfinalizer(my_own_class_run_at_end)


