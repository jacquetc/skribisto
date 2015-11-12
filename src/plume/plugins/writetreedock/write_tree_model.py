'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from PyQt5.Qt import QAbstractItemModel, QVariant, QModelIndex 
from PyQt5.QtCore import Qt
from core import subscriber, cfg


class WriteTreeModel(QAbstractItemModel):
    '''
    classdocs
    '''


    def __init__(self, parent=None):
        super(WriteTreeModel, self).__init__(parent)
        '''
        Constructor
        '''
        self.root_node = TreeNode()

        self.headers = ["name"]
        self._id_of_last_created_sheet = None     
        cfg.data.subscriber.subscribe_update_func_to_domain(0, self.reset_model, "data.project.close")
        cfg.data.subscriber.subscribe_update_func_to_domain(0, self.reset_model, "data.sheet_tree")
        cfg.data.subscriber.subscribe_update_func_to_domain(0, self.reset_model, "data.sheet_tree.title")
        cfg.data.subscriber.subscribe_update_func_to_domain(0, self.reset_model, "data.sheet_tree.properties")

    def columnCount(self, parent):
        return 1


    def rowCount(self, parent):
        
        if parent.column() > 0:
            return 0

        if not parent.isValid():
            parent_node = self.root_node
            
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
        return self.createIndex(row, column, node.childAtRow(row))


    def data(self, index, role):
        
        row = index.row()
        col = index.column() 
        
               
        if not index.isValid():
            return QVariant()
        

                   
        node = self.node_from_index(index)
       
        
        if (role == Qt.DisplayRole or role == Qt.EditRole) and col == 0:
            return node.title
    
        # properties :
        if role == Qt.UserRole:
            return node.properties;
        # sheet_id :
        if role == 37:
            return node.sheet_id;
            
        return QVariant();

    def parent(self, child):
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
        return index.internalPointer() if index.isValid() else self.root_node
   
#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):
        
        limit = [role]
        
        # title :
        if index.isValid() & role == Qt.EditRole & index.column() == 0:
            
            node = self.node_from_index(index)
            
            cfg.data.database.sheet_tree.set_title(node.sheet_id, value)
            node.title = value
            
            
            self.dataChanged.emit(index, index, limit)
            return True
        
        return False
 
    def supportedDropActions(self):
        return Qt.CopyAction | Qt.MoveAction


    def flags(self, index):
        defaultFlags = QAbstractItemModel.flags(self, index)
       
        if index.isValid():
            return Qt.ItemIsEditable | Qt.ItemIsDragEnabled | \
                    Qt.ItemIsDropEnabled | defaultFlags
           
        else:
            return Qt.ItemIsDropEnabled | defaultFlags
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
    
    def reset_model(self):
        self.beginResetModel()
        

        del self.root_node
        self.root_node = TreeNode()
        self.root_node.sheet_id = -1
        self.root_node.indent = -1
        self.root_node.sort_order = -1

        self._tree_in_tuple = cfg.data.database.sheet_tree.get_tree_model_necessities()

        if self._tree_in_tuple is []:        #empty /close
            self.endResetModel()
            return           

        # create tuple of elements
        list_of_tree_nodes = []
        for tuple_ in self._tree_in_tuple:
            node = TreeNode()
            node.sheet_id = tuple_[0]
            node.title = tuple_[1]
            node.sort_order = tuple_[2]
            node.indent = tuple_[3]
            node.deleted = tuple_[4]
            list_of_tree_nodes.append(node)
        self.tuple_of_tree_nodes = (list_of_tree_nodes)

        # create a nice dict
        # for node in self.tuple_of_tree_nodes:
        #     self._dict_node_by_sheet_id[node.sheet_id] = node
            #self._dict_sheet_id_by_sort_order[tuple_[2]] = tuple_[0]


        self.create_child_nodes(self.root_node)

        self.endResetModel()

    # def apply_node_variables_from_dict(self, node, sheet_id):
    #     """
    #
    #     """
    #     if sheet_id == -1:
    #         return
    #
    #     tuple_ = self._dict_node_by_sheet_id[sheet_id]
    #     node.sheet_id, node.title, node.sort_order, node.indent, node.deleted = tuple_
    #
    #     self._dict_sheet_id_by_sort_order[]
    #     node.parent_id = self.find_parent_id_from_child_sheet_id(sheet_id)
    #
    #     , node.children_id

    def find_parent_id_from_child_sheet_id(self, child_sheet_id):
        # find child
        parent_node = None
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
            parent_node.appendChild(child_node)
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
            
        
        

class TreeNode(object):
    def __init__(self, parent=None):
        super(TreeNode, self).__init__()
  
        self.title = "title"
        self.sheet_id = None
        self.parent_id = None
        self.children_id = None
        self.properties = None

        self.indent = None
        self.sort_order = None
        self.deleted = None
       
        self.parent = parent
        self.children = []
       
        self.setParent(parent)
       
    def setParent(self, parent):
        if parent != None:
            self.parent = parent
            self.parent.appendChild(self)
        else:
            self.parent = None
           
    def appendChild(self, child):
        if not child in self.children:
            self.children.append(child)
       
    def childAtRow(self, row):
        return self.children[row]
   
    def rowOfChild(self, child):       
        for i, item in enumerate(self.children):
            if item == child:
                return i
        return -1
   
    def removeChild(self, row):
        value = self.children[row]
        self.children.remove(value)

        return True
       
    def __len__(self):
        return len(self.children)





  
