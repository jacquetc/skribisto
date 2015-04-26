'''
Created on 3 mars 2015

@author: Cyril Jacquet
'''
from PyQt5.Qt import QObject

class Core(QObject):
    '''
    classdocs
    '''


    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(Core, self).__init__(parent)