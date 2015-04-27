from PyQt5.Qt import QObject
from PyQt5.QtCore import pyqtSignal
from . import sql

from .plugins import Plugins
from .story_tree import StoryTree
from .project import Project


class Database(QObject):
    
    state_changed = pyqtSignal(str, int, name='stateChanged')

    def __init__(self, parent=None):
        super(Database, self).__init__(parent)
        
        self._update_funcs = []
        
        
        #init all :
        self.project = Project()
        self.story_tree = StoryTree()
        self.plugins = Plugins()
        
        
        
        

        
        
        
        
        
        self.db = None

            
    def create_new_empty_database(self):
        self.database = sql.create_new_database()
    
    def load_test_project_db(self):
        """
        This func is only temporary. To have something to work with...
        """
        
        
        import sqlite3

        new_db = sqlite3.connect(':memory:') # create a memory database
        old_db = sqlite3.connect('/home/cyril/Devel/workspace_eclipse/plume-creator/resources/plume_test_project.sqlite')
        query = "".join(line for line in old_db.iterdump())

        # Dump old database in the new one. 
        new_db.executescript(query)
        
        self.db = new_db
        self.story_tree.db = new_db
        
    def load(self, path):
        pass
        
    def save_as(self, path):
        pass
        
    def save(self):
        pass
        
    def is_opened(self):
        pass
    
    def close_db(self):
        pass
    

    def subscribe_update_func(self, func):
        if func not in self._update_funcs:
            self._update_funcs.append(func)
            

    def unsubscribe_update_func(self, func):
        if func in self._update_funcs:
            self._update_funcs.remove(func)
            

    def announce_update(self):
        for func in self._update_funcs:
            func()
    
