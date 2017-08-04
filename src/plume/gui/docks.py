'''
Created on 7 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QDockWidget, QWidget
from PyQt5.Qt import pyqtSlot, QObject, QSettings, QSize, QByteArray
from . import cfg
from PyQt5.QtCore import Qt
from enum import Enum


class DockSystem(QObject):

    '''

    DockSystem
    For now, it's only for the write_tree docks
    '''
    DockTypes = Enum('DockType', 'WriteSubWindowDock WritePanelDock NotePanelDock NoteSubWindowDock')

    def __init__(self, parent, main_window,  dock_type):
        '''
        Constructor
        '''

        super(DockSystem, self).__init__(parent)
        self._paper_id = None
        self._project_id = None
        self.main_window = main_window
        self.dock_list = []
        self.dock_type = dock_type
        self.loading_settings = False

        if dock_type is self.DockTypes.WriteSubWindowDock:
            self._default_dock = "synopsis-dock"
            write_subwindow_dock_plugin_dict = cfg.gui_plugins.write_subwindow_dock_plugin_dict
            # add there other dicts with built-in docks
            self.dock_type_dict = write_subwindow_dock_plugin_dict

        if dock_type is self.DockTypes.WritePanelDock:
            self._default_dock = "write-tree-dock"
            write_panel_dock_plugin_dict = cfg.gui_plugins.write_panel_dock_plugin_dict
            # add there other dicts with built-in docks
            self.dock_type_dict = write_panel_dock_plugin_dict

        if dock_type is self.DockTypes.NoteSubWindowDock:
            self._default_dock = "note-properties-dock"
            note_subwindow_dock_plugin_dict = cfg.gui_plugins.note_subwindow_dock_plugin_dict
            # add there other dicts with built-in docks
            self.dock_type_dict = note_subwindow_dock_plugin_dict

        if dock_type is self.DockTypes.NotePanelDock:
            self._default_dock = "note-tree-dock"
            note_panel_dock_plugin_dict = cfg.gui_plugins.note_panel_dock_plugin_dict
            # add there other dicts with built-in docks
            self.dock_type_dict = note_panel_dock_plugin_dict




    def split_dock(self, dock):
        '''
        function:: split_dock(dock)
        :param dock:
        '''
        area = self.main_window.dockWidgetArea(dock)
        self.add_dock(dock.current_type, area)


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
        gui_part = self.dock_type_dict[type_str]
        gui_part = gui_part()
        gui_part.project_id = self.project_id
        gui_part.paper_id = self.paper_id
        widget = gui_part.get_widget()
        dock.current_type = type_str
        dock.setWindowTitle(gui_part.dock_displayed_name)
        dock.setObjectName(gui_part.dock_displayed_name)
        dock.setWidget(widget)
        dock.titleBarWidget().ui.comboBox.setCurrentText(
            gui_part.dock_displayed_name)

    def remove_dock(self, dock):
        '''
        function:: remove_dock(dock)
        :param dock:
        '''
        self.dock_list.remove(dock)
        dock.close()
        dock.deleteLater()

    def clear(self):
        for dock in self.dock_list:
            self.remove_dock(dock)
        self.dock_list = []


    @property
    def paper_id(self):
        return self._paper_id

    @paper_id.setter
    def paper_id(self, paper_id):
        self._paper_id = paper_id
        if self._paper_id is not None:
            for dock in self.dock_list:  # update all docks' sheet_id
                dock.widget().gui_part.paper_id = paper_id

    @property
    def project_id(self):
        return self._project_id

    @project_id.setter
    def project_id(self, project_id):
        self._project_id = project_id
        if self._project_id is not None:
            for dock in self.dock_list:  # update all docks' paper_id
                dock.widget().gui_part.project_id = project_id

    def load_settings(self):
        '''
        function:: load_settings()
        :param :
        '''
        self.loading_settings = True

        self.clear()

        settings = QSettings(self)
        settings.beginGroup("Docks")
        array_size = settings.beginReadArray(str(self.dock_type))
        # apply number limit
        # if array_size > 5:
        #     array_size = self.size_limit
        # load:
        size_list = []
        for i in range(array_size):
            settings.setArrayIndex(i)
            _type = settings.value("type")
            area = settings.value("area", type=Qt.DockWidgetArea)
            size_list.append(settings.value("size", type=QSize))

            self.add_dock(_type, area)

        settings.endArray()
        settings.endGroup()
        for i in range(len(self.dock_list)):
            self.dock_list[i].resize(size_list[i])

        if not self.dock_list:
            self.add_dock(self._default_dock)

        self.main_window.restoreState(settings.value("Docks/state/" + str(self.dock_type), 0, type=QByteArray))

        self.loading_settings = False


    def save_settings(self):
        '''
        function:: save_settings()
        :param :
        '''

        settings = QSettings(self)

        settings.beginGroup("Docks/" + str(self.dock_type))
        settings.remove("")
        settings.endGroup()

        settings.beginGroup("Docks")
        settings.beginWriteArray(str(self.dock_type), len(self.dock_list))
        for i in range(len(self.dock_list)):
            dock = self.dock_list[i]
            settings.setArrayIndex(i)
            settings.setValue("type", dock.current_type)
            settings.setValue("area", int(self.main_window.dockWidgetArea(dock)))
            settings.setValue("size", dock.size())

        settings.endArray()
        settings.endGroup()

        settings.setValue("Docks/state/" + str(self.dock_type), self.main_window.saveState())

from PyQt5.Qt import QTimer

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

        self.setFeatures((self.features() | QDockWidget.DockWidgetMovable) & ~QDockWidget.DockWidgetFloatable)


    @property
    def dock_system(self):
        return self._dock_system

    @dock_system.setter
    def dock_system(self, dock_system):
        if dock_system is not None:
            self._dock_system = dock_system
            self.titleBarWidget().fill_comboBox_with_types()

    def resizeEvent(self, event):
        if not self._dock_system.loading_settings:
            QTimer.singleShot(0, self.dock_system.save_settings)
        QDockWidget.resizeEvent(self, event)

    def closeEvent(self, event):
        if not self._dock_system.loading_settings:
            self.dock_system.remove_dock(self)
            QTimer.singleShot(0, self.dock_system.save_settings)
        QDockWidget.closeEvent(self, event)

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
        if self.parent_dock is None:
            return
        self.parent_dock.dock_system.split_dock(self.parent_dock)

    def fill_comboBox_with_types(self):
        # types_name_dict = {}
        gui_parts = self.parent_dock.dock_system.dock_type_dict.values()
        for part in gui_parts:
            # types_name_dict[part.dock_displayed_name] = part.dock_name
            self.ui.comboBox.addItem(
                part.dock_displayed_name, userData=part.dock_name)

    @pyqtSlot(int)
    def on_comboBox_currentIndexChanged(self, index):
        # print(self.ui.comboBox.itemData(index, Qt.UserRole))

        if self.parent_dock is None:
            return
        dock_type = self.ui.comboBox.itemData(index, Qt.UserRole)
        self.parent_dock.dock_system.change_type(self.parent_dock,  dock_type)

        if not self.parent_dock.dock_system.loading_settings:
            self.parent_dock.dock_system.save_settings()
