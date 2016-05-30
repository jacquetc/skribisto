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
        a_qry = a_curs.execute(s_sql, {'id': self.paper_id})
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

        return self.get("b_deleted")

    @delete_state.setter
    def delete_state(self, value: bool):
        '''
        :param value: bool
        :return:
        '''

        self.set("b_deleted", value)

    def add(self, imposed_id: int = -1):
        '''
        function:: add()
        '''
        if imposed_id == -1:
            id = ""
            id_value = ""
        else:
            id = self.id_name + ","
            id_value = str(imposed_id) + ","

        s_sql = """
        INSERT INTO """ + self.table_name + """
        (
        """ + id + """
        t_title,
        dt_created,
        dt_updated,
        dt_content,
        l_version,
        b_deleted
        )
        VALUES (
        """ + id_value + """
        :title,
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
        children = []
        # Get current sheet values. I'm sure you can do this in a single query...
        s_sql = """
        select
            l_indent,
            l_sort_order
        from """ + self.table_name + """
        where
        """ + self.id_name + """ =:id
            and
            b_deleted =:deleted
        """

        a_curs = self.sql_db.cursor()
        a_qry = a_curs.execute(s_sql, {'id': self.paper_id, 'deleted': self.delete_state})

        a_row = a_qry.fetchone()
        if a_row is None:
            # paper does not exist
            self.a_error.set_status(DbError.E_RECNOTFOU, 1, 'Paper does not exist')
            return DbError.R_ERROR  # Failed,

        i_parent_order = a_row[1]
        i_parent_indent = a_row[0]

        # Now get children that appear AFTER this sheet (sort Order > current)
        s_sql = """
        select
            """ + self.id_name + """ ,
            l_indent
        from
        """ + self.table_name + """
        where
            l_sort_order > :sort
            and
            b_deleted =:deleted
        order by
            l_sort_order
        """
        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'sort': i_parent_order, 'deleted': self.delete_state})

        for a_row in a_curs:
            if a_row[1] <= i_parent_indent:
                break       # No more children
            children.append(a_row[0])

        return children

    def remove(self):

        """
        remove completly
        :return:
        """
        s_sql = """
         delete from
         """ + self.table_name + """
         where
         """ + self.id_name + """ = :paper_id
         """

        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'paper_id': self.paper_id})

        if self.commit_state:
            self.sql_db.commit()

        # TODO: remove children

        return DbError.R_OK