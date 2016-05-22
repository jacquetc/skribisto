'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from gui import plugins as gui_plugins
from PyQt5.Qt import pyqtSlot


class PropertiesDockPlugin(gui_plugins.GuiWriteSubWindowDockPlugin):

    '''
    PropertiesDockPlugin
    '''
    is_builtin_plugin = True
    ignore = False

    def __init__(self):
        '''
        Constructor
        '''

        super(PropertiesDockPlugin, self).__init__()

    def gui_class(self):
        return GuiPropertyDock


# class CorePropertyDock():
#
#     '''
#     CorePropertyDock
#     '''
#
#     dock_name = "properties-dock"
#
#     def __init__(self):
#         '''
#         Constructor
#         '''
#
#         super(CorePropertyDock, self).__init__()
#         self._property_table_model = None
#         self._sheet_id = None
#         self.tree_sheet = None
#
#     @property
#     def sheet_id(self):
#         return self._sheet_id
#
#     @sheet_id.setter
#     def sheet_id(self, sheet_id):
#         if self._sheet_id == sheet_id:
#             pass
#         self._sheet_id = sheet_id
#         if self.sheet_id is not None:
#             self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
#                 self.sheet_id)
#             _ = self.property_table_model
#
#     @property
#     def property_table_model(self):
#         if self._property_table_model is None:
#             self._property_table_model = PropertyTableModel(self)
#             if self._sheet_id is not None:
#                 self._property_table_model.set_sheet_id(self._sheet_id)
#                 self._property_table_model.tree_sheet \
#                     = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(self.sheet_id)
#
#         return self._property_table_model
#
#     @pyqtSlot()
#     def add_property_row(self, index):
#         core_cfg.data.database.sheet_tree.set_property(self.sheet_id, "", "")
#         #self._property_table_model.insertRow(1, index)
#
#     @pyqtSlot()
#     def remove_property_row(self, index):
#         node = self.property_table_model.node_from_index(index)
#         core_cfg.data.database.sheet_tree.remove_property(self.sheet_id, node.key)
#         #self._property_table_model.removeRow(
#         #    index.row(), self._property_table_model.root_model_index())

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QSortFilterProxyModel
from gui import cfg as gui_cfg
from plugins.propertiesdock import properties_dock_ui


class GuiPropertyDock():

    '''
    GuiPropertyDock
    '''
    dock_name = "properties-dock"
    dock_displayed_name = _("Properties")

    def __init__(self):
        '''
        Constructor
        '''
        super(GuiPropertyDock, self).__init__()
        self.widget = None
        # self.core_part = None  # CorePropertyDock
        self._sheet_id = None
        self.tree_sheet = None

    @property
    def sheet_id(self):
        return self._sheet_id

    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            pass
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self.tree_sheet = gui_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
                self.sheet_id)
            # self.core_part = self.tree_sheet.get_instance_of(self.dock_name)

    def get_widget(self):

        if self.widget is None:
            self.widget = QWidget()
            self.ui = properties_dock_ui.Ui_PropertiesDock()
            self.ui.setupUi(self.widget)

            if self.tree_sheet is not None and self.core_part is not None:
                table_model = self.core_part.property_table_model

                # filter :
                self.filter = QSortFilterProxyModel(self.widget)
                self.filter.setFilterKeyColumn(-1)
                self.filter.setFilterCaseSensitivity(False)
                self.filter.setSourceModel(table_model)

                # model :
                self.ui.tableView.setModel(self.filter)

                # connect :
                self.ui.addPropButton.clicked.connect(self.add_property_row)
                self.ui.removePropButton.clicked.connect(
                    self.remove_property_row)
                self.ui.filterLineEdit.textChanged.connect(
                    self.filter.setFilterFixedString)
                self.ui.tableView.clicked.connect(self.set_current_row)

            self.widget.gui_part = self
        return self.widget

    @pyqtSlot()
    def add_property_row(self):
        index = self.filter.mapToSource(self.ui.tableView.currentIndex())
        self.core_part.add_property_row(index)

    @pyqtSlot()
    def remove_property_row(self):
        index = self.filter.mapToSource(self.ui.tableView.currentIndex())
        self.core_part.remove_property_row(index)

    @pyqtSlot('QModelIndex')
    def set_current_row(self, model_index):
        self.ui.tableView.setCurrentIndex(model_index)

