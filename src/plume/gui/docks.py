'''
Created on 7 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QDockWidget, QWidget
from PyQt5.Qt import pyqtSlot, QObject
from . import cfg
from PyQt5.QtCore import Qt

class DockSystem(QObject):
    '''
    DockSystem
    '''

    def __init__(self, parent, main_window):
        '''
        Constructor
        '''

        super(DockSystem, self).__init__(parent)
        self._sheet_id = None
        self.main_window = main_window
        self.dock_list = []
        self._default_dock = "properties-dock" 
        
        self.story_dock_plugin_dict = cfg.gui_plugins.story_dock_plugin_dict
        

    def split_dock(self, dock):
        '''
        function:: split_dock(dock)
        :param dock:
        '''

        pass

    def add_dock(self, type_str, area=Qt.RightDockWidgetArea):
        '''
        function:: add_dock(type, zone)
        :param type_str: type of dock
        :param zone:
        '''
        gui_part = self.story_dock_plugin_dict[type_str]
        widget = gui_part().get_widget()
        dock = DockTemplate(self)
        dock.setWindowTitle(gui_part.dock_displayed_name)
        self.dock_list.append(dock)
        dock.setWidget(widget)
        
        self.main_window.addDockWidget(area, dock)
    
    def remove_dock(self, dock):
        '''
        function:: remove_dock(dock)
        :param dock:
        '''

        pass
    
    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            for dock in self.dock_list:
                dock.widget().gui_part.sheet_id = sheet_id
         
        
    def load_settings(self):
        '''
        function:: load_settings()
        :param :
        '''

        pass

    def save_settings(self):
        '''
        function:: save_settings()
        :param :
        '''

        pass



class DockTemplate(QDockWidget):
    '''
    DockTemplate
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(DockTemplate, self).__init__(parent=None)
        
        title_widget = DockTitleWidget(self, self)
        self.setTitleBarWidget(title_widget)



from .dock_title_bar_ui import Ui_DockTitleBar


class DockTitleWidget(QWidget):
    '''
    DockTitleWidget
    '''

    def __init__(self, parent_dock, parent=None):
        '''
        Constructor
        '''

        super(DockTitleWidget, self).__init__(parent=None)
        self.parent_dock = parent_dock
        ui = Ui_DockTitleBar()
        ui.setupUi(self)
        
    @pyqtSlot()
    def on_closeButton_clicked(self):
        self.parent_dock.close()
