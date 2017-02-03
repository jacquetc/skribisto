'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex, QPersistentModelIndex, QSettings
from PyQt5.QtCore import Qt, QObject, QMimeData, QByteArray, QDataStream, QIODevice, pyqtSlot
from PyQt5.Qt import QUndoStack
from .. import cfg

class TreeModel(QAbstractItemModel):
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
    CharCountRole = Qt.UserRole + 11
    WordCountRole = Qt.UserRole + 12

    def __init__(self, id_name: str, paper_type: str, parent: QObject):
        super(TreeModel, self).__init__(parent)


        # inheriting classes will start at Qt.UserRole + 20

        self._root_node = TreeItem()
        self._paper_type = paper_type
        self._id_name = id_name
        self._all_data = []
        self._node_list = []
        self._id_of_last_created_node = None
        self._undo_stack = QUndoStack(self)
        self._id_list = []
        self._title_dict = {}
        self._indent_dict = {}
        self._sort_order_dict = {}

        self._settings = QSettings()


        cfg.data.projectHub().projectClosed.connect(self.clear_from_project)
        cfg.data.projectHub().allProjectsClosed.connect(self.clear_from_all_projects)

        if paper_type == "sheet":
            self.hub = cfg.data.sheetHub()
        if paper_type == "note":
            self.hub = cfg.data.noteHub()

        self.hub.paperRemoved.connect(self.reset_model)
        self.hub.paperAdded.connect(self.reset_model)
        self.hub.titleChanged.connect(self._update_index_title)
        self.hub.idChanged.connect(self._update_index_id)
        self.hub.deletedChanged.connect(self._update_index_deleted)

    @property
    def table_name(self):
        return self._table_name

    @property
    def paper_type(self):
        return self._paper_type

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
        child_node = node.child_at_row(row)
        index = self.createIndex(row, column, child_node)
        node.index = index

        return index

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

        # for development :
        if bool(self._settings.value("settings/misc/dev_mode", False)):
            if role == Qt.ToolTipRole and col == 0:
                return str(node.id) + " : " + str(node.sort_order)

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


        # title :
        if index.isValid() & role == Qt.EditRole & index.column() == 0:
            node = self.node_from_index(index)

            command = ChangeTitleCommand(node.project_id, node.id, value, self)
            self.undo_stack.push(command)
            self.undo_stack.setActive(True)

            # self.dataChanged.emit(index, index, limit)
            return True

        return False

    def flags(self, index):
        default_flags = QAbstractItemModel.flags(self, index)

        if index.isValid():
            return Qt.ItemIsEditable | Qt.ItemIsDragEnabled | \
                    Qt.ItemIsDropEnabled | default_flags

        else:
            return Qt.ItemIsDropEnabled | default_flags
    #
    # def reset_model(self):
    #     self.beginResetModel()
    #     #
    #     # self._all_data = cfg.data.get_database(0).get_tree(self._table_name).get_all()
    #     # all_headers = cfg.data.get_database(0).get_tree(self._table_name).get_all_headers()
    #     # header_dict = {}
    #     # for header in all_headers:
    #     #     header_dict[header] = QVariant()
    #
    #     del self._root_node
    #     self._root_node = TreeItem()
    #     self._root_node.sheet_id = -1
    #     self._root_node.indent = -1
    #     self._root_node.sort_order = -1
    #
    #     self._root_node.data = header_dict
    #     #
    #     # if self._all_data is []: # empty /close
    #     #     self.endResetModel()
    #     #     return
    #
    #     self._node_list = []
    #     self._populate_item(self._root_node)
    #
    #     self.endResetModel()
    #
    # def _populate_item(self, parent_item):
    #     while self._all_data:
    #         indent = self._all_data[0]["l_indent"]
    #
    #         if parent_item.indent < indent:
    #             data_dict = self._all_data.pop(0)
    #             item = TreeItem(parent_item)
    #             item.data = data_dict
    #             item.id = data_dict[self._id_name]
    #             self._populate_item(item)
    #             self._node_list.append(item)
    #             parent_item.append_child(item)
    #         else:
    #             return
    #
    @pyqtSlot()
    def clear_from_all_projects(self):
        self.beginResetModel()
        self._root_node = TreeItem()
        self.undo_stack.clear()
        self.endResetModel()

    @pyqtSlot(int)
    def clear_from_project(self, projectId: int):
        # TODO : finish that
        self.beginResetModel()
        self._root_node = TreeItem()
        self.undo_stack.clear()
        self.endResetModel()

    @property
    def item_list(self):
        return self._item_list



    # deprecated
    # def insert_child_node(self, parent_index):
    #     #self.id_of_last_created_node = \
    #     #       cfg.data.database.get_tree(self._table_name).add_new_child_papers(self.node_from_index(parent_index).id, 1)
    #
    #  # deprecated
    # def insert_node_by(self, index):
    #     self.id_of_last_created_node = \
    #             cfg.data.database.get_tree(self._table_name).add_new_papers_by(self.node_from_index(index).id, 1)
    #
    # # deprecated
    # def remove_node(self, index):
    #     cfg.data.database.get_tree(self._table_name).remove_papers(self.node_from_index(index).id, 1)

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
        for node in self._node_list:
            if node.id == child_node_id:
                child_index = self.tuple_of_tree_nodes.index(node)
                child_indent = node.indent
        # find parent
        for node in self._node_list:
            index = self._node_list.index(node)
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


    @property
    def undo_stack(self)->QUndoStack:
        return self._undo_stack

    def set_undo_stack_active(self):
        self._undo_stack.setActive(True)

    def _update_index_id(self, project_id: int, paper_id: int, new_id: str):
        for node in self._node_list:
            if node.project_id == project_id and node.id == paper_id:
                node.id = new_id
                self.dataChanged.emit(node.index, node.index, [self.IdRole])

    def _update_index_title(self, project_id: int, paper_id: int, new_title: str):
        for node in self._node_list:
            if node.project_id == project_id and node.id == paper_id:
                node.title = new_title
                self.dataChanged.emit(node.index, node.index, [self.TitleRole])

    def _update_index_deleted(self, project_id: int, paper_id: int, new_value: bool):
        for node in self._node_list:
            if node.project_id == project_id and node.id == paper_id:
                node.deleted = new_value
                for child_node in self.all_children_nodes(node):
                    child_node.deleted = new_value

                index_list = self.match(self.index(0, 0, QModelIndex()), self.IdRole, paper_id
                                        , 1, (Qt.MatchExactly | Qt.MatchRecursive))
                if index_list:
                    index = index_list[0]
                    if index.isValid():
                        self.dataChanged.emit(index, index, [self.DeletedRole])

    def all_children_nodes(self, node) -> list:

        children_list = []
        for child_node in node.children:
            children_list += self.all_children_nodes(child_node)

        return children_list


