'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt
from .. import cfg
from .tree_model import TreeModel


class SheetTreeModel(TreeModel):
    '''
    classdocs
    '''

    def __init__(self, parent, project_id):
        super(SheetTreeModel, self).__init__("tbl_sheet", "l_sheet_id", parent, project_id)

        '''
        Constructor
        '''




        self._item_list = []

        self.headers = ["name"]
        self._id_of_last_created_sheet = None     
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "data.sheet_tree")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "data.sheet_tree.title")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "data.sheet_tree.properties")



    def data(self, index, role):
        
        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if role == Qt.DisplayRole and col == 0:
            return self.data(index, self.TitleRole)

        return TreeModel.data(self, index, role)

    
    

#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):
        
        limit = [role]
        
        # # title :
        # if index.isValid() & role == Qt.EditRole & index.column() == 0:
        #
        #     node = self.node_from_index(index)
        #
        #     cfg.data.database.sheet_tree.set_title(node.sheet_id, value)
        #     node.title = value
        #
        #     self.dataChanged.emit(index, index, limit)
        #     return True
        
        return TreeModel.setData(self, index, value, role)


    '''
    def mimeTypes(self):
        types = QStringList()
        types.append('application/x-ets-qt4-instance')
        return types

    def mimeData(self, index):
        node = self.node_from_index(index[0])
        mimeData = PyMimeData(node)
        return mimeData


    def dropMimeData(self, mimedata, action, row, column, parentIndex):
        if action == Qt.IgnoreAction:
            return True

        dragNode = mimedata.instance()
        parentNode = self.node_from_index(parentIndex)

        # make an copy of the node being moved
        newNode = deepcopy(dragNode)
        newNode.setParent(parentNode)
        self.insertRow(len(parentNode)-1, parentIndex)
        self.emit(SIGNAL("dataChanged(QModelIndex,QModelIndex)"), 
parentIndex, parentIndex)
        return True

    '''



    '''
    def insert_child_row(self, parent_index):
        self._id_of_last_created_sheet = \
                cfg.data.database.sheet_tree.create_new_child_item(self.node_from_index(parent_index).sheet_id)

    def insertRow(self, row, parent):
        return self.insertRows(row, 1, parent)


    def insertRows(self, row, count, parent):
        self.beginInsertRows(parent, row, (row + (count - 1)))
        for _ in range(0, count):
            self._id_of_last_created_sheet = \
                cfg.data.database.sheet_tree.create_new_item_after(self.node_from_index(parent).sheet_id)
        self.endInsertRows()
        return True


    def removeRow(self, row, parentIndex):
        return self.removeRows(row, 1, parentIndex)


    def removeRows(self, row, count, parentIndex):
        self.beginRemoveRows(parentIndex, row, row)
        node = self.node_from_index(parentIndex)
        node.removeChild(row)
        self.endRemoveRows()
       
        return True



    @property
    def id_of_last_created_sheet(self):       
        return self._id_of_last_created_sheet

    def find_index_from_id(self, id_):
        start_index = self.index(0,0, QModelIndex())
        index_list = self.match(start_index, 37, id_, 1, Qt.MatchExactly | Qt.MatchRecursive)
        if len(index_list) == 0:
            return None
        else:
            return index_list[0]

    def find_parent_id_from_child_sheet_id(self, child_sheet_id):
        # find child
        parent_node = None
        child_indent = -1
        child_index = -1
        for node in self.tuple_of_tree_nodes:
            if node.sheet_id == child_sheet_id:
                child_index = self.tuple_of_tree_nodes.index(node)
                child_indent = node.indent
        # find parent
        for node in self.tuple_of_tree_nodes:
            index = self.tuple_of_tree_nodes.index(node)
            if node.indent is child_indent - 1 and index < child_index:
                parent_node = node

        if parent_node is None:
            return -1

        return parent_node.sheet_id


    def create_child_nodes(self, parent_node):

        # self.apply_node_variables_from_dict(parent_node, parent_node.sheet_id, self._dict)

        list_of_direct_child_nodes = []
        for node in self.tuple_of_tree_nodes:
            if node.sort_order > parent_node.sort_order and node.indent <= parent_node.indent:
                break
            if node.sort_order > parent_node.sort_order and node.indent is parent_node.indent + 1:
                list_of_direct_child_nodes.append(node)


        for child_node in list_of_direct_child_nodes:
            parent_node.append_child(child_node)
            child_node.setParent(parent_node)
            self.create_child_nodes(child_node)




        
        
    def get_synopsys(self,sheet_id):
        pass
    
    def set_synopsys(self,sheet_id, content):
        pass
    
    def get_notes(self,sheet_id):
        pass
    
    def set_notes(self,sheet_id, content):
        pass

    '''
