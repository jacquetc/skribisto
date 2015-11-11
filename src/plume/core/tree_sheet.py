'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from . import subscriber, cfg
from PyQt5.QtCore import QObject, pyqtSignal
from _csv import Error


class TreeSheetError(OSError):

    '''root for TreeSheetErrors, only used to except any reeSheet error, never raised'''
    pass


class KeyAlreadyPresentError(TreeSheetError):

    '''An error if the name is already present'''
    pass


class TreeSheet(QObject):

    '''
    TreeSheet
    '''

    def __init__(self, parent=None, sheet_id=None):
        '''
        Constructor
        '''

        super(TreeSheet, self).__init__(parent)

        self.sheet_id = sheet_id
        self._content = None
        self._other_contents = None
        self._content_type = ""
        self._creation_date = ""
        self._last_modification_date = ""
        self._properties = {}
        self.parent_id = None
        self.children_id = []
        self._version = -1

        if sheet_id is not None:
            self.load()

        # dict of instances, like for core_part of docks
        self._object_dict = {}
        # dict of class in waiting to be instantiated on demand
        self._class_to_instanciate_dict = {}
        # fill it with plugins
        self._class_to_instanciate_dict = cfg.core_plugins.write_tab_dock_plugin_dict

        self._subscribe_to_data()

    def load(self):
        # fill the sheet :

        self._content = cfg.data.database.main_tree.get_content(self.sheet_id)
        self._title = cfg.data.database.main_tree.get_title(self.sheet_id)
        #self._content_type = cfg.data.database.main_tree.get_content_type(self.sheet_id)
        self._properties = cfg.data.database.main_tree.get_properties(self.sheet_id)
        self._last_modification_date = cfg.data.database.main_tree.get_modification_date(
            self.sheet_id)
        self._creation_date = cfg.data.database.main_tree.get_creation_date(
            self.sheet_id)
        self._version = cfg.data.database.main_tree.get_version(self.sheet_id)

    def _subscribe_to_data(self,  is_subscribing=True):

        list_ = [[self.get_title, "data.tree.title"],
                 [self.get_content, "data.tree.content"],
                 #[self.get_content_type, "data.tree.content_type"],
                 [self.get_properties, "data.tree.properties"],
                 [self.get_modification_date, "data.tree.modification_date"],
                 [self.get_creation_date, "data.tree.creation_date"],
                 [self.get_properties, "data.tree.properties"],
                 [self.get_version, "data.tree.version"],
                 ]
        for func, domain in list_:
            if is_subscribing is True:
                cfg.data.subscriber.subscribe_update_func_to_domain(
                    0, func, domain)
            else:
                cfg.data.subscriber.unsubscribe_update_func_to_domain(0, func)

    def get_instance_of(self, instance_name):
        if instance_name in self._object_dict.keys():
            return self._object_dict[instance_name]

        class_name = self._class_to_instanciate_dict[instance_name]
        instance = class_name()
        self._object_dict[instance_name] = instance
        return instance

    def get_title(self):
        return cfg.data.database.main_tree.get_title(self.sheet_id)

    def set_title(self, new_title):
        cfg.data.database.main_tree.set_title(self.sheet_id, new_title)

    def get_content(self):
        '''
        function:: get_content(self)
        :param self:
        :rtype: content

        '''
        content = cfg.data.database.main_tree.get_content(self.sheet_id)
        return content

    def set_content(self, content):
        '''
        function:: set_content(self, content)
        :param content:
        '''

        cfg.data.database.main_tree.set_content(self.sheet_id, content)

    def get_properties(self):
        '''
        function:: get_properties()
        :rtype properties_dict:

        '''
        return cfg.data.database.main_tree.get_properties(self.sheet_id)

    def set_property(self, key, value):
        '''
        function:: set_property(key, value):
        :param key:
        :param value:
        '''
        self._properties[key] = value
        cfg.data.database.main_tree.set_properties(self.sheet_id, self._properties)

    def change_property_key(self, key, new_key):
        '''
        function:: change_property_key(key, new_key):
        :param key:
        :param new_key:
        '''
        if key not in self._properties.keys():
            value = ""
        else:
            value = self._properties.pop(key)

        self._properties[new_key] = value
        cfg.data.database.main_tree.set_properties(self.sheet_id, self._properties)

    def get_modification_date(self):
        '''
        function:: get_modification_date(self)
        :rtype modification_date:
        '''
        modification_date = cfg.data.database.main_tree.get_modification_date(
            self.sheet_id)
        return modification_date

    def set_modification_date(self, date):
        '''
        function:: set_modification_date(self, date)
        :param date:
        '''

    def get_creation_date(self):
        '''
        function:: get_creation_date(self)
        :rtype creation_date:

        '''
        creation_date = cfg.data.database.main_tree.get_creation_date(self.sheet_id)
        return creation_date

    def set_creation_date(self, date):
        '''
        function:: set_creation_date(self, date)
        :param date:
        '''

        pass

    def get_version(self):
        '''
        function:: get_version(self)
        :rtype version:

        '''
        version = cfg.data.database.main_tree.get_version(self.sheet_id)
        return version


