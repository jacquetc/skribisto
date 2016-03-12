'''
Created on 15 february 2015

@author:  Cyril Jacquet
'''

import os
import sqlite3


class Importer:
    def __init__(self):
        super(Importer, self).__init__()


    @staticmethod
    def create_sqlite_db_from( file_type: str, file_name: str)-> sqlite3.Connection :
        if file_name.endswith(".sqlite"):
            old_db = sqlite3.connect(file_name)
            new_db = sqlite3.connect(':memory:')  # create a memory database
            new_db.executescript("CREATE TABLE table_name (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT);")
            new_db.executescript("DROP TABLE IF EXISTS table_name")
            query = "".join(line for line in old_db.iterdump())

            # Dump old database in the new one.
            new_db.executescript(query)

            return new_db

    @staticmethod
    def create_empty_sqlite_db()-> sqlite3.Connection :
        database = sqlite3.connect(":memory:")
        cursor = database.cursor()
        # TODO fill
        cursor.execute(
            "CREATE TABLE tbl_sheet(key TEXT primary key, value TEXT)")
        cursor.execute("INSERT INTO project_table VALUES('name', 'test')")
        database.commit()

        return database