'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

import sqlite3
from .db_error import DbError


class DbProperty:
    '''
    DbProperty
    '''

    def __init__(self, sql_db: sqlite3.Connection, table_name: str, id_name: str
                 , property_id: int, commit_state: bool):
        '''
        Constructor
        '''
        self.table_name = table_name
        self.id_name = id_name
        self.a_error = DbError()  # Error status
        self.sql_db = sql_db
        self.property_id = property_id
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
        a_qry = a_curs.execute(s_sql, {'id': self.property_id})
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
        a_curs.execute(s_sql, {'id': self.property_id, 'value': value})
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
        a_curs.execute(s_sql, {"id": self.property_id})
        a_row = a_curs.fetchone()

        if a_row is None:
            # sheet does not exist
            self.a_error.set_status(DbError.E_RECNOTFOU, 1, 'Property does not exist')
            return DbError.R_ERROR  # Failed,

        return a_row[0]

    @property
    def name(self) -> str:
        '''
        function:: name()
        '''
        return self.get("t_name")

    @name.setter
    def name(self, value: str):
        '''
        function:: name()
        :param value: str
        '''
        self.set("t_name", value)

    @property
    def value(self):
        '''
        function:: value()
        '''
        return self.get("t_value")

    @value.setter
    def value(self, property_value):
        '''
        function:: value()
        :param property_value:
        '''

        self.set("t_value", property_value)

    def add(self, code_name: str, paper_id: int):
        '''
        function:: add()
        :param paper_id:
        :return:
        '''

        s_sql = """
        INSERT INTO """ + self.table_name + """
        (
        """ + code_name + """,
        t_name,
        t_value,
        dt_created,
        dt_updated,
        b_system
        )
        VALUES (
        :code_id,
        :name,
        "",
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        0
        )
        """

        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'code_id': paper_id, 'name': "new"})

        self.property_id = a_curs.lastrowid
        if self.commit_state:
            self.sql_db.commit()
        return self.property_id

    def remove(self):

        s_sql = """
        delete from
        """ + self.table_name + """
        where
        """ + self.id_name + """ = :property_id
        """

        a_curs = self.sql_db.cursor()
        a_curs.execute(s_sql, {'property_id': self.property_id})

        if self.commit_state:
            self.sql_db.commit()

        return DbError.R_OK