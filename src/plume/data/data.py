'''
Created on 19 aug. 2015

@author: Cyril Jacquet
'''

from . import subscriber, cfg
from .database import Database


class Data(object):
    def __init__(self):
        super(Data, self).__init__()

        self.subscriber = subscriber

        cfg.data = self
        self.database = None

    def open_project(self, path,  project_type, index=0):
        if project_type is "sqlite3":

            #create db in temp folder
            db = Database()
            self.database = db
            #load database
            db.project.load(path)



    def open_test_project(self):
        db = Database()
        self.database = db
        db.project.load_test_project_db()
