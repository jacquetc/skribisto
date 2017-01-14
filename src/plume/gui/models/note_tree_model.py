'''
Created on 13 avr. 2016

@author:  Cyril Jacquet
'''
from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt
from .. import cfg
from .tree_model import TreeModel, TreeItem
from ..paper_manager import NotePaper


class NoteTreeModel(TreeModel):
    '''
    classdocs
    '''
    SheetCodeRole = Qt.UserRole + 20

    def __init__(self, parent):
        super(NoteTreeModel, self).__init__("l_note_id", "note", parent)

        '''
        Constructor
        '''
        self._sheet_code_dict = {}

        self.headers = ["name"]

        cfg.data.projectHub().projectLoaded.connect(self.reset_model)


    def data(self, index, role):

        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if (role == Qt.DisplayRole or role == Qt.EditRole) and col == 0:
            return self.data(index, self.TitleRole)

        if role == self.SheetCodeRole and col == 0:
            return node.sheet_code

        return TreeModel.data(self, index, role)




#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):

        limit = [role]

        # title :
        # if index.isValid() & role == Qt.EditRole & index.column() == 0:
        #
        #     node = self.node_from_index(index)
        #
        #     note = NotePaper(node.id)
        #     note.title = value
        #
        #     self.dataChanged.emit(index, index, limit)
        #     # cfg.data_subscriber.announce_update(self._project_id, "sheet.title_changed", node.id)
        #     return True

        return TreeModel.setData(self, index, value, role)

    def reset_model(self):
        self.beginResetModel()

        del self._root_node
        self._root_node = NoteTreeItem()
        self._root_node.sheet_id = -1
        self._root_node.indent = -1
        self._root_node.sort_order = -1

        self._node_list = []
        project_id_list = cfg.data.projectHub().getProjectIdList()
        for project_id in project_id_list:
            note_hub = cfg.data.noteHub()
            self._id_list = note_hub.getAllIds(project_id)
            self._title_dict = note_hub.getAllTitles(project_id)
            self._indent_dict = note_hub.getAllIndents(project_id)
            self._sort_order_dict = note_hub.getAllSortOrders(project_id)
            self._sheet_code_dict = note_hub.getAllSheetCodes(project_id)

            # create project name items
            if len(project_id_list) > 1:
                item = NoteTreeItem(self._root_node)
                item.indent = 0
                item.indent_drift = 1
                item.title = "project " + str(project_id)
                self._node_list.append(item)
                self._root_node.append_child(item)
                self._populate_item(item, project_id)
            else:
                self._populate_item(self._root_node, project_id)

        self.endResetModel()

    def _populate_item(self, parent_item: TreeItem, project_id: int):
        while self._id_list:
            indent = self._indent_dict[self._id_list[0]]

            if parent_item.indent < indent:
                item = NoteTreeItem(parent_item)
                _id = self._id_list.pop(0)
                item.id = _id
                item.project_id = project_id
                item.indent_drift = parent_item.indent_drift
                item.indent = self._indent_dict[_id]
                item.sort_order = self._sort_order_dict[_id]
                item.title = self._title_dict[_id]
                item.sheet_code = self._sheet_code_dict[_id]
                self._node_list.append(item)
                parent_item.append_child(item)
                self._populate_item(item, project_id)
            else:
                return


class NoteTreeItem(TreeItem):
    def __init__(self, parent=None):
        super(NoteTreeItem, self).__init__(parent)
        self.sheet_code = 0
