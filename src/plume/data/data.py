'''
Created on 19 aug. 2015

@author: Cyril Jacquet
'''

from . import subscriber, cfg


class Data(object):
    def __init__(self):
        super(Data, self).__init__()

        self.subscriber = subscriber

        cfg.data = self
        self.db = None
        self.sec_db = None

    def open_project(self, path,  project_type, index=0):
        if project_type is "sqlite3":

            #create db in temp folder

            #load db
            self.load_temp_db(db_path)
