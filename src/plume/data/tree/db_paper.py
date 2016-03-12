'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

import sqlite3
from .db_error import DbError


class DbPaper:
    '''
    DbPaper
    '''

    def __init__(self, sql_db: sqlite3.Connection, table_name: str, id_name: str, paper_id: int, commit_state: bool):
        '''
        Constructor
        '''
        self.table_name = table_name
        self.id_name = id_name
        self.a_error = DbError()            # Error status
        self.sql_db = sql_db
        self.paper_id = paper_id
        self.commit_state = commit_state

    def exists(self):
        '''
        function:: exists()
        :param :
        #
        # Returns True if the sheet id exists, False otherwise
        #
        '''

        s_sql = """
        select """ + self.id_name + """
        from """ + self.table_name + """
        where
        """ + self.id_name + """ =:id
        """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.paper_id})
        a_row = a_qry.fetchone()
        if a_row is None:
            return False
        else:
            return True


    def copy(self, prefix: str):
        '''
        function:: copy(prefix: str)
        :param prefix: str:
        '''

        pass

    def commit(self):
        '''
        function:: commit()
        :param :
        #
        # Commits db. Only needed after all other calls if you set b_commit = False in __init__
        #
        '''
        self.sql_db.commit()
        return DbError.R_OK

    def set(self, value_name: str, value):
        '''
        function:: set(value_name: str, value)
        :param value_name: str:
        :param value:
        '''
        s_sql = """
        update """ + self.table_name + """
         set
            """ + value_name + """ =:value ,

            dt_updated = CURRENT_TIMESTAMP
        where
                """ + self.id_name + """ =:id
            """

        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'id': self.paper_id, 'value': value})
        if self.commit_state:
            self.sql_db.commit()

        return DbError.R_OK

    def get(self, value_name: str):
        '''
        function:: get(value_name: str, value)
        :param value_name: str:
        '''

        s_sql = """
            SELECT """ + value_name + """
            FROM """ + self.table_name + """
            WHERE
                """ + self.id_name + """ =:id
        """
        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {"id": self.paper_id})
        a_row = a_curs.fetchone()

        if a_row is None:
            # sheet does not exist
            self.a_error.set_status(DbError.E_RECNOTFOU, 1, 'Paper does not exist')
            return DbError.R_ERROR  # Failed,

        return a_row[0]

    @property
    def sort_order(self)-> int:
        '''
        function:: sort_order()
        '''
        return self.get("l_sort_order")

    @sort_order.setter
    def sort_order(self, value: int):
        '''
        function:: sort_order()
        :param value: int
        '''
        self.set("l_sort_order", value)

    @property
    def indent(self) -> int:
        '''
        function:: indent()
        '''

        return self.get("l_indent")

    @indent.setter
    def indent(self, value: int):
        '''
        function:: indent()
        :param value: int
        '''
        self.set("l_indent", value)

    @property
    def title(self) -> str:
        '''
        function:: title()
        '''
        return self.get("t_title")

    @title.setter
    def title(self, value: str):
        '''
        function:: title()
        :param value: str
        '''
        self.set("t_title", value)

    @property
    def content(self):
        '''
        function:: content()
        '''
        return self.get("m_content")

    @content.setter
    def content(self, value):
        '''

        :param value:
        '''

        self.set("m_content", value)

    @property
    def delete_state(self) -> bool:
        '''

        :return:
        '''

        return self.get("b_delete")

    @delete_state.setter
    def delete_state(self, value: bool):
        '''
        :param value: bool
        :return:
        '''

        self.set("b_delete", value)

    def add(self):
        '''
        function:: add()
        '''

        s_sql = """
        INSERT INTO """ + self.table_name + """
        (
        t_title,
        dt_created,
        dt_updated,
        dt_content,
        l_version,
        b_deleted
        )
        VALUES (
        :title
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        0,
        0
        )
        """

        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'title': "new"})

        self.paper_id = a_curs.lastrowid
        if self.commit_state:
            self.sql_db.commit()
        return self.paper_id

    def list_children(self) -> list:
        s_sql = """
        select l_indent, l_sort_order, b_deleted
        from """ + self.table_name + """
        where
        """ + self.id_name + """ =:id
        ORDER BY l_sort_order
        """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.paper_id})
