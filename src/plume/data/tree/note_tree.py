'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''


from .tree import Tree
from .db_paper import DbPaper
from .db_tree import DbTree


class NoteTree(Tree):
    '''
    NoteTree
    '''

    def __init__(self, sql_db):
        '''
        Constructor
        '''

        super(NoteTree, self).__init__(sql_db, "tbl_note", "note", "l_note_id")

    def add_new_child_note(self, parent_note_id: int = -1):
        '''
        function::     int add_new_child_note(parent_note_id: int = -1)
        :param parent_note_id: int = -1:
        '''

        return self.add_new_child_papers(parent_note_id, 1)[0]

    def move_note_to_synopsis(self, parent_note_id):
        '''
        function::     move_note_to_synopsis(parent_note_id)
        :param parent_note_id:
        '''

        pass

    def get_is_synopsis(self, note_id: int):
        '''
        function::     get_is_synopsis(note_id: int)
        :param note_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  note_id, False)
        return bool(a_paper.get("b_synopsis"))

    def set_is_synopsis(self, note_id: int, value: bool):
        '''
        function::     set_is_synopsis(note_id: int, value: bool)
        :param note_id: int:
        :param value: bool:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  note_id, False)
        a_paper.set("b_synopsis", value)

    def find_synopsis_from_sheet_code(self, sheet_code: int):
        """
        Self explanatory
        :param sheet_code: sheet id
        """
        a_tree = DbTree(self.sql_db, self.table_name, self.id_name, False)
        synopsis_list = a_tree.find_ids("b_synopsis", True)
        note_list = a_tree.find_ids("l_sheet_code", sheet_code)

        synopsis_note = list(set(synopsis_list).intersection(note_list))

        return synopsis_note


    def get_sheet_code(self, note_id: int):
        '''
        function::     get_sheet_code(note_id: int)
        :param note_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  note_id, False)
        return int(a_paper.get("l_sheet_code"))

    def set_sheet_code(self, note_id: int, value: int):
        '''
        function::     set_sheet_code(note_id: int, value: int)
        :param note_id: int:
        :param value: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name,  note_id, False)
        a_paper.set("l_sheet_code", value)
