'''
Created on 19 aug. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtCore import QObject
from . import subscriber, cfg
from .database import Database
from .exporter import Exporter


class Data(QObject):
    def __init__(self, parent = None):
        super(Data, self).__init__(parent)

        self.subscriber = subscriber

        cfg.data = self
        self.database = None
        self.database_for_int_dict = {}

    # def open_project(self, path,  project_type, index=0):
    #     if project_type is "sqlite3":
    #
    #         #create db in temp folder
    #         db = Database()
    #         self.database = db
    #         #load database
    #         db.project.load(path)
    #
    # def open_test_project(self):
    #     db = Database()
    #     self.database = db
    #     db.project.load_test_project_db()

    def create_new_empty_database(self, project_id: int):
        '''
        function:: create_new_empty_database(project_id: int)
        :param project_id: int:
        '''
        self.load_database(project_id, "")

    def load_database(self, project_id: int, file_name: str):
        '''
        function:: load_database(project_id: int, file_name: str)
        :param project_id: int:
        :param file_name: str:
        '''

        db = Database(project_id, file_name)
        self.database_for_int_dict[project_id] = db
        if project_id == 0:
            self.database = db
        self.subscriber.announce_update(project_id, "database_loaded")

    def save_database(self, project_id: int, file_name: str):
        '''
        function:: save_database(project_id: int, file_name: str)
        :param project_id: int:
        :param file_name: str:
        '''
        if project_id not in self.database_for_int_dict:
            raise ValueError("no database number {id} to save".format(id=repr(project_id)))

        database = self.database_for_int_dict[project_id]
        return Exporter.export_sqlite_db_to(database.sqlite_db, database.type, file_name)

    # TODO : useful ?
    def _get_database(self, project_id: int):
        '''
        function:: database(project_id: int)
        :param project_id: int:
        '''
        if project_id not in self.database_for_int_dict:
            raise ValueError("no database number {id} to fetch".format(id=repr(project_id)))

        return self.database_for_int_dict[project_id]

    def is_database_open(self, project_id: int):
        if project_id in self.database_for_int_dict:
            return True
        else:
            return False

    def close_database(self, project_id: int):
        if project_id not in self.database_for_int_dict:
            raise ValueError("no database number {id} to fetch".format(id=repr(project_id)))

        self.database_for_int_dict[project_id].close()
        if project_id == 0:
            self.database = None

