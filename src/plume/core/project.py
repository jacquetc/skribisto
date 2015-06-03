'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

from . import subscriber, cfg
from PyQt5.Qt import pyqtSlot

class Project():
    '''
    Project
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(Project, self).__init__()
        cfg.data.subscriber.subscribe_update_func_to_domain(self.signal_project_is_saved, "data.project.saved")
        cfg.data.subscriber.subscribe_update_func_to_domain(self.signal_project_is_not_saved, "data.project.notsaved")

        
        
    @pyqtSlot()
    def open_test_project(self):
        cfg.data.project.load_test_project_db()
        subscriber.announce_update("core.project.load")
       #subscriber.announce_update()        

    @pyqtSlot(str)
    def open(self, file_name):
        '''
        function:: open(file_name)
        :param file_name:
        '''
        cfg.data.project.load(file_name)
        subscriber.announce_update("core.project.load")
        
    def project_path(self):
        return self._project_path
        

    def save(self):
        '''
        function:: save()
        :param :
        '''

        cfg.data.project.save()

    def save_as(self, file_name, file_type):
        '''
        function:: save_as(file_name,   file_type)
        :param file_name:
        :param file_type:
        :param :
        '''
        cfg.data.project.save_as(file_name, file_type)


    def close_project(self):
        '''
        function:: close_project()
        :param :
        '''
        self._clear_project()

        
    def _clear_project(self):
        '''
        function:: _clear_project()
        :param :
        Clear all project from core
        '''
        cfg.data.project.close_db()
        subscriber.announce_update("core.project.close")
        
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
        
    def is_open(self):
        return cfg.data.project.is_open()
        
        
    def signal_project_is_saved(self):
        subscriber.announce_update("core.project.saved")
       
    def signal_project_is_not_saved(self):
        subscriber.announce_update("core.project.notsaved")       
