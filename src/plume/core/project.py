'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

from . import subscriber, cfg
from PyQt5.Qt import QObject, pyqtSlot

class Project():
    '''
    Project
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(Project, self).__init__()
        self._project_path = None
        
    @pyqtSlot()
    def load_test_project(self):
        cfg.data.project.load_test_project_db()
        #subscriber.announce_update()        

    @pyqtSlot(str)
    def open(self, project_path):
        '''
        function:: open(project_path)
        :param project_path:
        '''

    def project_path(self):
        return self._project_path
        

    def save(self):
        '''
        function:: save()
        :param :
        '''

        pass

    def save_as(self):
        '''
        function:: save_as()
        :param :
        '''

        pass

    def close_project(self):
        '''
        function:: close_project()
        :param :
        '''

        pass

    def quit(self):
        '''
        function:: quit()
        :param :
        '''

        pass

    def import_(self, type, path):
        '''
        function:: import(type, path)
        :param type:
        :param path:
        '''

        pass

    def export(self, type, path):
        '''
        function:: export(type, path)
        :param type:
        :param path:
        '''

        pass