class TreeItem(object):

    def __init__(self, parent=None):
        super(TreeItem, self).__init__()

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
        self.indent_drift = 0
        self.sort_order = None
        self.deleted = False
        self.creation_date = None
        self.update_date = None
        self.content_update_date = None
        self.version = None

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

    @property
    def indent(self):
        return self._indent - self.indent_drift

    @indent.setter
    def indent(self, value:int):
        self._indent = value + self.indent_drift



from PyQt5.Qt import QUndoCommand
from ..paper_manager import Paper


class ChangeTitleCommand(QUndoCommand):
    def __init__(self, project_id: int, paper_id: int, new_title: str, model: TreeModel):
        """

        :param project_id:
        :param paper_id:
        :param new_title:
        :param model:
        """
        super(ChangeTitleCommand, self).__init__()
        self._model = model
        self._paper_id = paper_id
        self._project_id = project_id
        paper = Paper(paper_type=self._model._paper_type, project_id=project_id, paper_id=self._paper_id)
        self._old = paper.title
        self._new = new_title
        self.setText(_("change title"))

    def redo(self):
        paper = Paper(paper_type=self._model._paper_type, project_id=self._project_id, paper_id=self._paper_id)
        paper.title = self._new

    def undo(self):
        paper = Paper(paper_type=self._model._paper_type, project_id=self._project_id, paper_id=self._paper_id)
        paper.title = self._old


class DeleteCommand(QUndoCommand):
    def __init__(self, project_id: int, paper_id: int, value: bool, model: TreeModel):
        super(DeleteCommand, self).__init__()
        self._model = model
        self._paper_id = paper_id
        self._project_id = project_id
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self._paper_id)
        self._old = paper.deleted
        self._new = value
        if value is True:
            self.setText(_("delete node"))
        else:
            self.setText(_("restore node"))

    def redo(self):
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self._paper_id)
        paper.deleted = self._new

    def undo(self):
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self._paper_id)
        paper.deleted = self._old


class AddChildNodeCommand(QUndoCommand):
    def __init__(self, project_id: int, parent_paper_id: int, model: TreeModel):
        super(AddChildNodeCommand, self).__init__()
        self._model = model
        self._project_id = project_id
        self.new_child_id_list = []
        self._parent_paper_id = parent_paper_id
        self.setText(_("add child node"))

    @property
    def last_new_child_id(self):
        if not self.new_child_id_list:
            return -1
        else:
            return self.new_child_id_list[0]

    def redo(self):
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self._parent_paper_id)
        self.new_child_id_list = paper.add_child_paper(self.new_child_id_list)

    def undo(self):
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self.last_new_child_id)
        paper.remove_paper()


class AddAfterNodeCommand(QUndoCommand):
    def __init__(self, project_id: int, parent_paper_id: int, model: TreeModel):
        super(AddAfterNodeCommand, self).__init__()
        self._model = model
        self._project_id = project_id
        self.new_id_list = []
        self._parent_paper_id = parent_paper_id
        self.setText(_("add node below"))

    @property
    def last_new_id(self):
        return self.new_id_list[0]

    def redo(self):
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self._parent_paper_id)
        self.new_id_list = paper.add_paper_after(self.new_id_list)

    def undo(self):
        print(self.last_new_id)
        paper = Paper(paper_type=self._model.paper_type, project_id=self._project_id, paper_id=self.last_new_id)
        paper.remove_paper()

