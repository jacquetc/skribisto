'''
Created on 8 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QTreeView, QMenu
from PyQt5.QtCore import Qt, QTimer
from gui import cfg
from gui.models.sheet_tree_model import SheetTreeModel
from gui.property import SheetProperty


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
        self.setAutoExpandDelay(1000)
        self.setDragEnabled(True)
        self.setDragDropMode(QTreeView.DragDrop)
        self.setAnimated(True)

        self.old_index = None
        self.clicked.connect(self.itemClicked)

        self._init_actions()

    def init(self):
        self.model().modelReset.connect(self.apply_expand)


    def _init_actions(self):
        pass


    def mousePressEvent(self, event):
        clicked_index = self.indexAt(event.pos())
        if clicked_index.isValid():
            vrect = self.visualRect(clicked_index)
            item_identation = vrect.x() - self.visualRect(self.rootIndex()).x()
            if event.pos().x() < item_identation:
                if not self.isExpanded(clicked_index):
                    self.set_item_expanded(clicked_index)
                else:
                    self.set_item_collapsed(clicked_index)
            else:
                QTreeView.mousePressEvent(self, event)

    def set_item_expanded(self, model_index):
        self.setExpanded(model_index, True)
        paper_id = model_index.data(SheetTreeModel.IdRole)
        SheetProperty.set_property(paper_id, "write_tree_item_expanded", "1")

    def set_item_collapsed(self, model_index):
        self.setExpanded(model_index, False)
        paper_id = model_index.data(SheetTreeModel.IdRole)
        SheetProperty.set_property(paper_id, "write_tree_item_expanded", "0")

    def apply_expand(self):
        index = self.indexAt(self.rect().topLeft())
        while index.isValid():
            paper_id = index.data(SheetTreeModel.IdRole)
            if SheetProperty.get_property(paper_id, "write_tree_item_expanded", "1") == "1":
                self.setExpanded(index, True)
            else:
                self.setExpanded(index, False)
            index = self.indexBelow(index)

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
            sheet_id = index.data(SheetTreeModel.IdRole)
            cfg.window.write_panel.open_sheet(sheet_id)

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


        id_ = self.model().id_of_last_created_node
        index = self.model().find_index_from_id(id_)

        self.setCurrentIndex(index)
        self.edit(index)


    def add_child_sheet(self):
        parent_index = self.currentIndex()
        self.model().insert_child_node(parent_index)
        id_ = self.model().id_of_last_created_node
        index = self.model().find_index_from_id(id_)
        print(index.isValid())
        # temp : # TODO to correct
        self.setCurrentIndex(index)
        self.edit(index)

    def remove_sheet(self):
        index = self.currentIndex()
        self.model().remove_node(index)

