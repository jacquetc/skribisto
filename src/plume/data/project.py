'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''

from . import subscriber, sql, cfg

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

            
    def create_new_empty_database(self):
        self.database = sql.create_new_database()
    
    def load_test_project_db(self):
        """
        This func is only temporary. To have something to work with...
        """
        
        
        import sqlite3

        new_db = sqlite3.connect(':memory:') # create a memory database
        old_db = sqlite3.connect('../../resources/plume_test_project.sqlite')
        query = "".join(line for line in old_db.iterdump())

        # Dump old database in the new one. 
        new_db.executescript(query)
        
        self.db = new_db
        cfg.data.db = new_db
        cfg.data.main_tree.db = new_db
        
        subscriber.announce_update("data.tree")
        
    def load(self, path):
        pass
        
    def save_as(self, path):
        pass
        
    def save(self):
        pass
        
    def is_opened(self):
        pass
    
    def close_db(self):
        pass
                   
