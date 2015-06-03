'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from core import plugins as core_plugins
from gui import plugins as gui_plugins
from core import cfg as core_cfg
from PyQt5.Qt import pyqtSlot
from PyQt5.QtCore import Qt

class SynopsisDockPlugin(core_plugins.CoreWriteTabDockPlugin, gui_plugins.GuiWriteTabDockPlugin):
    '''
    SynopsisDockPlugin
    Be careful, this plugin is the basis for the NotesDockPlugin in plugin/notesdock
    '''
    is_builtin_plugin = True

    def __init__(self):
        '''
        Constructor
        '''

        super(SynopsisDockPlugin, self).__init__()

        
    def core_class(self):        
        return CoreSynopsisDock
    
    def gui_class(self):
        return GuiSynopsisDock
    
class CoreSynopsisDock():
    '''
    CoreSynopsisDock
    '''

    dock_name = "synopsis-dock" 
    note_type_name = "synopsis"
    
    def __init__(self):
        '''
        Constructor
        '''

        super(CoreSynopsisDock, self).__init__()
        self._synopsis_rich_text = None
        self._sheet_id = None
        self.tree_sheet = None        

    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            return
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(self.sheet_id)
            _ = self.synopsis_rich_text
            
            
        
    @property   
    def synopsis_rich_text(self):
        if self._synopsis_rich_text is None:
            self._synopsis_rich_text = ""
            if self._sheet_id is not None:
                other_contents_dict = self.tree_sheet.get_other_contents()
                try :
                    self._synopsis_rich_text = other_contents_dict[self.note_type_name]
                except KeyError:
                    self._synopsis_rich_text = ""
            
        return self._synopsis_rich_text
        
    @synopsis_rich_text.setter
    def synopsis_rich_text(self,  text):
        if self.sheet_id is not None:
            self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(self.sheet_id)
            self.tree_sheet.set_other_content(self.note_type_name,  text) 
            self._synopsis_rich_text = text
    

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QObject
from gui import cfg as gui_cfg
from plugins.synopsisdock import synopsis_dock_ui

class GuiSynopsisDock(QObject):
    '''
    GuiSynopsisDock
    '''
    dock_name = "synopsis-dock" 
    dock_displayed_name = _("Synopsis")
    def __init__(self,  parent = None):
        '''
        Constructor
        '''
        super(GuiSynopsisDock, self).__init__(parent)
        self.widget = None
        self.core_part = None     #      CoreSynopsisDock
        self._sheet_id = None
        self.tree_sheet = None        

    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            pass
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self.tree_sheet = gui_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(self.sheet_id)
            self.core_part = self.tree_sheet.get_instance_of(self.dock_name)
            self.core_part.sheet_id = sheet_id
            core_cfg.data.subscriber.unsubscribe_update_func(self.get_update)
            core_cfg.data.subscriber.subscribe_update_func_to_domain(self.get_update,"data.tree.other_contents", self._sheet_id)

    def get_widget(self):
        
        if self.widget is None:
            self.widget = QWidget()
            self.ui = synopsis_dock_ui.Ui_SynopsisDock()
            self.ui.setupUi(self.widget)
            
            self.ui.writingZone.has_minimap = False
            self.ui.writingZone.has_scrollbar = False
            self.ui.writingZone.is_resizable = False
            self.ui.has_side_tool_bar = False        

            if self.tree_sheet is not None and self.core_part is not None:
                self.get_update()

                
                #connect :
                self.ui.writingZone.text_edit.textChanged.connect(self.apply_text_change, type=Qt.UniqueConnection )
                
            self.widget.gui_part = self
        return self.widget
 
    def get_update(self):
        self.ui.writingZone.text_edit.blockSignals(True)
        if self.tree_sheet is not None and self.core_part is not None:
            text = self.core_part.synopsis_rich_text
            self.ui.writingZone.set_rich_text(text)
        self.ui.writingZone.text_edit.blockSignals(False) 
        
    @pyqtSlot()
    def apply_text_change(self):
        core_cfg.data.subscriber.disable_func(self.get_update)
        self.core_part.synopsis_rich_text = self.ui.writingZone.text_edit.toHtml()
        core_cfg.data.subscriber.enable_func(self.get_update)

