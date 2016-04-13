'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt
from .. import cfg
from .tree_model import TreeModel
from ..paper_manager import SheetPaper


class SheetTreeModel(TreeModel):
    '''
    classdocs
    '''

    def __init__(self, parent, project_id):
        super(SheetTreeModel, self).__init__("tbl_sheet", "l_sheet_id", parent, project_id)

        '''
        Constructor
        '''


        self.headers = ["name"]
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.title_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.tree_structure_modified")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.properties")

    def data(self, index, role):
        
        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.node_from_index(index)

        if (role == Qt.DisplayRole or role == Qt.EditRole) and col == 0:
            return self.data(index, self.TitleRole)

        return TreeModel.data(self, index, role)

    
    

#------------------------------------------
#------------------Editing-----------------
#------------------------------------------

    def setData(self, index, value, role):
        
        limit = [role]
        
        # title :
        if index.isValid() & role == Qt.EditRole & index.column() == 0:

            node = self.node_from_index(index)

            sheet = SheetPaper(node.id)
            sheet.title = value

            self.dataChanged.emit(index, index, limit)
            # cfg.data_subscriber.announce_update(self._project_id, "sheet.title_changed", node.id)
            return True
        
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



    #
    #
    # def create_child_nodes(self, parent_node):
    #
    #     # self.apply_node_variables_from_dict(parent_node, parent_node.sheet_id, self._dict)
    #
    #     list_of_direct_child_nodes = []
    #     for node in self.tuple_of_tree_nodes:
    #         if node.sort_order > parent_node.sort_order and node.indent <= parent_node.indent:
    #             break
    #         if node.sort_order > parent_node.sort_order and node.indent is parent_node.indent + 1:
    #             list_of_direct_child_nodes.append(node)
    #
    #
    #     for child_node in list_of_direct_child_nodes:
    #         parent_node.append_child(child_node)
    #         child_node.setParent(parent_node)
    #         self.create_child_nodes(child_node)




    '''
        
    def get_synopsys(self,sheet_id):
        pass
    
    def set_synopsys(self,sheet_id, content):
        pass
    
    def get_notes(self,sheet_id):
        pass
    
    def set_notes(self,sheet_id, content):
        pass

    '''
