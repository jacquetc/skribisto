'''
Created on 15 february 2015

@author:  Cyril Jacquet
'''

import os
import sqlite3

class Exporter:
    def __init__(self):
        super(Exporter, self).__init__()

    @staticmethod
    def export_sqlite_db_to(sqlite_db: sqlite3.Connection, file_type: str, file_name: str):
        if "*.sqlite" in file_type:
            if not file_name.endswith(".sqlite"):
                file_name = "".join([file_name, ".sqlite"])
            # delete if already exist:
            if os.path.exists(file_name):
                os.remove(file_name)
            on_disk_db = sqlite3.connect(file_name)
            on_disk_db.executescript("CREATE TABLE table_name (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT);")
            on_disk_db.executescript("DROP TABLE IF EXISTS table_name")
            query = "".join(line for line in sqlite_db.iterdump())
            on_disk_db.executescript(query)
