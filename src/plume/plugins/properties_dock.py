'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''
from core import plugins as core_plugins
from gui import plugins as gui_plugins

from core import cfg as core_cfg


class PropertiesDockPlugin(core_plugins.CoreStoryDockPlugin, gui_plugins.GuiStoryDockPlugin):
    '''
    PropertiesDockPlugin
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(PropertiesDockPlugin, self).__init__()


    def print_name(self):
        print("PropertiesDockPlugin")
        
    def core_class(self):        
        return CorePropertyDock
    
    def gui_class(self):
        return GuiPropertyDock
    
class CorePropertyDock():
    '''
    CorePropertyDock
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(CorePropertyDock, self).__init__()
        self._property_table_model = None
        self._sheet_id = None
        self.dock_name = "properties-dock" 
   
    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self._property_table_model.set_sheet_id(sheet_id)
        
    @property   
    def property_table_model(self):
        if self._property_table_model is None:
            self._property_table_model = PropertyTableModel(self)  
            if self._sheet_id is not None:
                self._property_table_model.set_sheet_id(self._sheet_id)
            
            
        return self._property_table_model


from PyQt5.QtWidgets import QTableView
from gui import cfg as gui_cfg

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
        self.tableView = None
        self.core_property_dock = None           
        self._sheet_id = None
        

    @property
    def sheet_id(self):
        return self._sheet_id
    
    @sheet_id.setter
    def sheet_id(self, sheet_id):
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self.core_property_dock.sheet_id = sheet_id
         
        
    def get_widget(self):
        
        if self.tableView is None:
            self.tableView = QTableView()
            self.core_property_dock = gui_cfg.core.plugins.CorePropertyDock()
            table_model = self.core_property_dock.property_table_model
            self.tableView.setModel(table_model)
            self.tableView.gui_part = self
        return self.tableView
    

       
       
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
        
        self.root_node = TableNode()        
        self.headers = ["property", "value"]
        
    def set_sheet_id(self, sheet_id):
        self._sheet_id = sheet_id
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
        #if parent.column() > 0:
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

        node = self.nodeFromIndex(parent)
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
        

                   
        node = self.nodeFromIndex(index)
       
        
        if col == 0 and (role == Qt.DisplayRole or role == Qt.EditRole) :
            return node.key
        if col == 1 and (role == Qt.DisplayRole or role == Qt.EditRole) :
            return node.value
        

        return QVariant();
        

    def parent(self, child):
        '''
        function:: parent(child)
        :param child:
        '''
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
        '''
        function:: nodeFromIndex(index)
        :param index:
        '''
        return index.internalPointer() if index.isValid() else self.root_node
        
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
        
        # title :
        if index.isValid() & role == Qt.EditRole & index.column() == 0:
            
            node = self.nodeFromIndex(index) 
            
            #cfg.data.story_tree.rename(node.sheet_id, value)
            node.key = value
            
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
        self.beginInsertRows(parent, row, (row + (count - 1)))
        self.endInsertRows()
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
        self.beginRemoveRows(parentIndex, row, row)
        node = self.nodeFromIndex(parentIndex)
        node.removeChild(row)
        self.endRemoveRows()
       
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
        self.prop_dict = core_cfg.data.main_tree.get_properties(self._sheet_id)
        
        self.root_node.sheet_id = self._sheet_id
        self.create_child_nodes(self.root_node)


        self.endResetModel()
        

    def apply_node_variables_from_dict(self, node, sheet_id, dict_):
        '''
        function:: apply_node_variables_from_dict(node, sheet_id, dict_)
        :param node:
        :param sheet_id:
        :param dict_:
        '''
        pass
  
        

    def create_child_nodes(self, parent_node):
        '''
        function:: create_child_nodes(parent_node)
        :param parent_node:
        :rtype:  

        '''
        #add & scan children :
        

        if list(self.prop_dict.keys()) != []:
            for child_key in self.prop_dict.keys():
            

                child_node = TableNode(parent_node) 
                child_node.key = child_key
                child_node.value = self.prop_dict[child_key]
                child_node.sheet_id = self._sheet_id
                child_node.children_id = None                
                parent_node.appendChild(child_node)
                #self.create_child_nodes(child_node)


        
    
    
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
            self.parent.appendChild(self)
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


