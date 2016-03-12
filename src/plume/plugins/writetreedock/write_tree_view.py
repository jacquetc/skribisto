'''
Created on 8 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QTreeView, QMenu
from PyQt5.QtCore import Qt
from gui import cfg


class WriteTreeView(QTreeView):

    '''
    WriteTreeView
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(WriteTreeView, self).__init__(parent=None)

        self._old_index = None

        self.setEditTriggers(QTreeView.NoEditTriggers)
        self.setExpandsOnDoubleClick(False)

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
            sheet_id = index.data(37)
            # TODO cfg.core.tree_sheet_manager.open_sheet(sheet_id)

            self._two_clicks_checkpoint = True

        else:  # first click
            self.setCurrentIndex(index)
            self._one_click_checkpoint = True

    def contextMenuEvent(self, event):

        menu = QMenu(self)
        attachAction = menu.addAction(_("Add sheet after"))
        attachAction.triggered.connect(self.add_sheet_after)
        addChildSheetAction = menu.addAction(_("Add child sheet"))
        addChildSheetAction.triggered.connect(self.add_child_sheet)
        menu.exec_(self.mapToGlobal(event.pos()))

        return QTreeView.contextMenuEvent(self, event)

    def add_sheet_after(self):
        parent_index = self.currentIndex()
        self.model().insert_row_after(parent_index)
        # len(self.model().node_from_index(parent_index)),
        id_ = self.model().id_of_last_created_sheet
        index = self.model().find_index_from_id(id_)
        # temp :# TODO to correct
        self.expandAll()
        self.setCurrentIndex(index)
        self.edit(index)

    def add_child_sheet(self):
        parent_index = self.currentIndex()
        self.model().insert_child_row(parent_index)
        id_ = self.model().id_of_last_created_sheet
        index = self.model().find_index_from_id(id_)
        print(index.isValid())
        # temp : # TODO to correct
        self.expandAll()
        self.setCurrentIndex(index)
        self.edit(index)