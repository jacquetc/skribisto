# -*- coding: utf-8 -*-
'''
Created on 27 may 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import QDialog
from .preferences_ui import Ui_Preferences

class Preferences(QDialog):

    def __init__(self, parent):
        super(Preferences, self).__init__(parent = None)
        self.init_ui()


    def init_ui(self):
        self.ui = Ui_Preferences()
        self.ui.setupUi(self)
        
        
        #self.ui.themeButton
        
