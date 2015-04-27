'''
Created on 3 mars 2015

@author: Cyril Jacquet
'''
from PyQt5.Qt import QObject, pyqtSlot
from .story_tree_model import StoryTreeModel


class Core(QObject):
    '''
    classdocs
    '''


    def __init__(self, parent=None, data=None):
        '''
        Constructor
        '''
        super(Core, self).__init__(parent)
        self.data = data
        
        self.story_tree_model = StoryTreeModel(self, self.data)
        
    @pyqtSlot(str)
    def load_project(self, path=None):
        
        self.data.load_test_project_db()
        self.story_tree_model.reset_model()
    