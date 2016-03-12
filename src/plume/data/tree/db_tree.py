'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

import sqlite3
from .db_paper import DbPaper
from .db_error import DbError


class DbTree:
    '''
    DbTree
    '''

    renum_interval = 1000

    def __init__(self, sql_db: sqlite3.Connection, table_name: str, id_name: str, commit_state: bool):
        '''
        Constructor
        '''

        self.table_name = table_name
        self.id_name = id_name
        self.sql_db = sql_db
        self.version = 0
        self.error = DbError()
        self.commit_state = commit_state

    def get_all(self)-> list:
        """
        function:: get_all()
        :rtype: [{"name": value, "name_1": value1},...]
        """

        header_list = self.get_all_headers()

        s_sql = """
        select """ + ', '.join(header_list) + """
        from """ + self.table_name + """
        order by
            l_sort_order
            """
        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {})
        a_rows = a_qry.fetchall()
        final_list = []
        for a_row in a_rows:
            name_value_dict = {}
            for i in range(len(header_list)):
                header = header_list[i]
                value = a_row[i]
                name_value_dict[header] = value
            final_list.append(name_value_dict)

        return final_list

    def get_all_headers(self)-> list:
        """
        function:: get_all_headers()
        :rtype: [name, name,...]
        """

        s_sql = """
        PRAGMA table_info(""" + self.table_name + """)
            """
        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {})
        a_rows = a_qry.fetchall()
        header_list = []
        for a_row in a_rows:
            header = a_row[1]
            header_list.append(header)

        return header_list


    def renum_all(self):
        '''
        function:: renum_all()
        Renumber all non-deleted sheet in this version. DOES NOT COMMIT - Caller should
        '''

        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
        where
            l_version = :ver
        order by
            l_sort_order
            """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.version})
        a_rows = a_qry.fetchall()
        i_dest = DbTree.renum_interval
        for a_row in a_rows:
            # For each sheet to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_paper = DbPaper(self.sql_db, self.table_name, self.id_name, a_row[0], False)
            a_paper. sort_order = i_dest
            i_dest += DbTree.renum_interval

        if self.commit_state:
            self.sql_db.commit()

        return DbError.R_OK

    def move_list(self, id_list: list, paper_id: int)-> int:
        '''
        function:: move_list(id_list, paper_id: int)
        :param id_list:
        :param paper_id: int:
        # Move the list of sheets to after i_after_sheet.
        #
        # If i_after_sheet is 0, then move the sheets to the top. -1 = move sheets to bottom
        #
        # The sheets are moved in list order such at the first in the list ends up as the first after i_after_sheet.
        #
        # To move, we renumber sheets from i_dest + 1. The max sheets we can move is renum_interval -1.
        #
        '''

        if type(id_list) != list:
            self.error.set_status(DbError.E_INVTYPE, 1, 'Must be list')
            return DbError.R_ERROR         # Invalid type
        if type(paper_id) != int:
            self.error.set_status(DbError.E_INVTYPE, 2, 'Must be int')
            return DbError.R_ERROR         # Invalid type

        # sanity check. Max move is renum_interval -1
        if len(id_list) > (DbTree.renum_interval - 1):
            self.error.set_status(DbError.E_INTERNERR, 0, "Can't move more than {0} papers".format(DbTree.renum_interval - 1))
            return DbError.R_ERROR         # Too many sheets to move

        if paper_id == 0:
            i_dest = 0                  # First sheet order
        elif paper_id == -1:
            i_dest = 2**30              # hopefully greater than last sheet order!
        else:
            a_sh = DbPaper(self.sql_db, self.table_name, self.id_name, paper_id, False)
            i_dest = a_sh.sort_order
            if i_dest == DbError.R_ERROR:
                self.error.set_status(DbError.E_INVPARAM, 2, 'After Paper ({0}) not found'.format(paper_id))
                return DbError.R_ERROR         # Invalid value

        # Renumber each sheet to +1 after dest
        i_dest += 1
        for i_id in id_list:
            a_sh = DbPaper(self.sql_db, self.table_name, self.id_name, i_id, False)
            if not a_sh.exists():
                self.sql_db.rollback()
                self.error.set_status(DbError.E_INVPARAM, 1, 'Paper ({0}) not found'.format(i_id))
                return DbError.R_ERROR         # Invalid value

            a_sh.sort_order = i_dest
            i_dest += 1

        # Renumber all sheets to restore default spacing
        #???    self.renum_all()
        self.sql_db.commit()

        return DbError.R_OK

    def copy_list(self, id_list):
        '''
        function:: copy_list(id_list)
        :param id_list:
        '''

        pass

    def delete_list(self, id_list: list):
        '''
        function:: delete_list(id_list)
        :param id_list:
        #
        # Mark a list of sheets as deleted.
        # This function just sets the deleted flag in the record. Call empty_trash to really delete the records
        #
        '''

        if type(id_list) != list:
            self.error.set_status(DbError.E_INVTYPE, 1, 'Must be list')
            return DbError.R_ERROR         # Invalid type

        # Delete each sheet
        for i_id in id_list:
            a_paper = DbPaper(self.sql_db, self.table_name, self.id_name, i_id, False)
            if not a_paper.exists():
                self.sql_db.rollback()
                self.error.set_status(DbError.E_INVPARAM, 1, 'Paper ({0}) not found'.format(i_id))
                return DbError.R_ERROR         # Invalid value
            # Delete
            a_paper.delete = True

        self.sql_db.commit()
        return DbError.R_OK


    def undelete_list(self, id_list):
        '''
        function:: undelete_list(id_list)
        :param id_list:
        Permanently remove any deleted sheets for the CURRENT version

        '''

        if type(id_list) != list:
            self.error.set_status(DbError.E_INVTYPE, 1, 'Must be list')
            return DbError.R_ERROR         # Invalid type

        # Delete each sheet
        for i_id in id_list:
            a_paper = DbPaper(self.sql_db, self.table_name, self.id_name, i_id, False)
            if not a_paper.exists():
                self.sql_db.rollback()
                self.error.set_status(DbError.E_INVPARAM, 1, 'Paper ({0}) not found'.format(i_id))
                return DbError.R_ERROR         # Invalid value
            # Delete
            a_paper.delete = False

        self.sql_db.commit()
        return DbError.R_OK


    def empty_trash(self):
        '''
        function:: empty_trash()
         #
        # Permanently remove any deleted sheets for the CURRENT version
        #

        # Delete records
        '''

        s_sql = """
        delete from
        """ + self.table_name
        """
         where
            b_deleted = 1
            and l_version = :ver
            """
        self.sql_db.execute(s_sql, {'ver': self.version})
        self.sql_db.commit()

        return DbError.R_OK

    def list_visible_id(self):
        '''
        function:: list_visible_id()
        #
        #  Returns a list of visible sheet ids (i.e. not deleted) for the current version
        #
        '''

        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
        where
            b_deleted = 0
            and l_version = :ver
        order by
            l_sort_order
            """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row[self.id_name])

        return a_list

    def list_trash(self):
        '''
        function:: list_trash()
        '''
        #
        #  Returns a list of deleted sheet ids for the current version
        #
        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
        where
            b_deleted = 1
            and l_version = :ver
        order by
            l_sort_order
            """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row[self.id_name])

        return a_list


    def list_all(self):
        '''
        function:: list_all()
        #
        #  Returns a list of ALL sheet ids. Probably only useful for testing?
        #
        '''

        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
         order by
            l_sort_order
            """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row[self.id_name])

        return a_list

        pass

    def get_paper_above(self, paper_id: int):
        '''
        function:: get_paper_above(paper_id: int)
        :param paper_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name, paper_id, False)
        if not a_paper.exists():
            self.sql_db.rollback()
            self.error.set_status(DbError.E_INVPARAM, 1, 'Paper ({0}) not found'.format(paper_id))
            return DbError.R_ERROR
        if a_paper.sort_order == self.renum_interval:
            return 0

        return self.list_visible_id()[self.list_visible_id().index(paper_id) - 1]

    def get_paper_below(self, paper_id: int):
        '''
        function:: getPaperBelow(paper_id: int)
        :param paper_id: int:
        '''
        a_paper = DbPaper(self.sql_db, self.table_name, self.id_name, paper_id, False)
        if not a_paper.exists():
            self.sql_db.rollback()
            self.error.set_status(DbError.E_INVPARAM, 1, 'Paper ({0}) not found'.format(paper_id))
            return DbError.R_ERROR

        if a_paper.sort_order == self.list_all()[-1]:
            return -1

        return self.list_visible_id()[self.list_visible_id().index(paper_id) - 1]
