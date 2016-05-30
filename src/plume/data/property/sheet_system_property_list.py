'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

from .property_list import PropertyList
from .db_property import DbProperty


class SheetSystemPropertyList(PropertyList):
    '''
    SheetPropertyList
    '''

    def __init__(self, sql_db):
        '''
        Constructor
        '''
        super(SheetSystemPropertyList, self).__init__(sql_db, "tbl_sheet_system_property", "sheet_system_property"
                                                , "l_property_id", "l_sheet_code")
