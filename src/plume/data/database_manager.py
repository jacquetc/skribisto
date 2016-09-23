from PyQt5.QtCore import QObject, pyqtSignal, QThread, Qt
from .database import Database
from .exporter import Exporter
from . import cfg

class DatabaseManager(QObject):
    # project_id, file_name
    init_sent = pyqtSignal(int, str, name='init_sent')
    close_sent = pyqtSignal(int, name='close_sent')

    def __init__(self, parent, worker_thread:QThread):
        super(DatabaseManager, self).__init__(parent)
        self._worker_thread = worker_thread

        self.database_for_int_dict = {}

        self._increment = -1

    def create_new_empty_database(self, project_id: int):
        '''
        function:: create_new_empty_database(project_id: int)
        :param project_id: int:
        '''
        self.load_database(project_id, "")

    def load_database(self, file_name: str):
        '''
        function:: load_database(project_id: int, file_name: str)
        :param project_id: int:
        :param file_name: str:
        '''
        self._increment += 1
        project_id = self._increment

        db = Database()
        self.init_sent.connect(db.init)
        self.close_sent.connect(db.close)

        db.moveToThread(self._worker_thread)
        self.init_sent.emit(project_id, file_name)

        self.database_for_int_dict[project_id] = db
        # cfg.data.subscriber.announce_update(project_id, "database_loaded")
        return project_id

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
    def get_database(self, project_id: int):
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

        self.close_sent.emit(project_id)
        if project_id == 0:
            self.database = None

