'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from core import plugins as core_plugins
from gui import plugins as gui_plugins


class PropertiesDockPlugin(core_plugins.CoreStoryDockPlugin, gui_plugins.GuiStoryDockPlugin):
    '''
    PropertiesDockPlugin
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(PropertiesDockPlugin, self).__init__()


    def print_name(self):
        print("This is plugin 1")
        
    def core_class(self):        
        return CorePropertyDock
    
    def gui_class(self):
        return GuiPropertyDock
    
class CorePropertyDock():
    '''
    CorePropertyDock
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(CorePropertyDock, self).__init__()

from PyQt5.QtWidgets import QLabel

class GuiPropertyDock():
    '''
    GuiPropertyDock
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(GuiPropertyDock, self).__init__()

    def get_widget(self):
        return QLabel("TEEEEEEEEEEST")
