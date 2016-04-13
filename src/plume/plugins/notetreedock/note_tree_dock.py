'''
Created on 26 mai 2015

@author:  Cyril Jacquet
'''
from gui import plugins as gui_plugins
from PyQt5.Qt import pyqtSlot


class NoteTreeDockPlugin(gui_plugins.GuiNotePanelDockPlugin):
    '''
    NoteTreeDockPlugin
    '''
    is_builtin_plugin = True
    ignore = False

    def __init__(self):
        '''
        Constructor
        '''

        super(NoteTreeDockPlugin, self).__init__()

    def gui_class(self):
        return GuiNoteTreeDock
    
#    @pyqtSlot()
#    def add_property_row(self, index):
#        self._property_table_model.insertRow(1, index)
#    
#    @pyqtSlot()
#    def remove_property_row(self, index):
#        self._property_table_model.removeRow(index.row(), self._property_table_model.root_model_index())

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QSortFilterProxyModel
from gui import cfg as gui_cfg
from plugins.notetreedock import note_tree_dock_ui, note_tree_view, note_tree_proxy_model


class GuiNoteTreeDock:
    '''
    GuiNoteTreeDock
    '''
    dock_name = "note-tree-dock"
    dock_displayed_name = _("Notes")

    def __init__(self):
        '''
        Constructor
        '''
        super(GuiNoteTreeDock, self).__init__()
        self.widget = None
        self.filter = None
        self._note_tree_model = None

    @property
    def note_tree_model(self):
        if self._note_tree_model is None:
            # self._Note_tree_model = gui_cfg.window.window.Note_sub_window.tree_model
            self._note_tree_model = gui_cfg.models["0_note_tree_model"]

        return self._note_tree_model

    def get_widget(self):
        
        if self.widget is None:
            self.widget = QWidget()
            self.ui = note_tree_dock_ui.Ui_NoteTreeDock()
            self.ui.setupUi(self.widget)
            
            self.ui.treeView.hide()
            tree_view = note_tree_view.NoteTreeView()
            self.ui.mainVerticalLayout.addWidget(tree_view)
#                self.ui.treeView = Note_tree_view.NoteTreeView()

            #filter :
            self.filter = note_tree_proxy_model.NoteTreeProxyModel(self.widget)
            self.filter.setFilterKeyColumn(-1)
            self.filter.setFilterCaseSensitivity(False)
            self.filter.setSourceModel(self.note_tree_model)

            #model :
            tree_view.setModel(self.filter)

            #connect :
            #self.ui.addPropButton.clicked.connect(self.add_property_row)
            #self.ui.removePropButton.clicked.connect(self.remove_property_row)
            self.ui.filterLineEdit.textChanged.connect(self.filter.setFilterFixedString)
            #TODO: #self.ui.treeView.clicked.connect(self.set_current_row)
                
            self.widget.gui_part = self
        return self.widget
    
#    @pyqtSlot()
#    def add_property_row(self):
#        index = self.ui.tableView.currentIndex()
#        self.core_part.add_property_row(index)
#    
#    @pyqtSlot()
#    def remove_property_row(self):
#        index = self.ui.tableView.currentIndex()
#        self.core_part.remove_property_row(index)
#    
#    @pyqtSlot('QModelIndex')        
#    def set_current_row(self, model_index):
#        self.ui.tableView.setCurrentIndex(model_index)
#       
