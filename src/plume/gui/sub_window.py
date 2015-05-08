'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import QMainWindow, QTreeView
from PyQt5.QtWidgets import QDockWidget
from PyQt5.QtCore import Qt
from . import cfg


class SubWindow(QMainWindow):
    '''
    Abstract class. Do not use it directly. 
    '''


    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor
        '''
        super(SubWindow, self).__init__(parent=parent)
        self.parent_window_system_controller = parent_window_system_controller
        
        
    def detach_this_window(self):
        self.parent_window_system_controller.detach_window(self)
    
    def attach_back_to_parent_window_system(self):
        self.parent_window_system_controller.attach_window(self)
    
    
        

from PyQt5.QtWidgets import QTabWidget
from .docks import DockTemplate
from .story_tree import StoryTreeView

class WritePanel(SubWindow):
    '''
    'Write' main panel. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        
        '''
        super(WritePanel, self).__init__(parent=parent)
        
        self.setWindowTitle("Write")
        self.setObjectName("write_sub_window")

        
        # Project tree view dock :
        tree_view = StoryTreeView()
        tree_view.setModel(cfg.core.story_tree_model)
        
        dock = DockTemplate(self)
        dock.setWindowTitle(_("Project"))
        dock.setWidget(tree_view) 
        
        self.addDockWidget(Qt.LeftDockWidgetArea, dock)
               
from .writingzone.writingzone import WritingZone
       
class WriteTab(SubWindow):
    '''
    Inner tab in the Write panel. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor= None 
        '''
        super(WriteTab, self).__init__(parent=parent)
        
        self.setWindowTitle("WriteTab")
        self.writing_zone = WritingZone(self)
        self.setCentralWidget(self.writing_zone)
        

        dock = DockTemplate(self)
        dock.setWindowTitle(_("Properties"))
        prop_dock = cfg.gui_plugins.GuiPropertyDock()
        dock.setWidget(prop_dock.get_widget())
        
        self.addDockWidget(Qt.RightDockWidgetArea, dock)
        
        

from PyQt5.QtWidgets import QLabel

class BinderPanel(SubWindow):
    '''
    classdocs
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor
        '''
        super(BinderPanel, self).__init__(parent=parent)
        
        self.setWindowTitle("Binder")
        self.setObjectName("binder_sub_window")
        
        label = QLabel("Binder")
        self.setCentralWidget(label)
        
        

        
        
        
                    
            
                

