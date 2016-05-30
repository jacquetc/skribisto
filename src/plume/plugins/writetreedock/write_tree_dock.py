'''
Created on 26 mai 2015

@author:  Cyril Jacquet
'''
from gui import plugins as gui_plugins
from PyQt5.Qt import pyqtSlot


class WriteTreeDockPlugin(gui_plugins.GuiWritePanelDockPlugin):
    '''
    WriteTreeDockPlugin
    '''
    is_builtin_plugin = True
    ignore = False

    def __init__(self):
        '''
        Constructor
        '''

        super(WriteTreeDockPlugin, self).__init__()

    def gui_class(self):
        return GuiWriteTreeDock
    
#    @pyqtSlot()
#    def add_property_row(self, index):
#        self._property_table_model.insertRow(1, index)
#    
#    @pyqtSlot()
#    def remove_property_row(self, index):
#        self._property_table_model.removeRow(index.row(), self._property_table_model.root_model_index())

from PyQt5.QtWidgets import QWidget
from gui import cfg as gui_cfg
from plugins.writetreedock import write_tree_dock_ui, write_tree_view, write_tree_proxy_model


class GuiWriteTreeDock:
    '''
    GuiWriteTreeDock
    '''
    dock_name = "write-tree-dock" 
    dock_displayed_name = _("Project")

    def __init__(self):
        '''
        Constructor
        '''
        super(GuiWriteTreeDock, self).__init__()
        self.widget = None
        self.filter = None
        self._write_tree_model = None

    @property
    def write_tree_model(self):
        if self._write_tree_model is None:
            # self._write_tree_model = gui_cfg.window.window.write_sub_window.tree_model
            self._write_tree_model = gui_cfg.models["0_sheet_tree_model"]

        return self._write_tree_model

    def get_widget(self):
        
        if self.widget is None:
            self.widget = QWidget()
            self.ui = write_tree_dock_ui.Ui_WriteTreeDock()
            self.ui.setupUi(self.widget)
            
            self.ui.treeView.hide()
            tree_view = write_tree_view.WriteTreeView()
            self.ui.mainVerticalLayout.addWidget(tree_view)
#                self.ui.treeView = write_tree_view.WriteTreeView()

            #filter :
            self.filter = write_tree_proxy_model.WriteTreeProxyModel(self.widget)
            self.filter.setFilterKeyColumn(0)
            self.filter.setFilterCaseSensitivity(False)
            self.filter.setSourceModel(self.write_tree_model)

            #model :
            tree_view.setModel(self.filter)
            tree_view.init()
            #connect :
            self.ui.filterLineEdit.textChanged.connect(self.filter.setFilterFixedString)
            #TODO: #self.ui.treeView.clicked.connect(self.set_current_row)



                
            #self.widget.gui_part = self
        return self.widget





