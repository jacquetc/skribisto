'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

import sqlite3
from .db_property import DbProperty
from .db_error import DbError


class DbPropertyList:
    '''
    DbPropertyList
    '''

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
        from """ + self.table_name

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

    def move_list(self, id_list: list, property_id: int)-> int:
        '''
        function:: move_list(id_list, property_id: int)
        :param id_list:
        :param property_id: int:
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
        if type(property_id) != int:
            self.error.set_status(DbError.E_INVTYPE, 2, 'Must be int')
            return DbError.R_ERROR         # Invalid type

        # sanity check. Max move is renum_interval -1
        if len(id_list) > (DbPropertyList.renum_interval - 1):
            self.error.set_status(DbError.E_INTERNERR, 0, "Can't move more than {0} propertys".format(DbPropertyList.renum_interval - 1))
            return DbError.R_ERROR         # Too many sheets to move

        if property_id == 0:
            i_dest = 0                  # First sheet order
        elif property_id == -1:
            i_dest = 2**30              # hopefully greater than last sheet order!
        else:
            a_sh = DbProperty(self.sql_db, self.table_name, self.id_name, property_id, False)
            i_dest = a_sh.sort_order
            if i_dest == DbError.R_ERROR:
                self.error.set_status(DbError.E_INVPARAM, 2, 'After Property ({0}) not found'.format(property_id))
                return DbError.R_ERROR         # Invalid value

        # Renumber each sheet to +1 after dest
        i_dest += 1
        for i_id in id_list:
            a_sh = DbProperty(self.sql_db, self.table_name, self.id_name, i_id, False)
            if not a_sh.exists():
                self.sql_db.rollback()
                self.error.set_status(DbError.E_INVPARAM, 1, 'Property ({0}) not found'.format(i_id))
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

    def remove_list(self, id_list: list):
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
            a_property = DbProperty(self.sql_db, self.table_name, self.id_name, i_id, False)
            if not a_property.exists():
                self.sql_db.rollback()
                self.error.set_status(DbError.E_INVPARAM, 1, 'Property ({0}) not found'.format(i_id))
                return DbError.R_ERROR         # Invalid value
            # Delete
            if not a_property.remove():
                return DbError.R_ERROR

        return DbError.R_OK

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

    def find_id(self, code_name: str, paper_code: int, name: str):

        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
        where
            t_name = :property_name
            AND
            """ + code_name + """= :paper_code"""

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'property_name': name, 'paper_code': paper_code})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row[0])

        if a_list == []:
            result = None
        else:
            result = a_list[0]

        return result
