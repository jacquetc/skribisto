import sqlite3
from .exceptions import DbErr


class DbProperty:
    #
    # A class to manipulate properties
    #
    def __init__(self, table_name: str,
                 code_column_name: str, a_db: sqlite3.Connection, i_item_code: int, t_name: str, b_commit: bool):
        self._table_name = table_name
        self._code_column_name = code_column_name
        self._a_db = a_db
        self._b_commit = b_commit

        self._item_code = i_item_code
        self._name = t_name
        self._value = None
        self._created = None
        self._updated = None
        self._is_system = True

    @property
    def item_code(self):
        return self._item_code

    @item_code.setter
    def item_code(self, item_code: int):
        s_sql = """
        update
            """ + self._table_name + """
        set
            """ + self._code_column_name + """ = :content,
            dt_updated = CURRENT_TIMESTAMP
        where
            """ + self._code_column_name + """ = :code
            AND t_name = :name
            """

        a_curs = self._a_db.cursor()
        a_curs.execute(s_sql, {'code': self._item_code, 'content': item_code, 'name': self._name})
        if self._b_commit:
            self._a_db.commit()

        self._item_code = item_code
        return DbErr.R_OK

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, new_name: str):
        #
        # Updates the name + timestamp
        #
        s_sql = """
        update
            """ + self._table_name + """
        set
            t_name = :content,
            dt_updated = CURRENT_TIMESTAMP
        where
            """ + self._code_column_name + """ = :code
            AND t_name = :old_name
            """

        a_curs = self._a_db.cursor()
        a_curs.execute(s_sql, {'code': self._item_code, 'content': new_name, 'old_name': self._name})
        if self._b_commit:
            self._a_db.commit()

        self._name = new_name
        return DbErr.R_OK

    @property
    def value(self):
        if self._value is None:
            s_sql = """
                SELECT
                    t_value
                FROM
                """ + self._table_name + """
                WHERE
                """ + self._code_column_name + """ = :code
                AND t_name = :name
                 """
            a_curs = self._a_db.cursor()
            a_curs.execute(s_sql, {'code': self._item_code, 'name': self._name})
            result = a_curs.fetchone()
            for row in result:
                self._value = row
        return self._value

    @value.setter
    def value(self, new_value: str):
        s_sql = """
        update
            """ + self._table_name + """
        set
            t_value = :content,
            dt_updated = CURRENT_TIMESTAMP
        where
            """ + self._code_column_name + """ = :code
            AND t_name = :name
            """

        a_curs = self._a_db.cursor()
        a_curs.execute(s_sql, {'code': self._item_code, 'content': new_value, 'name': self._name})
        if self._b_commit:
            self._a_db.commit()

        self._value = new_value
        return DbErr.R_OK


    @property
    def is_system(self):
        return self._is_system

    @is_system.setter
    def is_system(self, is_system: bool):
        s_sql = """
        update
            """ + self._table_name + """
        set
            t_value = :content,
            dt_updated = CURRENT_TIMESTAMP
        where
            """ + self._code_column_name + """ = :code
            AND t_name = :name
            """

        a_curs = self._a_db.cursor()
        a_curs.execute(s_sql, {'code': self._item_code, 'content': is_system, 'name': self._name})
        if self._b_commit:
            self._a_db.commit()

        self._is_system = is_system
        return DbErr.R_OK

    @property
    def date_of_creation(self):
        if self._created is None:
            s_sql = """
                SELECT
                    dt_created
                FROM
                """ + self._table_name + """
                WHERE
                """ + self._code_column_name + """ = :code
                AND t_name = :name
                 """
            a_curs = self._a_db.cursor()
            a_curs.execute(s_sql, {'code': self._item_code, 'name': self._name})
            result = a_curs.fetchone()
            for row in result:
                self._created = row
        return self._created

    @property
    def date_of_update(self):
        if self._updated is None:
            s_sql = """
                SELECT
                    dt_updated
                FROM
                """ + self._table_name + """
                WHERE
                """ + self._code_column_name + """ = :code
                AND t_name = :name
                 """
            a_curs = self._a_db.cursor()
            a_curs.execute(s_sql, {'code': self._item_code, 'name': self._name})
            result = a_curs.fetchone()
            for row in result:
                self._updated = row
        return self._updated

    def commit(self):
        #
        # Commits db. Only needed after all other calls if you set b_commit = False in __init__
        #
        self._a_db.commit()
        return DbErr.R_OK

    def add(self):
        s_sql = """
            insert into
                """ + self._table_name + """
            (
                """ + self._code_column_name + """,
                t_name,
                dt_created,
                dt_updated,
                b_system
            )
            values(
                :item_code;
                :name,
                :value,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                1
            )
            """

        a_curs = self._a_db.cursor()
        a_curs.execute(s_sql, {'code': self._item_code, 'name': self._name, 'value': self._value})

        if self._b_commit:
            self._a_db.commit()
        return self.property()

    def property(self):
        return Property(self._item_code, self._name, self._value, self._created, self._updated, self._is_system)


class Property:
    #
    # A class to store property content
    #
    def __init__(self, i_tree_item_code, t_name, t_value, dt_created, dt_updated, b_system):
        self._item_code = i_tree_item_code
        self._name = t_name
        self._value = t_value
        self._created = dt_created
        self._updated = dt_updated
        self._is_system = b_system

    @property
    def name(self):
        return self._name

    @property
    def value(self):
        return self._value

    @property
    def item_code(self):
        return self._item_code

    @property
    def created(self):
        return self._created

    @property
    def updated(self):
        return self._updated

    @property
    def is_system(self):
        return self._is_system

