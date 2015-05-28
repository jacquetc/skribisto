from PyQt5.QtWidgets import QToolBar
from PyQt5.QtCore import Qt
#from ..cfg import core


class ToolBar(QToolBar):
    
    def __init__(self, parent=None):
        super(ToolBar, self).__init__(parent=parent)
        self._action_list = []
        
        self.setFloatable(False)
        self.setMovable(False)
        self.setOrientation(Qt.Vertical)
        
    @property
    def action_list(self):
        return self._action_list
            
    @action_list.setter
    def action_list(self,  list):
        self._action_list = list
        for action in self._action_list:
            self.addAction(action)
                
#    def addAction(self,  action):
#        
#        QToolBar.addAction(action)
#        
