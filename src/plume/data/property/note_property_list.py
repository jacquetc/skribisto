'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

from .property_list import PropertyList
from .db_property import DbProperty


class NotePropertyList(PropertyList):
    '''
    SheetPropertyList
    '''

    def __init__(self, sql_db):
        '''
        Constructor
        '''
        super(NotePropertyList, self).__init__(sql_db, "tbl_note_property", "note_property"
                                                , "l_property_id", "l_note_code")
