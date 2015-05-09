'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from . import subscriber, cfg
from PyQt5.QtCore import QObject, pyqtSignal




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
    
    def load(self):
        #fill the sheet :
        
        self._content = cfg.data.main_tree.get_content(self.sheet_id)
        self._title = cfg.data.main_tree.get_title(self.sheet_id)
        self._content_type = cfg.data.main_tree.get_content_type(self.sheet_id)
        self._other_contents = cfg.data.main_tree.get_other_contents(self.sheet_id)
        self._properties = cfg.data.main_tree.get_properties(self.sheet_id)
        self._last_modification_date = cfg.data.main_tree.get_modification_date(self.sheet_id)
        self._creation_date = cfg.data.main_tree.get_creation_date(self.sheet_id)
        self._version = cfg.data.main_tree.get_version(self.sheet_id)
    
    def get_title(self):
        return self._title
    
    def change_title(self, new_title):
        cfg.data.main_tree.set_title(self.sheetid, new_title)


    def get_content(self):
        '''
        function:: get_content(self)
        :param self:
        :rtype: content

        '''
        content = self._content
        return content

    def set_content(self, content):
        '''
        function:: set_content(self, content)
        :param content:
        '''

        pass

    def get_other_contents(self):
        '''
        function:: get_other_contents(self)
        :rtype other_contents:

        '''
        other_contents = self._other_contents
        return other_contents

    def set_other_contents(self, dict_):
        '''
        function:: set_other_contents(self, dict_)

        '''

    def get_content_type(self):
        '''
        function:: get_content_type(self)
        :rtype content_type:

        '''
        content_type = self.content_type
        return content_type
    def set_content_type(self, content_type):
        '''
        function:: set_content_type(self, content_type)
        :param content_type:

        '''
        pass

    def get_properties(self):
        '''
        function:: get_properties(self)
        :rtype properties_dict:

        '''
        properties_dict = self._properties
        return properties_dict

    def set_property(self, key, value): 
        '''
        function:: set_property(self, key, value):
        :param key:
        :param value:
        '''

        pass
    
    def get_modification_date(self):
        '''
        function:: get_modification_date(self)
        :rtype modification_date:
        '''
        modification_date = self._last_modification_date
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
        creation_date = self._creation_date
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
        version = self._version
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
        
    def load(self):
    
        '''
        function:: load(sheet_id)
        '''
        TreeSheet.load(self)          
        
        self.notes = cfg.data.main_tree.get_synopsys(self.sheet_id)
        self.synopsys = cfg.data.main_tree.get_notes(self.sheet_id)
    
        def get_synopsys(self):
            pass
        
        def set_synopsys(self, content):
            pass
        
        def get_notes(self):
            pass
        
        def set_notes(self):
            pass


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
        
        #emit signal to Gui
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
            
        
       
        