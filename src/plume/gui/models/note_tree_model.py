'''
Created on 13 avr. 2016

@author:  Cyril Jacquet
'''
from PyQt5.QtCore import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt
from .. import cfg
from .tree_model import TreeModel
from ..paper_manager import NotePaper


class NoteTreeModel(TreeModel):
    '''
    classdocs
    '''

    def __init__(self, parent, project_id):
        super(NoteTreeModel, self).__init__("tbl_note", "l_note_id", parent, project_id)

        '''
        Constructor
        '''

        self.headers = ["name"]
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.title_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.tree_structure_modified")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note.properties")

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

            sheet = NotePaper(node.id)
            sheet.title = value

            self.dataChanged.emit(index, index, limit)
            # cfg.data_subscriber.announce_update(self._project_id, "sheet.title_changed", node.id)
            return True

        return TreeModel.setData(self, index, value, role)

