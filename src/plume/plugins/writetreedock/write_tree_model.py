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
        cfg.data.subscriber.subscribe_update_func_to_domain(self.reset_model, "data.project.close")
        cfg.data.subscriber.subscribe_update_func_to_domain(self.reset_model, "data.tree")
        cfg.data.subscriber.subscribe_update_func_to_domain(self.reset_model, "data.tree.title")
        cfg.data.subscriber.subscribe_update_func_to_domain(self.reset_model, "data.tree.properties")

    def columnCount(self, parent):
        return 1


    def rowCount(self, parent):
        
        if parent.column() > 0:
            return 0

        if not parent.isValid():
            parent_node = self.root_node
            
            return len(parent_node)
        else:
            parent_node = self.nodeFromIndex(parent)
            return len(parent_node)
        
        


    def headerData(self, section, orientation, role):
        if orientation == Qt.Horizontal and role == Qt.DisplayRole:
            return QVariant(self.headers[section])
        return QVariant()

    def index(self, row, column, parent):
        if not self.hasIndex(row, column, parent):
            return QModelIndex()

        node = self.nodeFromIndex(parent)
        return self.createIndex(row, column, node.childAtRow(row))


    def data(self, index, role):
        
        row = index.row()
        col = index.column() 
        
               
        if not index.isValid():
            return QVariant()
        

                   
        node = self.nodeFromIndex(index)
       
        
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

        node = self.nodeFromIndex(child)
       
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
    
    
    def nodeFromIndex(self, index):
        return index.internalPointer() if index.isValid() else self.root_node
   
#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):
        
        limit = [role]
        
        # title :
        if index.isValid() & role == Qt.EditRole & index.column() == 0:
            
            node = self.nodeFromIndex(index) 
            
            cfg.data.main_tree.set_title(node.sheet_id, value)
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
        node = self.nodeFromIndex(index[0])       
        mimeData = PyMimeData(node)
        return mimeData


    def dropMimeData(self, mimedata, action, row, column, parentIndex):
        if action == Qt.IgnoreAction:
            return True

        dragNode = mimedata.instance()
        parentNode = self.nodeFromIndex(parentIndex)

        # make an copy of the node being moved
        newNode = deepcopy(dragNode)
        newNode.setParent(parentNode)
        self.insertRow(len(parentNode)-1, parentIndex)
        self.emit(SIGNAL("dataChanged(QModelIndex,QModelIndex)"), 
parentIndex, parentIndex)
        return True

    '''
    def insertRow(self, row, parent):
        return self.insertRows(row, 1, parent)


    def insertRows(self, row, count, parent):
        self.beginInsertRows(parent, row, (row + (count - 1)))
        for _ in range(0, count):
            self._id_of_last_created_sheet = cfg.data.main_tree.create_new_sheet(self.nodeFromIndex(parent).sheet_id, "write")
        self.endInsertRows()
        return True


    def removeRow(self, row, parentIndex):
        return self.removeRows(row, 1, parentIndex)


    def removeRows(self, row, count, parentIndex):
        self.beginRemoveRows(parentIndex, row, row)
        node = self.nodeFromIndex(parentIndex)
        node.removeChild(row)
        self.endRemoveRows()
       
        return True



    @property
    def id_of_last_created_sheet(self):       
        return self._id_of_last_created_sheet

    def find_index_from_id(self, id):
        start_index = self.index(0,0, QModelIndex())
        index_list = self.match(start_index, 37, id, 1, Qt.MatchExactly | Qt.MatchRecursive)
        if len(index_list) == 0:
            return None
        else:
            return index_list[0]
    
    def reset_model(self):
        self.beginResetModel()
        

        del self.root_node
        self.root_node = TreeNode()
          
        
        list_ = cfg.data.main_tree.get_tree_model_necessities("write")
        if list_ == []: #empty /close
            self.root_node = TreeNode()
            self.endResetModel()
            return           
        
        # create a nice dict
        self._dict = {}
        for tuple_ in list_:
            self._dict[tuple_[0]] = tuple_

        self.root_node.sheet_id = cfg.data.main_tree.get_root_id("write")
        self.create_child_nodes(self.root_node)


        self.endResetModel()

    def apply_node_variables_from_dict(self, node, sheet_id, dict_):
        """
        
        """
        tuple_ = self._dict[sheet_id]
        node.sheet_id, node.title, node.parent_id, node.children_id, node.properties = tuple_
        


    def create_child_nodes(self, parent_node):
        
        
        
        #add & scan children :
        
        self.apply_node_variables_from_dict(parent_node, parent_node.sheet_id, self._dict)


        if parent_node.children_id != None:
            for child_id in parent_node.children_id:
            

                child_node = TreeNode(parent_node) 
                child_node.sheet_id = child_id  
                parent_node.appendChild(child_node)
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





  