class StoryTreeSheet(TreeSheet):

    '''
    StoryTree
    '''

    def __init__(self, sheet_id=None):
        '''
        Constructor
        '''

        super(StoryTreeSheet, self).__init__(sheet_id=sheet_id)

        self.notes = ""
        self.synopsys = ""

    def get_synopsys(self):
        other_content = TreeSheet.get_other_contents(self)
        if 'synopsys' in other_content.keys():
            return str(other_content['synopsys'])
        else:
            return ''

    def set_synopsys(self, content):
        other_contents = TreeSheet.get_other_contents(self)
        other_contents['synopsys'] = content
        cfg.data.database.main_tree.set_other_contents(self.sheetid, other_contents)

    def get_notes(self):
        other_content = TreeSheet.get_other_contents(self)
        if 'note' in other_content.keys():
            return str(other_content['note'])
        else:
            return ''

    def set_notes(self, content):
        other_contents = TreeSheet.get_other_contents(self)
        other_contents['note'] = content
        cfg.data.database.main_tree.set_other_contents(self.sheetid, other_contents)

    def __del__(self):
        # unsubscribe :
        self._subscribe_to_data(False)


class TreeSheetManager(QObject):

    '''
    TreeSheetManager
    '''
    sheet_is_opening = pyqtSignal(TreeSheet, name='sheet_is_opening')

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(TreeSheetManager, self).__init__(parent)

        self.sheet_list = []

        cfg.data.subscriber.subscribe_update_func_to_domain(
            0, self.close_all_sheets,  "data.project.close")

    def get_tree_sheet_from_sheet_id(self, sheet_id):
        for sheet in self.sheet_list:
            if sheet.sheet_id == sheet_id:
                return sheet
        return None

    def open_sheet(self, sheet_id):
        '''
        function:: open_sheet(sheet_id)
        :param sheet_id:
        :rtype: tree_sheet:

        '''
        for sheet in self.sheet_list:
            if sheet_id == sheet.sheet_id:
                return sheet

        tree_sheet = self.only_load_sheet(sheet_id)
        self.sheet_list.append(tree_sheet)

        # emit signal to Gui
        self.sheet_is_opening.emit(tree_sheet)

        return tree_sheet

    def only_load_sheet(self, sheet_id):
        '''
        function:: only_load_sheet(sheet_id)
        :param sheet_id:
        :rtype: tree_sheet:

        It doesn't add the TreeSheet to the manager. Good if you want only to use a TreeSheet temporarily
        '''
        tree_sheet = TreeSheet(parent=self, sheet_id=sheet_id)

        return tree_sheet

    def close_sheet(self, tree_sheet):
        '''
        function:: close_sheet(tree_sheet)
        :param tree_sheet: id or TreeSheet object
        '''
        if isinstance(tree_sheet, int):
            sheet_id = tree_sheet
            for sheet in self.sheet_list:
                if sheet_id == sheet.sheet_id:
                    self.sheet_list.remove(tree_sheet)

        if isinstance(tree_sheet, TreeSheet):
            self.sheet_list.remove(tree_sheet)
            tree_sheet.deleteLater()

    def close_all_sheets(self):
        for sheet in self.sheet_list:
            self.close_sheet(sheet)
