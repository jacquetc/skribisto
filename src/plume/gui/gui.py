'''
Created on 3 mars 2015

@author: cyril
'''
from PyQt5.Qt import QObject

class Gui(QObject):


    def __init__(self, data, core):
  
        super(Gui, self).__init__()
        self.core = core
        self.data = data


    def init_gui(self):
  
        from gui.main_window import MainWindow
        from .sub_window import WriteSubWindow

        self.window = MainWindow(self, self.data, self.core)
        self.window.show()
        
        

        