'''
Created on 24 octobre 2015

@author:  Cyril Jacquet
'''


class DatabaseBaseClass:
    def __init__(self, database_subsriber):
        super(DatabaseBaseClass, self).__init__()
        self.subscriber = database_subsriber
