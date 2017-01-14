'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QAbstractListModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt, QObject, pyqtSlot
from .. import cfg

class NoteListModel(QAbstractListModel):
    '''
    Tree
    '''
    IdRole = Qt.UserRole
    ProjectIdRole = Qt.UserRole + 1
    TitleRole = Qt.UserRole + 2
    ContentRole = Qt.UserRole + 3
    SortOrderRole = Qt.UserRole + 4
    IndentRole = Qt.UserRole + 5
    DateCreatedRole = Qt.UserRole + 6
    DateUpdatedRole = Qt.UserRole + 7
    DateContentRole = Qt.UserRole + 8
    DeletedRole = Qt.UserRole + 9
    VersionRole = Qt.UserRole + 10
    SheetCodeRole = Qt.UserRole + 20

    def __init__(self, parent: QObject):
        super(NoteListModel, self).__init__(parent)


        # inheriting classes will start at Qt.UserRole + 20

        self._root_node = ListItem()
        self._table_name = "tbl_note"
        self._id_name = "l_note_id"
        self._all_data = []
        self._node_list = []
        self._id_of_last_created_node = None
        self._id_list = []
        self._title_dict = {}
        self._indent_dict = {}
        self._sort_order_dict = {}
        self._sheet_code_dict = {}

        cfg.data.projectHub().projectLoaded.connect(self.reset_model)
        cfg.data.projectHub().projectClosed.connect(self.clear_from_project)
        cfg.data.projectHub().allProjectsClosed.connect(self.clear_from_all_projects)

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
        if role == self.ProjectIdRole and col == 0:
            return node.project_id
        if role == self.TitleRole and col == 0:
            return node.title
        if role == self.SortOrderRole and col == 0:
            return node.sort_order
        if role == self.IndentRole and col == 0:
            return node.indent
        if role == self.DateCreatedRole and col == 0:
            return node.creation_date
        if role == self.DateUpdatedRole and col == 0:
            return node.update_date
        if role == self.DateContentRole and col == 0:
            return node.content_update_date
        if role == self.DeletedRole and col == 0:
            return bool(node.deleted)
        if role == self.VersionRole and col == 0:
            return node.version

        if role == self.SheetCodeRole and col == 0:
            return node.sheet_code

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


        del self._root_node
        self._root_node = ListItem()
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

            self._populate_item(self._root_node, project_id)

        self.endResetModel()

    def _populate_item(self, parent_item, project_id: int):
        while self._id_list:

            _id = self._id_list.pop(0)
            item = ListItem(parent_item)
            item.id = _id
            item.project_id = project_id
            item.indent = self._indent_dict[_id]
            item.sort_order = self._sort_order_dict[_id]
            item.title = self._title_dict[_id]
            item.sheet_code = self._sheet_code_dict[_id]
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

    @pyqtSlot()
    def clear_from_all_projects(self):
        self.beginResetModel()
        self._root_node = ListItem()
        self.endResetModel()

    @pyqtSlot(int)
    def clear_from_project(self, projectId: int):
        # TODO : finish that
        self.beginResetModel()
        self._root_node = ListItem()
        self.endResetModel()


class ListItem(object):

    def __init__(self, parent=None):
        super(ListItem, self).__init__()

        self.data = {}
        self.index = QModelIndex()

        self.project_id = -1
        self.parent_id = None
        self.children_id = None
        self.properties = None

        self._parent = None

        self.parent = parent
        self.children = []

        self.id = -1
        self._indent = -1
        self.sort_order = None
        self.deleted = False
        self.creation_date = None
        self.update_date = None
        self.content_update_date = None
        self.version = None
        self.sheet_code = None

    @property
    def parent(self):
        return self._parent

    @parent.setter
    def parent(self, parent):
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

