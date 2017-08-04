'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt
from .. import cfg
from .tree_model import TreeModel, TreeItem
from ..paper_manager import SheetPaper



class SheetTreeModel(TreeModel):
    '''
    classdocs
    '''

    def __init__(self, parent):
        super(SheetTreeModel, self).__init__("l_sheet_id", "sheet", parent)

        '''
        Constructor
        '''


        self.headers = ["name"]

        cfg.data.projectHub().projectLoaded.connect(self.reset_model)
        # TODO : add signal connections

        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.title_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.deleted_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.structure_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet.properties")

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
        #
        # # title :
        # if index.isValid() & role == Qt.EditRole & index.column() == 0:
        #
        #     node = self.node_from_index(index)
        #
        #     sheet = SheetPaper(node.id)
        #     sheet.title = value
        #
        #     self.dataChanged.emit(index, index, limit)
        #     # cfg.data_subscriber.announce_update(self._project_id, "sheet.title_changed", node.id)
        #     return True
        
        return TreeModel.setData(self, index, value, role)

    def reset_model(self):
        self.beginResetModel()
        #
        # self._all_data = cfg.data.get_database(0).get_tree(self._table_name).get_all()
        # all_headers = cfg.data.get_database(0).get_tree(self._table_name).get_all_headers()
        # header_dict = {}
        # for header in all_headers:
        #     header_dict[header] = QVariant()

        del self._root_node
        self._root_node = SheetTreeItem()
        self._root_node.id = -1
        self._root_node.indent = -1
        self._root_node.sort_order = -1

        self._node_list = []
        project_id_list = cfg.data.projectHub().getProjectIdList()
        for project_id in project_id_list:
            sheet_hub = cfg.data.sheetHub()
            self._id_list = sheet_hub.getAllIds(project_id)
            self._title_dict = sheet_hub.getAllTitles(project_id)
            self._indent_dict = sheet_hub.getAllIndents(project_id)
            self._sort_order_dict = sheet_hub.getAllSortOrders(project_id)
            # create project name items
            if len(project_id_list) > 1:
                item = SheetTreeItem(self._root_node)
                item.indent = 0
                item.indent_drift = 1
                item.id = 0
                item.project_id = project_id
                item.title = "project " + str(project_id)
                self._node_list.append(item)
                self._root_node.append_child(item)
                self._populate_item(item, project_id)
            else:
                self._populate_item(self._root_node, project_id)


        #
        # if self._all_data is []: # empty /close
        #     self.endResetModel()
        #     return


        self.endResetModel()

    def _populate_item(self, parent_item: TreeItem, project_id: int):
        while self._id_list:
            indent = self._indent_dict[self._id_list[0]]

            if parent_item.indent < indent:
                item = SheetTreeItem(parent_item)
                _id = self._id_list.pop(0)
                item.id = _id
                item.project_id = project_id
                item.indent_drift = parent_item.indent_drift
                item.indent = self._indent_dict[_id]
                item.sort_order = self._sort_order_dict[_id]
                item.title = self._title_dict[_id]
                self._node_list.append(item)
                parent_item.append_child(item)
                self._populate_item(item, project_id)
            else:
                return

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


class SheetTreeItem(TreeItem):

    def __init__(self, parent=None):
        super(SheetTreeItem, self).__init__(parent)
        self.char_count = 0
        self.word_count = 0
