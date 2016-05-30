'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.Qt import QAbstractTableModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt, QObject
from .. import cfg
from PyQt5.Qt import QUndoStack, QUndoCommand

class PropertyModel(QAbstractTableModel):
    '''
    Tree
    '''

    IdRole = Qt.UserRole
    NameRole = Qt.UserRole + 1
    ValueRole = Qt.UserRole + 2
    DateCreatedRole = Qt.UserRole + 3
    DateUpdatedRole = Qt.UserRole + 4
    SystemRole = Qt.UserRole + 5
    CodeRole = Qt.UserRole + 6

    def __init__(self, table_name: str, id_name: str, code_name: str, property_type: str
                 , parent: QObject, project_id: int):

        super(PropertyModel, self).__init__(parent)
        # inheriting classes will start at Qt.UserRole + 20

        self._project_id = project_id
        self._root_node = ListItem()
        self._table_name = table_name
        self._id_name = id_name
        self._code_name = code_name
        self._property_type = property_type
        self._all_data = []
        self._node_list = []
        self._id_of_last_created_node = None

        self.headers = [_("name"), _("value")]

        self._undo_stack = QUndoStack(self)

    @property
    def property_type(self):
        return self._property_type

    @property
    def table_name(self):
        return self._table_name

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

    def columnCount(self, parent=None):
        return 2

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

        if role == Qt.DisplayRole and col == 0:
            return node.name
        if role == Qt.DisplayRole and col == 1:
            return node.value

        if role == self.NameRole :
            return node.name
        if role == self.ValueRole:
            return node.value
        if role == self.IdRole:
            return node.id
        if role == self.CodeRole:
            return node.data[self._code_name]
        if role == self.DateCreatedRole:
            return node.data["dt_created"]
        if role == self.DateUpdatedRole:
            return node.data["dt_updated"]


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
        # name :
        if index.isValid() and index.column() == 0 and role == (Qt.EditRole or self.NameRole):

            node = self.node_from_index(index)

            # self.PropertyClass(node.id).name = value
            #
            # self.dataChanged.emit(index, index, limit)
            command = ChangeNameCommand(node.id, value, self)
            self.undo_stack.push(command)
            self.undo_stack.setActive(True)
            return True

        if index.isValid() and index.column() == 1 and role == (Qt.EditRole or self.ValueRole):

            node = self.node_from_index(index)
            command = ChangeValueCommand(node.id, value, self)
            self.undo_stack.push(command)
            self.undo_stack.setActive(True)
            # self.PropertyClass(node.id).value = value
            #
            # self.dataChanged.emit(index, index, limit)
            return True

        return False

    def flags(self, index):
        default_flags = QAbstractTableModel.flags(self, index)

        if index.isValid():
            return Qt.ItemIsEditable | Qt.ItemIsDragEnabled | \
                    Qt.ItemIsDropEnabled | default_flags

        else:
            return Qt.ItemIsDropEnabled | default_flags

    def reset_model(self):
        self.beginResetModel()

        self._all_data = cfg.data.get_database(0).get_table(self._table_name).get_all()
        all_headers = cfg.data.get_database(0).get_table(self._table_name).get_all_headers()
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
        self.undo_stack.clear()
        self.endResetModel()


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

    @property
    def undo_stack(self)->QUndoStack:
        return self._undo_stack

    def set_undo_stack_active(self):
        self._undo_stack.setActive(True)


    def set_property(self, paper_id: int, name: str, value: str):
        self.undo_stack.push(SetPropertyCommand(paper_id, name, value, self))


    def get_property(self, paper_id: int, name: str, default_value: str):
        property_id = self.find_id(paper_id, name)
        if property_id is None:
            property_id = Property(self._table_name, self._property_type, -1).add(paper_id)[0]
            self.undo_stack.clear()
            paper_property = Property(self._table_name, self._property_type, property_id)
            paper_property.name = name
            paper_property.value = default_value


        return Property(self._table_name, self._property_type, property_id).value


    def find_id(self, paper_id: int, name: str):
        """

        :param paper_id:
        :param name:
        :return: return None if nothing or id
        """
        return cfg.data.get_database(0).get_table(self._table_name).find_id(paper_id, name)


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
    def name(self):
        return self.data["t_name"]

    @property
    def value(self):
        return self.data["t_value"]


from ..property import Property

class ChangeNameCommand(QUndoCommand):
    def __init__(self, property_id: int, new_name: str, model: PropertyModel):
        super(ChangeNameCommand, self).__init__()
        self._model = model
        self._property_id = property_id
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        self._old = property.name
        self._new = new_name
        self.setText(_("change property name"))

    def redo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        property.name = self._new

    def undo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        property.name = self._old

class ChangeValueCommand(QUndoCommand):
    def __init__(self, property_id: int, new_value: str, model: PropertyModel):
        super(ChangeValueCommand, self).__init__()
        self._model = model
        self._property_id = property_id
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        self._old = property.value
        self._new = new_value
        self.setText(_("change property value"))

    def redo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        property.value = self._new

    def undo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        property.value = self._old


class RemovePropertyCommand(QUndoCommand):
    def __init__(self, property_id: int, model: PropertyModel):
        super(RemovePropertyCommand, self).__init__()
        self._model = model
        self._property_id = property_id
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        self._paper_code = property.paper_code
        self._saved_property = property
        self._saved_property.copy_deeply()
        self.setText(_("remove property"))

    def redo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self._property_id)
        property.remove()

    def undo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=-1)
        property.add(self._paper_code, [self._property_id])
        self._saved_property.paste_deeply()

class AddPropertyCommand(QUndoCommand):
    def __init__(self, paper_id: int, model: PropertyModel):
        super(AddPropertyCommand, self).__init__()
        self._model = model
        self._paper_id = paper_id
        self.new_id_list = []
        self._saved_property = None
        self.setText(_("add property"))


    @property
    def last_new_id(self):
        return self.new_id_list[0]

    def redo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=-1)
        self.new_id_list = property.add(self._paper_id, self.new_id_list)
        if self._saved_property is not None:
            self._saved_property.paste_deeply()


    def undo(self):
        property = Property(table_name=self._model.table_name, property_type=self._model.property_type
                            , property_id=self.new_id_list[0])
        self._saved_property = property
        self._saved_property.copy_deeply()
        property.remove(self.new_id_list)

class SetPropertyCommand(QUndoCommand):
    def __init__(self, paper_id: int, name: str, value: str, model: PropertyModel):
        super(SetPropertyCommand, self).__init__()
        self._model = model
        self._paper_id = paper_id
        self._name = name
        self._value = value
        self._saved_property = None
        self._property_id = None
        self._property_created = False
        self.setText(_("set property"))

        self._command_add = AddPropertyCommand(self._paper_id, self._model)

    @property
    def last_new_id(self):
        return self._property_id

    def redo(self):
        self._property_id = self._model.find_id(self._paper_id, self._name)

        if self._property_id is None:
            self._command_add.redo()
            self._property_id = self._command_add.last_new_id
            self._property_created = True
        self._command_change_name = ChangeNameCommand(self._property_id, self._name, self._model)
        self._command_change_name.redo()
        self._command_change_value = ChangeValueCommand(self._property_id, self._value, self._model)
        self._command_change_value.redo()

    def undo(self):

        self._command_change_name.undo()
        self._command_change_value.undo()
        if self._property_created:
            self._command_add.undo()



