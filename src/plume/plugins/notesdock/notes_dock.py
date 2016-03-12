'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from plugins.synopsisdock.synopsis_dock import CoreSynopsisDock,  GuiSynopsisDock
from gui import plugins as gui_plugins


class NotesDockPlugin(gui_plugins.GuiWriteTabDockPlugin):

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

    def core_class(self):
        return CoreNotesDock

    def gui_class(self):
        return GuiNotesDock

# TODO: redo a dock displaying a list of notes
class CoreNotesDock(CoreSynopsisDock):

    '''
    CoreNotesDock, based on CoreSynopsisDock
    '''

    dock_name = "notes-dock"

    def __init__(self):
        '''
        Constructor
        '''

        super(CoreNotesDock, self).__init__()


class GuiNotesDock(GuiSynopsisDock):

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
