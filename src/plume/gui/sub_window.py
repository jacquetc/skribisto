'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import QMainWindow
from . import cfg


class SubWindow(QMainWindow):

    '''
    Abstract class. Do not use it directly.
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor
        '''
        super(SubWindow, self).__init__(parent=parent)
        self.parent_window_system_controller = parent_window_system_controller
        self._is_attached = True

    def is_attached(self):
        return self._is_attached

    def detach_this_window(self):
        self.parent_window_system_controller.detach_sub_window(self)
        self._is_attached = False

    def attach_back_to_parent_window_system(self):
        self.parent_window_system_controller.attach_sub_window(self)
        self._is_attached = True

    def closeEvent(self,  event):
        if self.parent_window_system_controller is not None:
            self.attach_back_to_parent_window_system()
            event.ignore()
        else:
            event.accept()


