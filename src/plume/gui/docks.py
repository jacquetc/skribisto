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
    For now, it's only for tl story_tree docks
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
        
        story_dock_plugin_dict = cfg.gui_plugins.story_dock_plugin_dict
        self.story_dock_type_dict = story_dock_plugin_dict # add there other dicts with built-in docks 

    def split_dock(self, dock):
        '''
        function:: split_dock(dock)
        :param dock:
        '''
        area = self.main_window.dockWidgetArea(dock)
        self.add_dock(dock.current_type, area)
        pass

    def add_dock(self, type_str, area=Qt.RightDockWidgetArea):
        '''
        function:: add_dock(type, zone)
        :param type_str: type of dock
        :param zone:
        '''

        dock = DockTemplate(self, dock_system=self)
        self.change_type(dock, type_str)
        self.dock_list.append(dock)
        
        self.main_window.addDockWidget(area, dock)
        
    def change_type(self, dock, type_str):
        '''
        function:: change_type(dock, type_str)
        :param dock:
        :param type_str:
        '''
        gui_part = self.story_dock_type_dict[type_str]
        gui_part = gui_part()
        gui_part.sheet_id = self.sheet_id
        widget = gui_part.get_widget()
        dock.current_type = type_str
        dock.setWindowTitle(gui_part.dock_displayed_name)
        dock.setWidget(widget)

    def remove_dock(self, dock):
        '''
        function:: remove_dock(dock)
        :param dock:
        '''
        self.dock_list.remove(dock)
        dock.close()
        dock.deleteLater()
    
    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            for dock in self.dock_list: #update all docks' sheet_id
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

    def __init__(self, parent=None, dock_system=None):
        '''
        Constructor
        '''

        super(DockTemplate, self).__init__(parent=None)
        self.current_type = None
        title_widget = DockTitleWidget(self, self)
        self.setTitleBarWidget(title_widget)
        self.dock_system = dock_system

    @property
    def dock_system(self):
        return self._dock_system
    
    @dock_system.setter
    def dock_system(self, dock_system):
        if dock_system is not None:
            self._dock_system = dock_system
            self.titleBarWidget().fill_comboBox_with_types()

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
        self.ui = Ui_DockTitleBar()
        self.ui.setupUi(self)
        
    @pyqtSlot()
    def on_closeButton_clicked(self):
        self.parent_dock.close()
    
    @pyqtSlot()
    def on_addDockButton_clicked(self):
        self.parent_dock.dock_system.split_dock(self.parent_dock)
        
    def fill_comboBox_with_types(self):
        #types_name_dict = {}
        gui_parts = self.parent_dock.dock_system.story_dock_type_dict.values()
        for part in gui_parts:
            #types_name_dict[part.dock_displayed_name] = part.dock_name
            self.ui.comboBox.addItem(part.dock_displayed_name, userData=part.dock_name)
    
    @pyqtSlot(int)    
    def on_comboBox_currentIndexChanged(self, index):
        print(self.ui.comboBox.itemData(index, Qt.UserRole))
        
        
        