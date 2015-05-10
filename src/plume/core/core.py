'''
Created on 3 mars 2015

@author: Cyril Jacquet
'''
from PyQt5.Qt import QObject, pyqtSlot
from .story_tree_model import StoryTreeModel
from . import subscriber, cfg
from .project import Project
from .plugins import Plugins
from .tree_sheet import TreeSheetManager


class Core(QObject):
    '''
    classdocs
    '''


    def __init__(self, parent=None, data=None):
        '''
        Constructor
        '''
        super(Core, self).__init__(parent)
        cfg.core = self
        cfg.data = data
        self.subscriber = subscriber
        
        
        #init all :
        self.project = Project()
        self.plugins = Plugins()
        cfg.core_plugins = self.plugins
        self.tree_sheet_manager = TreeSheetManager() 
        self.story_tree_model = StoryTreeModel(self)
        
