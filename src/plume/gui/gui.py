'''
Created on 3 mars 2015

@author: cyril
'''
from PyQt5.Qt import QObject
from . import cfg

class Gui(QObject):


    def __init__(self, core):
  
        super(Gui, self).__init__()
        cfg.core = core


    def init_gui(self):
  
        from .main_window import MainWindow

        self.window = MainWindow(self)
        self.window.show()
        
        

        