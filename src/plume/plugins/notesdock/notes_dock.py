'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

from gui import plugins as gui_plugins


class NotesDockPlugin(gui_plugins.GuiWriteSubWindowDockPlugin):

    '''
    NotesDockPlugin, based on SynopsisDockPlugin
    '''
    is_builtin_plugin = True
    ignore = True
    def __init__(self):
        '''
        Constructor
        '''

        super(NotesDockPlugin, self).__init__()

    def gui_class(self):
        return GuiNotesDock

# TODO: redo a dock displaying a list of notes

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QObject
from gui import cfg as gui_cfg
from gui.paper_manager import NotePaper

class GuiNotesDock(QObject):

    '''
    GuiNotesDock, based on GuiSynopsisDock
    '''
    dock_name = "notes-dock"
    dock_displayed_name = _("Notes")

    def __init__(self,  parent=None):
        '''
        Constructor
        '''
        super(GuiNotesDock, self).__init__(parent)
