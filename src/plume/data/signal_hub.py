from PyQt5.QtCore import QObject, pyqtSignal, pyqtSlot
from . import cfg


class SignalHub(QObject):
    item_value_returned = pyqtSignal(int, int, 'QString', object, name='item_value_returned')
    item_value_changed = pyqtSignal(int, int, 'QString', object, name='item_value_changed')
    tasks_run = pyqtSignal(bool, name='tasks_run')

    task_paused = pyqtSignal(name='task_paused')
    task_resumed = pyqtSignal(name='task_resumed')
    task_started = pyqtSignal(name='task_started')
    # int return_code (0 : no error , 1 : error )
    task_finished = pyqtSignal(int, name='task_finished')


    error_sent = pyqtSignal('QString', name='error_sent')

    def __init__(self, parent):
        super(SignalHub, self).__init__(parent)

    @pyqtSlot('QString')
    def send_error(self, error_string: str):
        cfg.data.task_manager.pause_current_task()
        cfg.data.task_manager.clear()
        cfg.data.task_manager.resume_current_task()
        self.error_sent.emit(error_string)
