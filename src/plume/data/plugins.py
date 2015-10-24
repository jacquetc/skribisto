'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from .base import DatabaseBaseClass


class Plugins(DatabaseBaseClass):

    '''
    classdocs
    '''

    def __init__(self, database_subsriber):
        super(Plugins, self).__init__(database_subsriber)
        '''
        Constructor
        '''
