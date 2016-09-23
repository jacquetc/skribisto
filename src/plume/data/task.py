from PyQt5.QtCore import QObject, pyqtSignal, QThread
from .import cfg


class Task(QObject):

    task_started = pyqtSignal(name='task_started')

    # int return_code (0 : no error , 1 : error )
    task_finished = pyqtSignal(int, name='task_finished')

    item_value_changed = pyqtSignal(int, int, 'QString', object, name='item_value_changed')
    # sent back after a modification, contain the new value
    # int project_id, int paper_id, QString type, QVariant new_value

    item_value_returned = pyqtSignal(int, int, 'QString', object, name='item_value_returned')
    # sent back when asking for a value
    # int project_id, int paper_id, QString type, QVariant value

    def __init__(self):
        super(Task, self).__init__()
        self.item_value_returned.connect(cfg.data.signal_hub.item_value_returned)
        self.item_value_changed.connect(cfg.data.signal_hub.item_value_changed)

    def do_task(self):
        pass

class TaskManager(QObject):

    # bool: if task is running, True, else False
    tasks_run = pyqtSignal(bool, name='tasks_run')

    _operate = pyqtSignal(name='_operate')

    def __init__(self, parent, worker_thread:QThread):
        super(TaskManager, self).__init__(parent)
        self._task_list = []
        self._list_was_empty = True

        self._worker = worker_thread
        self._worker.start()
        
    def __del__(self):
        self._worker.quit()
        self._worker.wait()

    def append(self, task: Task):

        task.task_finished.connect(self._delete_task)

        task.moveToThread(self._worker)
        
        self._list_was_empty = False
        if not self._task_list:
            self._list_was_empty = True

        self._task_list.append(task)

        if len(self._task_list) == 1 and self._list_was_empty:
            self._start_next_task()

    def _start_next_task(self):
        if not self._task_list:
            return

        top_task = self._task_list[0]
        self._operate.connect(top_task.do_task)
        self._operate.emit()

    def _delete_task(self):
        task = self.sender()
        self._operate.disconnect(task.do_task)
        self._task_list.remove(task)
        task.deleteLater()

        # start next task:
        self._start_next_task()



