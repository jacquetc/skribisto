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
        self.sqlite_db = None
        # init this Database subscriber
        self.subscriber = subscriber.DatabaseSubscriber(self._database_id)

        # init all :
        self.project = Project(self.subscriber)
        self.sheet_tree = SheetTree(self.subscriber)
        self.sheet_tree = NoteTree(self.subscriber)
        self.plugins = Plugins(self.subscriber)
