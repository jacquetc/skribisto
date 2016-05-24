'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

from .property_list import PropertyList
from .db_property import DbProperty


class SheetPropertyList(PropertyList):
    '''
    SheetPropertyList
    '''

    def __init__(self, sql_db):
        '''
        Constructor
        '''
        super(SheetPropertyList, self).__init__(sql_db, "tbl_sheet_property", "sheet_property"
                                                , "l_property_id", "l_sheet_code")
