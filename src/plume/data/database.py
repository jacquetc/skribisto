from .plugins import Plugins
from .tree import Tree
from .project import Project
from . import cfg, subscriber


class Database(object):

    def __init__(self, is_main_database=False):
        super(Database, self).__init__(is_main_database)
        self.is_main_db = is_main_database
        if is_main_database is True:
            cfg.database = self


        # init all :
        self.project = Project(self)
        self.main_tree = Tree(self)
        self.plugins = Plugins(self)


    def announce_update(self, domain, sheet_id=-1):
        '''
        function:: announce_update(domain)
        :param domain:
        :param sheet_id: int. optional. if present, can narrow_down the update.
        '''
        if self.is_main_database is True:
            subscriber.announce_update(domain, sheet_id)