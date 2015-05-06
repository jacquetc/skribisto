'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from . import subscriber, cfg

class TreeSheet():
    '''
    TreeSheet
    '''

    def __init__(self, sheet_id=None):
        '''
        Constructor
        '''

        super(TreeSheet, self).__init__()

        self.sheet_id = 0
        self._content = None
        self.content_type = ""
        self._creation_date = ""
        self._last_modification_date = ""
        self._properties = {}
        self.parent_id = 0
        self.children_id = []

    def set_property(self, key, value):
        '''
        function:: set_property(key, value)
        :param key:
        :param value:
        '''

        pass

    def get_property(self, key):
        '''
        function:: get_property(key)
        :param key:
        '''

        pass


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
