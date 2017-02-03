'''
Created on 8 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QTreeView, QMenu
from PyQt5.QtCore import Qt, QTimer, QModelIndex, QCoreApplication, QVariant
from gui import cfg
from gui.models.sheet_tree_model import SheetTreeModel
from gui.property import SheetProperty
from gui.paper_manager import SheetPaper
from gui.models.tree_model import AddChildNodeCommand, AddAfterNodeCommand\
    , DeleteCommand




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
        self.apply_expand()

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
        project_id = model_index.data(SheetTreeModel.ProjectIdRole)
        cfg.data.sheetPropertyHub().setProperty(project_id, paper_id, "write_tree_item_expanded", "1")
        # TODO icfg.models["0_sheet_system_property_model"].set_property(paper_id, "write_tree_item_expanded", "1")

    def set_item_collapsed(self, model_index):
        self.setExpanded(model_index, False)
        paper_id = model_index.data(SheetTreeModel.IdRole)
        project_id = model_index.data(SheetTreeModel.ProjectIdRole)
        cfg.data.sheetPropertyHub().setProperty(project_id, paper_id, "write_tree_item_expanded", "0")
        # TODO icfg.models["0_sheet_system_property_model"].set_property(paper_id, "write_tree_item_expanded", "0")

    def apply_expand(self):
        index = self.indexAt(self.rect().topLeft())
        while index.isValid():
            paper_id = index.data(SheetTreeModel.IdRole)
            project_id = index.data(SheetTreeModel.ProjectIdRole)
            # if True:
            # TODO if cfg.models["0_sheet_system_property_model"].get_property(paper_id, "write_tree_item_expanded", "1") == "1":
            if cfg.data.sheetPropertyHub().getProperty(project_id, paper_id, "write_tree_item_expanded") == "1":
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
            project_id = index.data(SheetTreeModel.ProjectIdRole)
            sheet_id = index.data(SheetTreeModel.IdRole)
            cfg.window.write_panel.open_sheet(project_id, sheet_id)

            self._two_clicks_checkpoint = True

        else:  # first click
            self.setCurrentIndex(index)
            self._one_click_checkpoint = True

    def contextMenuEvent(self, event):

        self.setCurrentIndex(self.indexAt(event.pos()))

        menu = QMenu(self)
        add_child_after_action = menu.addAction(_("Add sheet after"))
        add_child_after_action.triggered.connect(self.add_sheet_after)
        add_child_sheet_action = menu.addAction(_("Add child sheet"))
        add_child_sheet_action.triggered.connect(self.add_child_sheet)
        remove_sheet_action = menu.addAction(_("Remove sheet"))
        remove_sheet_action.triggered.connect(self.delete_sheet)
        menu.exec_(self.mapToGlobal(event.pos()))

        return QTreeView.contextMenuEvent(self, event)

    def add_sheet_after(self):
        parent_index = self.currentIndex()
        sheet_id = parent_index.data(SheetTreeModel.IdRole)
        project_id = parent_index.data(SheetTreeModel.ProjectIdRole)
        model = cfg.models["0_sheet_tree_model"]

        command = AddAfterNodeCommand(project_id, sheet_id, model)
        model.undo_stack.push(command)
        new_sheet_id = command.last_new_id
        model.set_undo_stack_active()

        # select index

        self.select_by_id(new_sheet_id)


    def add_child_sheet(self):

        parent_index = self.currentIndex()
        sheet_id = parent_index.data(SheetTreeModel.IdRole)
        project_id = parent_index.data(SheetTreeModel.ProjectIdRole)
        model = cfg.models["0_sheet_tree_model"]

        command = AddChildNodeCommand(project_id, sheet_id, model)
        model.undo_stack.push(command)
        new_sheet_id = command.last_new_child_id
        model.set_undo_stack_active()

        self.select_by_id(new_sheet_id)


        #new_sheet_id = SheetPaper(sheet_id).add_sheet_after()
        # index = model.find_index_from_id(new_sheet_id)
        #
        # self.edit(index)
        # index_list = model.match(self.model().mapToSource(self.rootIndex()), SheetTreeModel.IdRole, new_sheet_id
        #                            , -1, (Qt.MatchExactly | Qt.MatchRecursive | Qt.MatchWrap))
        # print(len(index_list))
        # if index_list != []:
        #     index = index_list[0]
        #     if index.isValid():
        #         self.setCurrentIndex(index)
        #         self.edit(index)

    def delete_sheet(self):
        parent_index = self.currentIndex()
        project_id = parent_index.data(SheetTreeModel.ProjectIdRole)
        sheet_id = parent_index.data(SheetTreeModel.IdRole)
        model = cfg.models["0_sheet_tree_model"]

        command = DeleteCommand(project_id, sheet_id, True, model)
        model.undo_stack.push(command)
        model.set_undo_stack_active()

    def select_by_id(self, paper_id: int):
        index_list = self.model().match(self.model().index(0, 0), SheetTreeModel.IdRole, paper_id
                                        , 1, (Qt.MatchExactly | Qt.MatchRecursive))
        if index_list:
            index = index_list[0]
            if index.isValid():
                self.setCurrentIndex(index)