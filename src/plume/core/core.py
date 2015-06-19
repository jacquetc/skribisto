'''
Created on 3 mars 2015

@author: Cyril Jacquet
'''
from PyQt5.Qt import QObject, pyqtSlot
from . import subscriber, cfg
from .project import Project
from .plugins import Plugins
from .tree_sheet import TreeSheetManager


class Core(QObject):

    '''
    Core
    '''

    def __init__(self, parent=None, data=None):
        '''
        Constructor
        '''
        super(Core, self).__init__(parent)
        cfg.core = self
        cfg.data = data
        self.subscriber = subscriber

        # init all :
        self.project = Project()
        self.plugins = Plugins()
        cfg.core_plugins = self.plugins
        self.tree_sheet_manager = TreeSheetManager()
        self.write_panel_core = WritePanelCore(self)


class WritePanelCore(QObject):

    '''
    WritePanelCore
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(WritePanelCore, self).__init__(parent)

        # dict of instances, like for core_part of docks
        self._object_dict = {}
        # dict of class in waiting to be instantiated on demand
        self._class_to_instanciate_dict = {}
        # fill it with plugins
        self._class_to_instanciate_dict = cfg.core_plugins.write_panel_dock_plugin_dict

    def get_instance_of(self, instance_name):
        if instance_name in self._object_dict.keys():
            return self._object_dict[instance_name]

        class_name = self._class_to_instanciate_dict[instance_name]
        instance = class_name()
        self._object_dict[instance_name] = instance
        return instance
