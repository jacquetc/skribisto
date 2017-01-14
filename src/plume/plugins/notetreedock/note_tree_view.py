'''
Created on 8 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QTreeView, QMenu
from PyQt5.QtCore import Qt
from gui import cfg
from gui.models.note_tree_model import NoteTreeModel


class NoteTreeView(QTreeView):

    '''
    WriteTreeView
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(NoteTreeView, self).__init__(parent=None)

        self._old_index = None

        self.setEditTriggers(QTreeView.NoEditTriggers)
        self.setExpandsOnDoubleClick(False)
        self.setDragEnabled(True)
        self.setDragDropMode(QTreeView.DragDrop)

        self.old_index = None
        self.clicked.connect(self.itemClicked)

        self._init_actions()

    def _init_actions(self):
        pass

    def itemClicked(self, index):

        if index != self._old_index:  # reset if change
            self._one_click_checkpoint = False
            self._two_clicks_checkpoint = False
        self._old_index = index
        # third click
        if self._one_click_checkpoint and self._two_clicks_checkpoint:
            self.setCurrentIndex(index)
            self.edit(index)

            # reset :
            self._one_click_checkpoint = False
            self._two_clicks_checkpoint = False
        elif self._one_click_checkpoint == True:  # second click
            project_id = index.data(NoteTreeModel.ProjectIdRole)
            note_id = index.data(NoteTreeModel.IdRole)
            cfg.window.note_panel.open_note(project_id, note_id)

            self._two_clicks_checkpoint = True

        else:  # first click
            self.setCurrentIndex(index)
            self._one_click_checkpoint = True

    def contextMenuEvent(self, event):

        menu = QMenu(self)
        add_child_after_action = menu.addAction(_("Add sheet after"))
        add_child_after_action.triggered.connect(self.add_sheet_after)
        add_child_sheet_action = menu.addAction(_("Add child sheet"))
        add_child_sheet_action.triggered.connect(self.add_child_sheet)
        remove_sheet_action = menu.addAction(_("Remove sheet"))
        remove_sheet_action.triggered.connect(self.remove_sheet)
        menu.exec_(self.mapToGlobal(event.pos()))

        return QTreeView.contextMenuEvent(self, event)

    def add_sheet_after(self):
        parent_index = self.currentIndex()
        self.model().insert_node_after(parent_index)
        # len(self.model().node_from_index(parent_index)),
        id_ = self.model().id_of_last_created_node
        index = self.model().find_index_from_id(id_)
        # temp :# TODO to correct
        self.expandAll()
        self.setCurrentIndex(index)
        self.edit(index)

    def add_child_sheet(self):
        parent_index = self.currentIndex()
        self.model().insert_child_node(parent_index)
        id_ = self.model().id_of_last_created_node
        index = self.model().find_index_from_id(id_)
        # temp : # TODO to correct
        self.expandAll()
        self.setCurrentIndex(index)
        self.edit(index)

    def remove_sheet(self):
        index = self.currentIndex()
        self.model().remove_node(index)
