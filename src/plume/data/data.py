'''
Created on 19 aug. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtCore import QObject, QThread
from . import subscriber, cfg
from .signal_hub import SignalHub
from .write import Write
from .task import TaskManager
from .database_manager import DatabaseManager


class Data(QObject):
    def __init__(self, parent = None):
        super(Data, self).__init__(parent)

        cfg.data = self

        self.subscriber = subscriber
        self.signal_hub = SignalHub(self)

        self._worker_thread = QThread(self)
        cfg.worker_thread = self._worker_thread
        self.task_manager = TaskManager(self, self._worker_thread)
        self.database_manager = DatabaseManager(self, self._worker_thread)

        self.write = Write


