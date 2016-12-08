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

    def __init__(self, parent, project_id):
        super(NoteTreeModel, self).__init__("l_note_id", "note", parent, project_id)

        '''
        Constructor
        '''

        self.headers = ["name"]
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.title_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.deleted_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.structure_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.properties")

    def data(self, index, role):

        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if (role == Qt.DisplayRole or role == Qt.EditRole) and col == 0:
            return self.data(index, self.TitleRole)

        if role == self.SheetCodeRole and col == 0:
            return node.data["l_sheet_code"]

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
        #
        # self._all_data = cfg.data.get_database(0).get_tree(self._table_name).get_all()
        # all_headers = cfg.data.get_database(0).get_tree(self._table_name).get_all_headers()
        # header_dict = {}
        # for header in all_headers:
        #     header_dict[header] = QVariant()

        del self._root_node
        self._root_node = TreeItem()
        self._root_node.sheet_id = -1
        self._root_node.indent = -1
        self._root_node.sort_order = -1

        note_hub = cfg.data.noteHub()
        self._id_list = note_hub.getAllTitles()
        self._title_dict = note_hub.getAllTitles()
        self._indent_dict = note_hub.getAllIndents()
        self._sort_order_dict = note_hub.getAllSortOrders()
        self._sheet_code_dict = note_hub.getAllSheetCodes()

        self._populate_item(self._root_node)

        #
        # if self._all_data is []: # empty /close
        #     self.endResetModel()
        #     return

        self._node_list = []
        self._populate_item(self._root_node)

        self.endResetModel()

    def _populate_item(self, parent_item):
        while self._id_list:
            indent = self._indent_dict(self._id_list[0])

            if parent_item.indent < indent:
                item = NoteTreeItem(parent_item)
                _id = self._id_list.pop(0)
                item.id = self._id_list.pop(0)
                item.indent = self._indent_dict[_id]
                item.sort_order = self._sort_order_dict[_id]
                item.title = self._title_dict[_id]
                # item.sheet_code = self._sheet_code_dict[_id]
                self._node_list.append(item)
                self._populate_item(item)
                parent_item.append_child(item)
            else:
                return


class NoteTreeItem(TreeItem):
    def __init__(self, parent=None):
        super(TreeItem, self).__init__(parent)
        self.sheet_code = 0
