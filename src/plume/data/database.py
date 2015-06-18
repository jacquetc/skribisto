from PyQt5.Qt import QObject
from PyQt5.QtCore import pyqtSignal

from .plugins import Plugins
from .tree import Tree
from .project import Project
from . import subscriber, cfg


class Database(QObject):

    state_changed = pyqtSignal(str, int, name='stateChanged')

    def __init__(self, parent=None):
        super(Database, self).__init__(parent)

        cfg.data = self
        self.subscriber = subscriber

        # init all :
        self.project = Project()
        self.main_tree = Tree()
        self.plugins = Plugins()
