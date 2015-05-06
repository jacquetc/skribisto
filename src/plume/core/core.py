'''
Created on 3 mars 2015

@author: Cyril Jacquet
'''
from PyQt5.Qt import QObject, pyqtSlot
from .story_tree_model import StoryTreeModel
from . import subscriber, cfg
from .project import Project
from .plugins import Plugins
from .tree_sheet import StoryTreeSheet


class Core(QObject):
    '''
    classdocs
    '''


    def __init__(self, parent=None, data=None):
        '''
        Constructor
        '''
        super(Core, self).__init__(parent)
        cfg.data = data
        self.subscriber = subscriber
        
        
        #init all :
        self.project = Project()
        self.story_tree = StoryTreeSheet()
        self.plugins = Plugins() 
        
        
        self.story_tree_model = StoryTreeModel(self)
        
    @pyqtSlot(str)
    def load_project(self, path=None):
        
        cfg.data.load_test_project_db()
        self.story_tree_model.reset_model()
        subscriber.announce_update()
    