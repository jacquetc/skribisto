import pytest

from data import Data


@pytest.fixture(scope="module", autouse=True)
def my_own_class_run_at_beginning(request):
    print('\nIn my_own_class_run_at_beginning()')
    data = Data()
    def my_own_class_run_at_end():
        print('In my_own_class_run_at_end()')
        #data.deleteLater()

    request.addfinalizer(my_own_class_run_at_end)


