'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QAbstractListModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt, QObject
from .. import cfg

class NoteListModel(QAbstractListModel):
    '''
    Tree
    '''
    IdRole = Qt.UserRole
    TitleRole = Qt.UserRole + 1
    ContentRole = Qt.UserRole + 2
    SortOrderRole = Qt.UserRole + 3
    IndentRole = Qt.UserRole + 4
    DateCreatedRole = Qt.UserRole + 5
    DateUpdatedRole = Qt.UserRole + 6
    DateContentRole = Qt.UserRole + 7
    DeletedRole = Qt.UserRole + 8
    VersionRole = Qt.UserRole + 9
    CharCountRole = Qt.UserRole + 10
    WordCountRole = Qt.UserRole + 11

    SheetCodeRole = Qt.UserRole + 20

    def __init__(self, parent: QObject, project_id: int):
        super(NoteListModel, self).__init__(parent)


        # inheriting classes will start at Qt.UserRole + 20

        self._project_id = project_id
        self._root_node = ListItem()
        self._table_name = "tbl_note"
        self._id_name = "l_note_id"
        self._all_data = []
        self._node_list = []
        self._id_of_last_created_node = None

        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.title_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.tree_structure_modified")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.properties")

    @property
    def tree_db(self):
        return cfg.data.database(self._project_id).get_tree(self._table_name);


    def rowCount(self, parent=None):

        if parent.column() > 0:
            return 0

        if not parent.isValid():
            parent_node = self._root_node

            return len(parent_node)
        else:
            parent_node = self.node_from_index(parent)
            return len(parent_node)

    # def headerData(self, section, orientation, role):
    #     if orientation == Qt.Horizontal and role == Qt.DisplayRole:
    #         return QVariant(self.headers[section])
    #     return QVariant()

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

        if role == self.IdRole and col == 0:
            return node.id
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


        if role == self.SheetCodeRole and col == 0:
            return node.data["l_sheet_code"]

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

    def flags(self, index):
        default_flags = QAbstractListModel.flags(self, index)

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
        self._root_node = ListItem()
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

            data_dict = self._all_data.pop(0)
            item = ListItem(parent_item)
            item.data = data_dict
            item.id = data_dict[self._id_name]
            self._node_list.append(item)
            parent_item.append_child(item)


    def clear(self):
        self.beginResetModel()
        self._root_node = ListItem()
        self.endResetModel()


    @property
    def item_list(self):
        return self._item_list

    def insert_child_node(self, parent_index):
        self.id_of_last_created_node = \
                cfg.data.database.get_tree(self._table_name).add_new_child_papers(self.node_from_index(parent_index).id, 1)

    def insert_node_by(self, index):
        self.id_of_last_created_node = \
                cfg.data.database.get_tree(self._table_name).add_new_papers_by(self.node_from_index(index).id, 1)

    def remove_node(self, index):
        cfg.data.database.get_tree(self._table_name).remove_papers(self.node_from_index(index).id, 1)

    # def insertRow(self, row, parent):
    #     return self.insertRows(row, 1, parent)
    #
    #
    # def insertRows(self, row, count, parent):
    #     self.beginInsertRows(parent, row, (row + (count - 1)))
    #
    #     self.id_of_last_created_node = \
    #         cfg.data.database.get_tree(self._table_name).add_new_papers_by(self.node_from_index(parent).id, count)
    #     self.endInsertRows()
    #     return True
    #
    #
    # def removeRow(self, row, parentIndex):
    #     return self.removeRows(row, 1, parentIndex)
    #
    #
    # def removeRows(self, row, count, parentIndex):
    #     # TODO wrong
    #     self.beginRemoveRows(parentIndex, row, row)
    #     cfg.data.database.get_tree(self._table_name).remove_papers(self.node_from_index(parent).id, count)
    #     self.endRemoveRows()
    #
    #     return True



    @property
    def id_of_last_created_node(self):
        return self._id_of_last_created_node

    @id_of_last_created_node.setter
    def id_of_last_created_node(self, value):
        self._id_of_last_created_node = value[-1]

    def find_index_from_id(self, id_: int):
        for node in self._node_list:
            if node.id == id_:
                return node.index

        return None

    def find_parent_id_from_child_node_id(self, child_node_id):
        # find child
        parent_node = None
        child_indent = -1
        child_index = -1
        for node in self.tuple_of_tree_nodes:
            if node.id == child_node_id:
                child_index = self.tuple_of_tree_nodes.index(node)
                child_indent = node.indent
        # find parent
        for node in self.tuple_of_tree_nodes:
            index = self.tuple_of_tree_nodes.index(node)
            if node.indent is child_indent - 1 and index < child_index:
                parent_node = node

        if parent_node is None:
            return -1

        return parent_node.id

# TODO : drag drop
#     def mimeData(self, list_of_QModelIndex):
#         custom_mime_data = QMimeData()
#         encoded_data = QByteArray()
#
#         stream = QDataStream(encoded_data, QIODevice.WriteOnly)
#
#         for index in list_of_QModelIndex:
#             if index.isValid():
#                 stream.writeQVariant(self.node_from_index(index).data)
#
#         custom_mime_data.setData("application/plume-creator-tree", encoded_data)
#
#         return custom_mime_data
#
#     def mimeTypes(self):
#         return ["application/plume-creator-tree"]
#
#     def dropMimeData(self, mimedata, action, row, column, parentIndex):
#         if action == Qt.IgnoreAction:
#             return True
#
#         if
#
#         return True

    def supportedDropActions(self):
        return Qt.CopyAction

    def insertRow(self, row, parent):
        return self.insertRows(row, 1, parent)

    def insertRows(self, row, count, parent):
        self.beginInsertRows(parent, row, (row + (count - 1)))
        self.endInsertRows()
        return True

    def removeRow(self, row, parentIndex):
        return self.removeRows(row, 1, parentIndex)

    def removeRows(self, row, count, parentIndex):
        self.beginRemoveRows(parentIndex, row, row)
        self.endRemoveRows()
        return True


class ListItem(object):

    def __init__(self, parent=None):
        super(ListItem, self).__init__()

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
