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

class WriteSubWindow(SubWindow):
    '''
    Write Sub Window. 
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        
        '''
        super(WriteSubWindow, self).__init__(parent=parent)
        
        self.setWindowTitle("Write")
        self.setObjectName("write_sub_window")
        self.tabWidget = QTabWidget(self)
        self.setCentralWidget(self.tabWidget)
        
        self.tabWidget.addTab(WritingTabSubWindow(self), _("WritingTabSubWindow"))
        
        
from .writingzone.writingzone import WritingZone
from .docks import DockTemplate
       
class WritingTabSubWindow(SubWindow):
    '''
    Inner tab in the WriteSubWindoow. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor= None 
        '''
        super(WritingTabSubWindow, self).__init__(parent=parent)
        
        self.setWindowTitle("WriteTab")
        self.writing_zone = WritingZone(self)
        self.setCentralWidget(self.writing_zone)
        
        
        tree_view = QTreeView()
        tree_view.setModel(cfg.core.story_tree_model)
        
        dock = DockTemplate(self)
        dock.setWindowTitle("Tree")
        dock.setWidget(tree_view)
        prop_dock = cfg.gui_plugins.GuiPropertyDock()
        dock.setWidget(prop_dock.get_widget())
        
        self.addDockWidget(Qt.LeftDockWidgetArea, dock)
        
        

from PyQt5.QtWidgets import QLabel

class BinderSubWindow(SubWindow):
    '''
    classdocs
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor
        '''
        super(BinderSubWindow, self).__init__(parent=parent)
        
        self.setWindowTitle("Binder")
        self.setObjectName("binder_sub_window")
        
        label = QLabel("Binder")
        self.setCentralWidget(label)
        
        

        
        
        
                    
            
                

