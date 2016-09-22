import pytest
import pytestqt

from . import cfg
from ...data.data import Data
from PyQt5.QtCore import QCoreApplication

@pytest.fixture(scope="module", autouse=True)
def my_own_class_run_at_beginning(request):
    print('\nOpening db')
    cfg.app = QCoreApplication.instance()
    cfg.data = Data(cfg.app)
    cfg.data.database_manager.load_database(0, '../../../../resources/plume_test_project.sqlite')

    def my_own_class_run_at_end():
        print('Closing db')
        cfg.data.database_manager.close_database(0)
        cfg.data.deleteLater()

    request.addfinalizer(my_own_class_run_at_end)


