from PyQt5.QtCore import QObject, pyqtSignal, QThread, pyqtSlot, QEventLoop
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

    _stop_waiting_sent = pyqtSignal(name='_stop_waiting_sent')

    task_paused = pyqtSignal(name='task_paused')

    task_resumed = pyqtSignal(name='task_resumed')

    error_sent = pyqtSignal('QString', name='error_sent')


    def __init__(self):
        super(Task, self).__init__()
        self.item_value_returned.connect(cfg.data.signal_hub.item_value_returned)
        self.item_value_changed.connect(cfg.data.signal_hub.item_value_changed)
        self.task_started.connect(cfg.data.signal_hub.task_started)
        self.task_finished.connect(cfg.data.signal_hub.task_finished)
        self.task_paused.connect(cfg.data.signal_hub.task_paused)
        self.task_resumed.connect(cfg.data.signal_hub.task_resumed)

    def do_task(self):
        pass

    def _wait_for_reply_signal(self):
        loop = QEventLoop()
        self._stop_waiting_sent.connect(loop.quit)
        loop.exec_()

    @pyqtSlot(int, 'QString')
    def set_reply(self, reply_code: int, reply_string: str):
        self._stop_waiting_sent.emit()

    def pause(self):
        self.task_paused.emit()
        self._wait_for_reply_signal()

    def resume(self):
        self._stop_waiting_sent.emit()
        self.task_resumed.emit()


class TaskManager(QObject):

    # bool: if task is running, True, else False
    tasks_run = pyqtSignal(bool, name='tasks_run')

    _operate = pyqtSignal(name='_operate')

    _set_current_task_reply_sent = pyqtSignal(int, 'QString', name='_set_current_task_reply_sent')


    def __init__(self, parent, worker_thread:QThread):
        super(TaskManager, self).__init__(parent)
        self._task_list = []
        self._list_was_empty = True
        self._new_tasks_are_refused = False
        self._current_task = None

        self.tasks_run.connect(cfg.data.signal_hub.tasks_run)

        self._worker = worker_thread
        self._worker.start()
        
    def __del__(self):
        self._worker.quit()
        self._worker.wait()

    def append(self, task: Task):

        if self._new_tasks_are_refused == True:
            return

        task.task_finished.connect(self._delete_task)
        task.task_finished.connect(self._start_next_task)

        task.moveToThread(self._worker)
        
        self._list_was_empty = False
        if not self._task_list:
            self._list_was_empty = True

        self._task_list.append(task)

        if len(self._task_list) == 1 and self._list_was_empty:
            self._start_next_task()

    @pyqtSlot()
    def _start_next_task(self):
        if not self._task_list:
            self.tasks_run.emit(False)
            return

        self._current_task = self._task_list[0]

        self._set_current_task_reply_sent.connect(self._current_task.set_reply)

        self._operate.connect(self._current_task.do_task)
        self._operate.emit()
        self.tasks_run.emit(True)

    @pyqtSlot()
    def _delete_task(self):
        task = self.sender()
        self._operate.disconnect(task.do_task)
        self._task_list.remove(task)
        task.deleteLater()

    def set_current_task_reply(self, reply_code: int, reply_string: str):
        self._set_current_task_reply_sent.emit(reply_code, reply_string)

    def pause_current_task(self):
        self._current_task.pause()

    def resume_current_task(self):
        self._current_task.resume()

    def clear(self):
        if not self._task_list:
            for task in self._task_list:
                task.deleteLater()
            self._task_list.clear()

