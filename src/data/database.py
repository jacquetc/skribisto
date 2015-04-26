from PyQt5.Qt import QObject
from PyQt5.QtCore import pyqtSignal
from . import sql

class Database(QObject):
    
    
    state_changed = pyqtSignal(str, int, name='stateChanged')

    def __init__(self, parent=None):
        super(Database, self).__init__(parent)
    
    def create_new_empty_database(self):
        self.database = sql.create_new_database()
    
    def load(self, path):
        pass
        
    def save_as(self, path):
        pass
        
    def save(self):
        pass
        
    def is_opened(self):
        pass
    
    
