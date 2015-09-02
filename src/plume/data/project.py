'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''

from . import subscriber, sql, cfg
import sqlite3
import os


class Project(object):

    '''
    classdocs
    '''

    def __init__(self):
        super(Project, self).__init__()
        '''
        Constructor
        '''

        self.db = None
        self._is_open = False
        self._project_path = None
        self._project_file_type = None

    def create_new_empty_database(self):
        self.database = sql.create_new_database()

    def load_test_project_db(self):
        """
        This func is only temporary. To have something to work with...
        """
        self.load('../../resources/plume_test_project.sqlite')
# new_db = sqlite3.connect(':memory:') # create a memory database
#        query = "".join(line for line in old_db.iterdump())
#
# Dump old database in the new one.
#        new_db.executescript(query)
#
#        self.db = new_db
#        cfg.data.db = new_db
#        cfg.data.main_tree.db = new_db
#
#        subscriber.announce_update("data.tree")
#        subscriber.announce_update("data.project.close")
#        subscriber.announce_update("data.project.load")
#        subscriber.announce_update("data.project.saved")
#        self._is_open = True
#
        from os.path import expanduser
        home = expanduser("~")
        self._project_path = os.path.join(home, "test_project.sqlite")
#        self._file_type = "*.sqlite"

    def load(self, file_name):
        if file_name.endswith(".sqlite"):
            old_db = sqlite3.connect(file_name)
            new_db = sqlite3.connect(':memory:')  # create a memory database
            query = "".join(line for line in old_db.iterdump())

            # Dump old database in the new one.
            new_db.executescript(query)

            self.db = new_db
            cfg.data.db = self.db
            cfg.data.main_tree.db = self.db
            cfg.database.announce_update("data.tree")
            cfg.database.announce_update("data.project.close")
            cfg.database.announce_update("data.project.load")
            cfg.database.announce_update("data.project.saved")
            self._is_open = True
            self._project_path = file_name
            self._project_file_type = "*.sqlite"

    def save_as(self, file_name,   file_type):
        if "*.sqlite" in file_type:
            if not file_name.endswith(".sqlite"):
                file_name = "".join([file_name, ".sqlite"])
            # delete if already exist:
            if os.path.exists(file_name):
                os.remove(file_name)
            on_disk_db = sqlite3.connect(file_name)
            query = "".join(line for line in self.db.iterdump())
            on_disk_db.executescript(query)
            cfg.database.announce_update("data.project.saved")
            self._project_path = file_name
            self._project_file_type = "*.sqlite"

    def save(self):
        file_name = self.project_path()
        self.save_as(file_name,  self._project_file_type)

    def is_open(self):
        return self._is_open

    def project_path(self):
        return self._project_path

    def project_file_type(self):
        return self._file_type

    def close_db(self):
        self.db = None
        cfg.data.db = None
        cfg.data.main_tree.db = None
        cfg.database.announce_update("data.project.close")
        self._is_open = False