from PyQt5.QtCore import QAbstractTableModel, QVariant, QModelIndex, Qt


class PropertyTableModel(QAbstractTableModel):

    '''
    PropertyTableModel
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(PropertyTableModel, self).__init__(parent=None)

        self.tree_sheet = None
        self.root_node = TableNode()
        self.headers = ["property", "value"]

    def set_sheet_id(self, sheet_id):
        # unsubscribe:
        core_cfg.data.subscriber.unsubscribe_update_func(self.reset_model)
        self._sheet_id = sheet_id
        self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
            sheet_id)
        # subscribe:
        core_cfg.data.subscriber.subscribe_update_func_to_domain(0,
            self.reset_model,  "data.sheet_tree.properties",  self._sheet_id)
        self.reset_model()

    def columnCount(self, parent):
        '''
        function:: columnCount(parent)
        :param parent:
        '''
        return 2

    def rowCount(self, parent):
        '''
        function:: rowCount(parent)
        :param parent:
        '''
        # if parent.column() > 0:
        #   return 0

        parent_node = self.root_node

        return len(parent_node)

    def headerData(self, section, orientation, role):
        '''
        function:: headerData(section, orientation, role)
        :param section:
        :param orientation:
        :param role:
        '''
        if orientation == Qt.Horizontal and role == Qt.DisplayRole:
            return QVariant(self.headers[section])
        return QVariant()

    def index(self, row, column, parent):
        '''
        function:: index(row, column, parent)
        :param row:
        :param column:
        :param parent:
        '''
        if not self.hasIndex(row, column, parent):
            return QModelIndex()

        node = self.node_from_index(parent)
        return self.createIndex(row, column, node.childAtRow(row))

    def data(self, index, role):
        '''
        function:: data(index, role)
        :param index:
        :param role:
        '''
        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if col == 0 and (role == Qt.DisplayRole or role == Qt.EditRole):
            return node.key
        if col == 1 and (role == Qt.DisplayRole or role == Qt.EditRole):
            return node.value

        return QVariant()

    def parent(self, child):
        '''
        function:: parent(child)
        :param child:
        '''
        if not child.isValid():
            return QModelIndex()

        node = self.node_from_index(child)

        if node is None:
            return QModelIndex()

        parent = node.parent
        if parent == self.root_node:
            return QModelIndex()

        if parent is None:
            return QModelIndex()

        grandparent = parent.parent
        if grandparent is None:
            return QModelIndex()
        row = grandparent.rowOfChild(parent)

        assert row != - 1
        return self.createIndex(row, 0, parent)

    def node_from_index(self, index):
        '''
        function:: node_from_index(index)
        :param index:
        '''
        return index.internalPointer() if index.isValid() else self.root_node

    def root_model_index(self):
        '''
        function:: root_model_index()
        '''
        return QModelIndex()
#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):
        '''
        function:: setData(index, value, role)
        :param index:
        :param value:
        :param role:
        '''
        limit = [role]
        if not index.isValid():
            return False
        # title :
        if role == Qt.EditRole and index.column() == 0:

            node = self.node_from_index(index)
            self.tree_sheet.change_property_key(node.key, value)
            node.key = value

            self.dataChanged.emit(index, index, limit)
            return True

        if role == Qt.EditRole and index.column() == 1:

            node = self.node_from_index(index)
            self.tree_sheet.set_property(node.key, value)
            node.value = value

            self.dataChanged.emit(index, index, limit)
            return True
        return False

    def supportedDropActions(self):
        '''
        function:: supportedDropActions()
        :param :
        '''
        return Qt.CopyAction | Qt.MoveAction

    def flags(self, index):
        '''
        function:: flags(index)
        :param index:
        :rtype:         return

        '''
        defaultFlags = QAbstractTableModel.flags(self, index)

        if index.isValid():
            return Qt.ItemIsEditable | Qt.ItemIsDragEnabled | \
                Qt.ItemIsDropEnabled | defaultFlags

        else:
            return Qt.ItemIsDropEnabled | defaultFlags

    def insertRow(self, row, parent):
        '''
        function:: insertRow(row, parent)
        :param row:
        :param parent:
        '''
        return self.insertRows(row, 1, parent)

    def insertRows(self, row, count, parent):
        '''
        function:: insertRows(row, count, parent)
        :param row:
        :param count:
        :param parent:
        '''
        if not parent.isValid():
            parent = QModelIndex()
        #self.beginInsertRows(parent, row, (row + (count - 1)))
        #self.create_child_nodes(self.root_node, {"": ""})
        #self.endInsertRows()
        return True

    def removeRow(self, row, parentIndex):
        '''
        function:: removeRow(row, parentIndex)
        :param row:
        :param parentIndex:
        '''
        return self.removeRows(row, 1, parentIndex)

    def removeRows(self, row, count, parentIndex):
        '''
        function:: removeRows(row, count, parentIndex)
        :param row:
        :param count:
        :param parentIndex:
        '''
        if not parentIndex.isValid():
            return False
        # self.beginRemoveRows(parentIndex, row, row)
        # node = self.node_from_index(parentIndex)
        # node.removeChild(row)
        # self.endRemoveRows()

        return True

        pass

    def reset_model(self):
        '''
        function:: reset_model()
        :param :
        '''
        self.beginResetModel()

        del self.root_node
        self.root_node = TableNode()

        # create a nice dict
        self.prop_list = self.tree_sheet.get_properties()

        self.root_node.sheet_id = self._sheet_id
        self.create_child_nodes(self.root_node, self.prop_list)

        self.endResetModel()

    def apply_node_variables_from_dict(self, node, sheet_id):
        '''
        function:: apply_node_variables_from_dict(node, sheet_id, dict_)
        :param node:
        :param sheet_id:
        :param dict_:
        '''
        pass

    def create_child_nodes(self, parent_node, property_list):
        '''
        function:: create_child_nodes(parent_node)
        :param parent_node:
        :rtype:

        '''
        # add & scan children :

        if len(property_list) is not 0:
            for property_ in property_list:

                child_node = TableNode(parent_node)
                child_node.key = property_.name
                child_node.value = property_.value
                child_node.sheet_id = self._sheet_id
                child_node.children_id = None
                parent_node.append_child(child_node)
                # self.create_child_nodes(child_node)


class TableNode():

    '''
    TableNode
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(TableNode, self).__init__()

        self.sheet_id = 0
        self.key = ""
        self.value = ""
        self.parent = parent
        self.children = []

    def setParent(self, parent):
        '''
        function:: setParent(parent)
        :param parent:
        :rtype:

        '''
        if parent != None:
            self.parent = parent
            self.parent.append_child(self)
        else:
            self.parent = None

    def appendChild(self, child):
        '''
        function:: appendChild(child)
        :param child:
        :rtype:         return

        '''
        if not child in self.children:
            self.children.append(child)

    def childAtRow(self, row):
        '''
        function:: childAtRow(row)
        :param row:
        :rtype:         return

        '''
        return self.children[row]

    def rowOfChild(self, child):
        '''
        function:: rowOfChild(child)
        :param child:
        :rtype:         return

        '''
        for i, item in enumerate(self.children):
            if item == child:
                return i
        return -1

    def removeChild(self, row):
        '''
        function:: removeChild(self, row)
        :param self:
        :param row:
        :rtype:         return

        '''
        value = self.children[row]
        self.children.remove(value)

        return True

    def __len__(self):
        '''
        function:: __len__(self)
        :param self:
        :rtype:         return

        '''

        return len(self.children)
