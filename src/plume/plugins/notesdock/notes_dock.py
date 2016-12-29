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

from PyQt5.QtWidgets import QWidget, QVBoxLayout
from PyQt5.QtCore import QObject, QUrl
from PyQt5.QtQuickWidgets import QQuickWidget
from gui import cfg as gui_cfg
from plugins.notesdock import note_list_proxy_model
import os
import inspect

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
        self.widget = None
        self.filter = None
        self._note_list_model = None
        self._sheet_id = None

        # filter :
        self.filter = note_list_proxy_model.NoteListProxyModel(self)
        self.filter.setSourceModel(self.note_list_model)

    @property
    def note_list_model(self):
        if self._note_list_model is None:
            # self._write_tree_model = gui_cfg.window.window.write_sub_window.tree_model
            self._note_list_model = gui_cfg.models["0_note_list_model"]

        return self._note_list_model

    @property
    def paper_id(self):
        """
        Only used for compatibility with API, return sheet_id
        :return:
        """
        return self.sheet_id

    @paper_id.setter
    def paper_id(self, paper_id):
        """
        Only used for compatibility with API, call sheet_id
        :param id:
        """
        self.sheet_id = paper_id

    @property
    def sheet_id(self):
        return self._sheet_id

    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            pass
        self._sheet_id = sheet_id
        self.get_update()
        if self.sheet_id is not None:
            gui_cfg.data.subscriber.unsubscribe_update_func(self.get_update)
            gui_cfg.data.subscriber.subscribe_update_func_to_domain(0, self.get_update,
                                                                           "note.content_changed", self._sheet_id)
    def get_update(self):
        if self.sheet_id is not None and self.filter is not None:
            self.filter.filterNoteBySheet(self.sheet_id)


    def get_widget(self):
        if self.widget is None:
            self.widget = QWidget()
            # self.ui = note_list_dock_ui.Ui_WriteTreeDock()
            # self.ui.setupUi(self.widget)
            #
            # self.ui.treeView.hide()
            # tree_view = note_tree_view.WriteTreeView()
            # self.ui.mainVerticalLayout.addWidget(tree_view)



            #model :
            #tree_view.setModel(self.filter)

            #connect :
            #self.ui.addPropButton.clicked.connect(self.add_property_row)
            #self.ui.removePropButton.clicked.connect(self.remove_property_row)
            #self.ui.filterLineEdit.textChanged.connect(self.filter.setFilterFixedString)

            #self.widget.gui_part = self
            self.widget.setLayout(QVBoxLayout())
            mQQuickWidget = QQuickWidget(self.widget)
            mQQuickWidget.setResizeMode(QQuickWidget.SizeRootObjectToView)
            mQQuickWidget.rootContext().setContextProperty("myModel", self.filter)
            abspath = os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe())))
            mQQuickWidget.setSource(QUrl(abspath + "/qml/noteListView.qml"))
            self.widget.layout().addWidget(mQQuickWidget)

        # connect(mQQuickWidget->rootObject(), SIGNAL(openNoteSignal(int)), this, SLOT(test(int)));
        # connect(mQQuickWidget->rootObject(), SIGNAL(displayContextMenuSignal(int,int,int)), this, SLOT(displayContextMenu(int,int,int)));


        self.filter.filterNoteBySheet(self.sheet_id)
        return self.widget
