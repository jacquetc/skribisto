'''
Created on 26 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QDialog
from PyQt5.QtCore import pyqtSlot
from .start_window_ui import Ui_StartWindow


class StartWindow(QDialog):

    def __init__(self, parent=None):
        super(StartWindow, self).__init__(parent)
        self.ui = Ui_StartWindow()
        self.ui.setupUi(self)
