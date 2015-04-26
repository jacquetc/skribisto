'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import QMainWindow
from .common import DataAndCoreSetter


class SubWindow(QMainWindow, DataAndCoreSetter):
    '''
    Abstract class. Do not use it directly. 
    '''


    def __init__(self, parent=None, parent_window_system_controller=None, data=None, core=None):
        '''
        Constructor
        '''
        super(SubWindow, self).__init__(parent=parent, data=data, core=core)
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

    def __init__(self, parent=None, parent_window_system_controller=None, data=None, core=None):
        '''
        
        '''
        super(WriteSubWindow, self).__init__(parent=parent, data=data, core=core)
        
        self.setWindowTitle("Write")
        self.setObjectName("write_sub_window")
        self.tabWidget = QTabWidget(self)
        self.setCentralWidget(self.tabWidget)
        
        self.tabWidget.addTab(WritingTabSubWindow(self, data=self.data, core=self.core), "WritingTabSubWindow")
        
        
from .writingzone.writingzone import WritingZone
        
class WritingTabSubWindow(SubWindow):
    '''
    Inner tab in the WriteSubWindoow. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None, data=None, core=None):
        '''
        Constructor
        '''
        super(WritingTabSubWindow, self).__init__(parent=parent, data=data, core=core)
        
        self.setWindowTitle("WriteTab")
        self.writing_zone = WritingZone(self, data, core)
        self.setCentralWidget(self.writing_zone)

from PyQt5.QtWidgets import QLabel

class BinderSubWindow(SubWindow):
    '''
    classdocs
    '''

    def __init__(self, parent=None, parent_window_system_controller=None, data=None, core=None):
        '''
        Constructor
        '''
        super(BinderSubWindow, self).__init__(parent=parent, data=data, core=core)
        
        self.setWindowTitle("Binder")
        self.setObjectName("binder_sub_window")
        
        label = QLabel("Binder")
        self.setCentralWidget(label)
        
        
                    
            
                

