# -*- coding: utf-8 -*-
'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5 import QtCore
from PyQt5.QtWidgets import (QMainWindow, QWidget, QActionGroup, QLabel, \
                             QHBoxLayout, QVBoxLayout, QPushButton, QAction, \
                             QToolBar, QMenuBar)
from PyQt5.QtGui import QIcon
from PyQt5.QtCore import QObject, Qt
from .writingzone.writingzone import WritingZone
from .common import DataAndCoreSetter
from .window_system import WindowSystemController
from .sub_window import WriteSubWindow, BinderSubWindow
from PyQt5.Qt import QToolButton
from . import cfg

class MainWindow(QMainWindow, WindowSystemController):

    def __init__(self, parent):
        super(MainWindow, self).__init__()
        self.init_ui()


    def init_ui(self):
        self.setWindowTitle("Plume Creator")

        widget = QWidget()
        layout = QHBoxLayout()

        #self.sub_window = SubWindow()
        self.stackWidget = SubWindowStack(self)
        self.side_bar = SideBar(self)
        #self.side_bar.detach_signal.connect(self.detach_window)

        layout.addWidget(self.side_bar)
        layout.addWidget(self.stackWidget)
        #layout.addWidget(self.sub_window)

        widget.setLayout(layout)
        self.setCentralWidget(widget)

        self.resize(1024, 780)
        
     
        
        # window system :
        self.window_system_parent_widget = self.stackWidget
        self.side_bar.window_system_controller = self
        

        #sub_windows :
        self._sub_window_action_group = QActionGroup(self)
       
        ##write window
        self.write_sub_window = WriteSubWindow(parent=self, parent_window_system_controller=self \
                                               )       
        self.attach_sub_window(self.write_sub_window)
        write_action = QAction("Write", self)
        write_action.setProperty("sub_window_object_name", "write_sub_window")
        self.add_action_to_window_system(write_action)
        self._sub_window_action_group.addAction(write_action)
        

        ##binder window
        self.binder_sub_window = BinderSubWindow(parent=self, parent_window_system_controller=self\
                                           ) 
        self.binder_sub_window.setWindowTitle("Binder")
        self.binder_sub_window.setObjectName("binder_sub_window")
        self.attach_sub_window(self.binder_sub_window)
        binder_action = QAction("Binder", self)
        binder_action.setProperty("sub_window_object_name", "binder_sub_window")
        self.add_action_to_window_system(binder_action)       
        self._sub_window_action_group.addAction(binder_action)
        


        # menu bar
        menu_bar = self.menuBar()
        
        
        project_menu = QMenu("Project", self)
        
        open_project_act = QAction("&Open project",self)
        open_project_act.triggered.connect(cfg.core.load_project)
        project_menu.addAction(open_project_act)
        
        menu_bar.addMenu(project_menu)

        cfg.core.subscriber.subscribe_update_func(self.rrrrr)
        
    def rrrrr(self):
        print("rrrr")


class SubWindow(QMainWindow):

    def __init__(self, parent=None):
        super(SubWindow, self).__init__(parent)
        writing_zone = WritingZone()
        widget = QWidget()
        layout = QHBoxLayout()  
        layout.addWidget(writing_zone)
        widget.setLayout(layout)
        self.setCentralWidget(widget)


from PyQt5.QtWidgets import QStackedWidget
from PyQt5.QtCore import Qt
from .window_system import WindowSystemParentWidget

class SubWindowStack(QStackedWidget,WindowSystemParentWidget):
    '''
    classdocs
    ''' 
    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(SubWindowStack, self).__init__(parent)   
        
        
        
        
        
    def attach_sub_window(self, sub_window):
        WindowSystemParentWidget.attach_sub_window(self, sub_window)
        self.addWidget(sub_window)

        
    def detach_sub_window(self, sub_window):
        WindowSystemParentWidget.detach_sub_window(self, sub_window)

    def set_sub_window_visible(self, sub_window):
        WindowSystemParentWidget.set_sub_window_visible(self, sub_window)
        if sub_window.isWindow():
            sub_window.setWindowState(sub_window.windowState() & ~Qt.WindowMinimized | Qt.WindowActive)
            sub_window.activateWindow()
        else:
            self.setCurrentWidget(sub_window)
        
    


from .window_system import WindowSystemActionHandler
from PyQt5.QtWidgets import QToolButton, QMenu

class SideBar(QWidget, WindowSystemActionHandler):

    
    
    
    
    #detach_signal = QtCore.pyqtSignal(QMainWindow)

    
    def __init__(self, parent=None):
        super(SideBar, self).__init__(parent)

        self.side_bar_button_list = []

        layout = QVBoxLayout()
        self.sub_window_buttons_layout = QVBoxLayout()
        

        layout.addLayout(self.sub_window_buttons_layout)
        self.setLayout(layout)
        

    def onDetachClicked(self):
        '''
        Must only be used as a slot for Qt signals ! 
        '''
        self.detach_sub_window()
        
        
    def onAttachClicked(self):
        '''
        Must only be used as a slot for Qt signals ! 
        '''
        self.attach_sub_window()
        for button in self.side_bar_button_list:
            if button.property("sub_window_object_name") == self.sender().property("sub_window_object_name"):
                button.click()
        

    def action_toggled_slot(self, value):
        '''
        Must only be used as a slot for Qt signals ! 
        '''
        if value == True:
            self.set_sub_window_visible()
            
    def set_sub_window_visible(self):
        '''
        Must only be used as a slot for Qt signals ! 
        '''
        sub_window = self.get_sub_window_linked_to_action(self.sender())
        self.window_system_controller.window_system_parent_widget.set_sub_window_visible(sub_window)        
    
    def clear(self):
        for button in self.side_bar_button_list:
            self.side_bar_button_list.remove(button)
            button.deleteLater()
            del button
        self.side_bar_button_list = []
        
    def update_from_window_system_ctrl(self):
        
        class SideBarButton(QToolButton):
            
            def __init__(self, parent, action):
                super(SideBarButton, self).__init__(parent)
                self.setCheckable(True)
                self.setDefaultAction(action)
                self._prop = action.property("sub_window_object_name")
                self.setProperty("sub_window_object_name", self._prop)

                
                
            def contextMenuEvent(self, event):
                menu = QMenu(self)
                attachAction = menu.addAction("Attach")
                attachAction.setProperty("sub_window_object_name", self._prop)
                attachAction.triggered.connect(self.parent().onAttachClicked)
                detachAction = menu.addAction("Detach")
                detachAction.setProperty("sub_window_object_name", self._prop)
                detachAction.triggered.connect(self.parent().onDetachClicked)
                menu.exec_(self.mapToGlobal(event.pos()))

                return QToolButton.contextMenuEvent(self, event)        
        
        
        WindowSystemActionHandler.update_from_window_system_ctrl(self)
        self.clear()
        
        for action in self._sub_window_action_list:
            action.setCheckable(True)
            action.toggled.connect(self.action_toggled_slot)
            
            button = SideBarButton(self, action)
            self.side_bar_button_list.append(button)
            self.sub_window_buttons_layout.addWidget(button)
            self.side_bar_button_list.append(button)
        


            
            
class TestforSideBar(QObject):

    def __init__(self, parent=None):
        super(SideBar, self).__init__(parent)

        self.action = QAction(QIcon('test'), 'Test', self)
        self.action.setProperty('linked_widget', )
        


