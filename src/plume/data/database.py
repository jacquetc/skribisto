from .plugins import Plugins
from .sheet_tree import SheetTree
from .note_tree import NoteTree
from .project import Project
from . import cfg, subscriber


class Database(object):

    def __init__(self):
        super(Database, self).__init__()
        self._database_id = subscriber.get_new_database_id(self)
        #shortcut :
        if self._database_id is 0:
            cfg.database = self
        self._sqlite_db = None
        # init this Database subscriber
        self.subscriber = subscriber.DatabaseSubscriber(self._database_id)

        # init all :
        self.project = Project(self.subscriber)
        self.sheet_tree = SheetTree(self.subscriber)
        self.note_tree = NoteTree(self.subscriber)
        self.plugins = Plugins(self.subscriber)

    @property
    def sqlite_db(self):
        return self._sqlite_db

    @sqlite_db.setter
    def sqlite_db(self, db):
        self._sqlite_db = db
        self.sheet_tree.a_db = db
        self.note_tree.a_db = db
