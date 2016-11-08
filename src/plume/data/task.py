from PyQt5.QtCore import QObject, pyqtSignal, QThread, pyqtSlot, QEventLoop, QTimer, QEventLoop

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

        self.return_value = None

        self.error_sent.connect(cfg.data.signal_hub.error_sent)
        self.item_value_returned.connect(cfg.data.signal_hub.item_value_returned)
        self.item_value_changed.connect(cfg.data.signal_hub.item_value_changed)
        self.task_started.connect(cfg.data.signal_hub.task_started)
        self.task_finished.connect(cfg.data.signal_hub.task_finished)
        self.task_paused.connect(cfg.data.signal_hub.task_paused)
        self.task_resumed.connect(cfg.data.signal_hub.task_resumed)
        self.name = self.__class__

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
        print("_stop_waiting_sent")
        self.task_resumed.emit()

    def abort(self):
        pass

class TaskManager(QObject):

    # bool: if task is running, True, else False
    tasks_run = pyqtSignal(bool, name='tasks_run')

    _operate = pyqtSignal(name='_operate')

    _set_current_task_reply_sent = pyqtSignal(int, 'QString', name='_set_current_task_reply_sent')

    _pause_sent = pyqtSignal(name="_pause_sent")

    _resume_sent = pyqtSignal(name="_resume_sent")



    def __init__(self, parent, worker_thread:QThread):
        super(TaskManager, self).__init__(parent)
        self._task_list = []
        self._list_was_empty = True
        self._new_tasks_are_refused = False
        self._current_task = None
        self._timer = None
        self._return_value = None
        self._timer = QTimer(self)

        self.tasks_run.connect(cfg.data.signal_hub.tasks_run)

        self._worker = worker_thread
        self._worker.start()
        
    def __del__(self):
        self._worker.quit()
        self._worker.wait()

    def append(self, task: Task):

        if self._new_tasks_are_refused:
            return

        task.error_sent.connect(self._terminate_all_tasks)
        task.task_finished.connect(self._delete_task)
        task.task_finished.connect(self._start_next_task)

        self._list_was_empty = False
        if not self._task_list:
            self._list_was_empty = True

        self._task_list.append(task)

        if len(self._task_list) == 1 and self._list_was_empty:
            self._start_next_task()

    def append_and_wait_for_reply(self,  task: Task):
        self.append(task)

        self._timer.singleShot(2000, self._send_error_timeout)
        # make task to wait :
        loop = QEventLoop()
        self._timer.timeout.connect(loop.quit)
        task.task_finished.connect(loop.quit)
        task.error_sent.connect(loop.quit)
        loop.exec()
        self._timer.stop()

        self._return_value = self._current_task.return_value

    def _send_error_timeout(self):
        text = "Timeout ! for " + repr(self._current_task.name)
        print(text)
        cfg.data.signal_hub.error_sent.emit("text")


    @property
    def return_value(self):
        value = self._return_value
        self._return_value = None
        return value

    @pyqtSlot()
    def _start_next_task(self):
        if not self._task_list:
            self.tasks_run.emit(False)
            return

        self._current_task = self._task_list[0]

        self._current_task.moveToThread(self._worker)

        #self._set_current_task_reply_sent.connect(self._current_task.set_reply)
        print("task : " + repr(self._current_task.name))
        self._operate.connect(self._current_task.do_task)
        self._operate.emit()
        self.tasks_run.emit(True)

    @pyqtSlot()
    def _delete_task(self):
        task = self.sender()
        self._operate.disconnect(task.do_task)
        self._task_list.remove(task)
        task.deleteLater()

    @pyqtSlot()
    def _terminate_all_tasks(self):
        task = self.sender()
        task.abort()
        self.clear()

    def set_current_task_reply(self, reply_code: int, reply_string: str):
        self._set_current_task_reply_sent.emit(reply_code, reply_string)

    def pause_current_task(self):
        self._pause_sent.connect(self._current_task.pause)
        self._pause_sent.emit()
        self._pause_sent.disconnect(self._current_task.pause)

    def resume_current_task(self):
        self._resume_sent.connect(self._current_task.resume)
        self._resume_sent.emit()
        self._resume_sent.disconnect(self._current_task.resume)

    def clear(self):
        if not self._task_list:
            for task in self._task_list:
                task.deleteLater()
            self._task_list.clear()

