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
from .docks import DockTemplate, DockSystem
from .story_tree import StoryTreeView
from .window_system import WindowSystemController

class WritePanel(SubWindow, WindowSystemController):
    '''
    'Write' main panel. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        
        '''
        super(WritePanel, self).__init__(parent=parent)
        
        self.setWindowTitle("Write")
        self.setObjectName("write_sub_window")


        #connect core signal to open sheets :
        cfg.core.tree_sheet_manager.sheet_is_opening.connect(self.open_write_tab)
        
        # Project tree view dock :
        tree_view = StoryTreeView()
        tree_view.setModel(cfg.core.story_tree_model)
        
        dock = DockTemplate(self)
        dock.setWindowTitle(_("Project"))
        dock.setWidget(tree_view) 
        
        self.addDockWidget(Qt.LeftDockWidgetArea, dock)
        
        #set TabWidget:
        self.tab_widget = QTabWidget(self)
        self.setCentralWidget(self.tab_widget)
        
            
    def open_write_tab(self, tree_sheet):

        new_write_tab = WriteTab(self, self)
        new_write_tab.tree_sheet = tree_sheet
        self.tab_widget.addTab(new_write_tab, new_write_tab.tab_title) 
        #temp for test:       
        new_write_tab.dock_system.add_dock("properties-dock")
        
               
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
        
        self._tree_sheet = None
        self.tab_title = "Error"
        
        self.dock_system = DockSystem(self, self)
        
        self.setWindowTitle("WriteTab")
        self.writing_zone = WritingZone(self)
        self.setCentralWidget(self.writing_zone)
        
        
    @property
    def tree_sheet(self):
        return self._tree_sheet
    
    @tree_sheet.setter
    def tree_sheet(self, tree_sheet_object):
        self._tree_sheet = tree_sheet_object
        self._load_from_tree_sheet(tree_sheet_object)
        
    def _load_from_tree_sheet(self, tree_sheet_object):
        self.tab_title = tree_sheet_object.get_title()
        self.writing_zone.rich_text_edit.setText(tree_sheet_object.get_content())
        self.dock_system.sheet_id = tree_sheet_object.sheet_id
                
    def change_tab_title(self, new_title):
        tab_widget = self.parent().tab_widget
        index = tab_widget.indexOf(self)
        tab_widget.setTabText(index, new_title)

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
        
        

        
        
        
                    
            
                

