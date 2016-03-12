'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt, QObject
from .. import cfg

class TreeModel(QAbstractItemModel):
    '''
    Tree
    '''

    def __init__(self, table_name: str, id_name: str, parent: QObject, project_id: int):
        super(TreeModel, self).__init__(parent)

        self.IdRole = Qt.UserRole
        self.TitleRole = Qt.UserRole + 1
        self.ContentRole = Qt.UserRole + 2
        self.SortOrderRole = Qt.UserRole + 3
        self.IndentRole = Qt.UserRole + 4
        self.DateCreatedRole = Qt.UserRole + 5
        self.DateUpdatedRole = Qt.UserRole + 6
        self.DateContentRole = Qt.UserRole + 7
        self.DeletedRole = Qt.UserRole + 8
        self.VersionRole = Qt.UserRole + 9
        self.CharCountRole = Qt.UserRole + 10
        self.WordCountRole = Qt.UserRole + 11
        # inheriting classes will start at Qt.UserRole + 20

        self._project_id = project_id
        self._root_node = TreeItem()
        self._table_name = table_name
        self._id_name = id_name
        self._all_data = []
        self._node_list = []

    @property
    def tree_db(self):
        return cfg.data.database(self._project_id).get_tree(self._table_name);

    def columnCount(self, parent=None):
        return 1

    def rowCount(self, parent=None):

        if parent.column() > 0:
            return 0

        if not parent.isValid():
            parent_node = self._root_node

            return len(parent_node)
        else:
            parent_node = self.node_from_index(parent)
            return len(parent_node)

    def headerData(self, section, orientation, role):
        if orientation == Qt.Horizontal and role == Qt.DisplayRole:
            return QVariant(self.headers[section])
        return QVariant()

    def index(self, row, column, parent):
        if not self.hasIndex(row, column, parent):
            return QModelIndex()

        node = self.node_from_index(parent)
        return self.createIndex(row, column, node.child_at_row(row))

    def data(self, index, role):

        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if role == self.TitleRole and col == 0:
            return node.title
        if role == self.SortOrderRole and col == 0:
            return node.data["l_sort_order"]
        if role == self.IndentRole and col == 0:
            return node.data["l_indent"]
        if role == self.DateCreatedRole and col == 0:
            return node.data["dt_created"]
        if role == self.ContentRole and col == 0:
            return node.data["m_content"]
        if role == self.DateUpdatedRole and col == 0:
            return node.data["dt_updated"]
        if role == self.DateContentRole and col == 0:
            return node.data["dt_content"]
        if role == self.DeletedRole and col == 0:
            return node.data["b_deleted"]
        if role == self.VersionRole and col == 0:
            return node.data["l_version"]

        return QVariant()

    def node_from_index(self, index):
        return index.internalPointer() if index.isValid() else self._root_node



    def parent(self, child):
        if not child.isValid():
            return QModelIndex()

        node = self.node_from_index(child)

        if node is None:
            return QModelIndex()

        parent = node.parent
        if parent == self._root_node:
            return QModelIndex()

        if parent is None:
            return QModelIndex()

        # needed to know if
        grandparent = parent.parent
        if grandparent is None:
            return QModelIndex()
        row = grandparent.row_of_child(parent)

        assert row != - 1
        index = self.createIndex(row, 0, parent)
        parent.index = index
        return index


#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):

        limit = [role]

        # # title :
        # if index.isValid() & role == self.TitleRole & index.column() == 0:
        #
        #     node = self.node_from_index(index)
        #
        #     self.tree_db.set_title(node.id, value)
        #     node.title = value
        #
        #     self.dataChanged.emit(index, index, limit)
        #     return True

        return False

    def supportedDropActions(self):
        return Qt.CopyAction | Qt.MoveAction

    def flags(self, index):
        default_flags = QAbstractItemModel.flags(self, index)

        if index.isValid():
            return Qt.ItemIsEditable | Qt.ItemIsDragEnabled | \
                    Qt.ItemIsDropEnabled | default_flags

        else:
            return Qt.ItemIsDropEnabled | default_flags

    def reset_model(self):
        self.beginResetModel()

        self._all_data = cfg.data.get_database(0).get_tree(self._table_name).get_all()
        all_headers = cfg.data.get_database(0).get_tree(self._table_name).get_all_headers()
        header_dict = {}
        for header in all_headers:
            header_dict[header] = QVariant()

        del self._root_node
        self._root_node = TreeItem()
        self._root_node.sheet_id = -1
        self._root_node.indent = -1
        self._root_node.sort_order = -1

        self._root_node.data = header_dict

        if self._all_data is []: # empty /close
            self.endResetModel()
            return

        self._node_list = []
        self._populate_item(self._root_node)

        self.endResetModel()

    def _populate_item(self, parent_item):
        while self._all_data:
            indent = self._all_data[0]["l_indent"]

            if parent_item.indent < indent:
                data_dict = self._all_data.pop(0)
                item = TreeItem(parent_item)
                item.data = data_dict
                item.id = data_dict[self._id_name]
                self._populate_item(item)
                self._node_list.append(item)
                parent_item.append_child(item)
            else:
                return

    def clear(self):
        self.beginResetModel()
        self._root_node = TreeItem()
        self.endResetModel()


    @property
    def item_list(self):
        return self._item_list


class TreeItem(object):

    def __init__(self, parent=None):
        super(TreeItem, self).__init__()

        self.data = {}
        self.index = QModelIndex()
        self.id = -1
        self.parent_id = None
        self.children_id = None
        self.properties = None

        self._parent = None

        self.parent = parent
        self.children = []

    @property
    def parent(self):
        return self._parent

    @parent.setter
    def parent(self, parent: QModelIndex()):
        if parent is not None:
            self._parent = parent
        else:
            self._parent = None

    def append_child(self, child):
        if child not in self.children:
            self.children.append(child)

    def child_at_row(self, row):
        return self.children[row]

    def row_of_child(self, child):
        for i, item in enumerate(self.children):
            if item == child:
                return i
        return -1

    def remove_child(self, row):
        value = self.children[row]
        self.children.remove(value)

        return True

    def row(self):
        if self._parent:
            return int(self._parent.children.index(self))
        return -1

    def __len__(self):
        return len(self.children)

    @property
    def indent(self):
        return self.data["l_indent"]

    @indent.setter
    def indent(self, value: int):
        self.data["l_indent"] = value

    @property
    def sort_order(self):
        return self.data["l_sort_order"]

    @sort_order.setter
    def sort_order(self, value: int):
        self.data["l_sort_order"] = value

    @property
    def title(self):
        return self.data["t_title"]

    @title.setter
    def title(self, value: str):
        self.data["t_title"] = value

    @property
    def delete_state(self):
        return self.data["b_deleted"]

    @delete_state.setter
    def delete_state(self, value: bool):
        self.data["b_deleted"] = value

    @property
    def content(self):
        return self.data["m_content"]

    @content.setter
    def content(self, value):
        self.data["m_content"] = value
