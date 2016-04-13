'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

from .tree import Tree
from .db_paper import DbPaper


class SheetTree(Tree):
    '''
    SheetTree
    '''

    def __init__(self, sql_db):
        '''
        Constructor
        '''
        super(SheetTree, self).__init__(sql_db, "tbl_sheet", "sheet", "l_sheet_id")

    def get_word_count(self, sheet_id: int):
        '''
        function:: get_word_count(paper_id: int)
        :param sheet_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  sheet_id, False)
        return int(a_paper.get("l_word_count"))

    def set_word_count(self, sheet_id: int, count: int):
        '''
        function:: set_word_count(paper_id: int, count: int)
        :param sheet_id: int:
        :param count: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  sheet_id, False)
        a_paper.set("l_char_count", count)

    def get_char_count(self, sheet_id: int):
        '''
        function:: get_char_count(paper_id: int)
        :param sheet_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  sheet_id, False)
        return int(a_paper.get("l_char_count"))

    def set_char_count(self, sheet_id: int, count: int):
        '''
        function:: set_char_count(paper_id: int, count: int)
        :param sheet_id: int:
        :param count: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  sheet_id, False)
        a_paper.set("l_char_count", count)

