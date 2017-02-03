'''
Created on 22 may 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import Qt
from .property_model import PropertyModel, ListItem
from .. import cfg
from PyQt5.QtCore import QObject

class SheetPropertyModel(PropertyModel):
    '''
    classdocs
    '''

    def __init__(self, parent: QObject):
        super(SheetPropertyModel, self).__init__("tbl_sheet_property"
                                                 , "l_property_id", "l_sheet_code"
                                                 , "sheet_property", parent)

        '''
        Constructor
        '''

        cfg.data.projectHub().projectLoaded.connect(self.reset_model)
        cfg.data.sheetPropertyHub().propertyChanged.connect(self.reset_model)
        cfg.data.sheetPropertyHub().propertyAdded.connect(self.reset_model)
        cfg.data.sheetPropertyHub().propertyRemoved.connect(self.reset_model)

        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.name_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.value_changed")
        # # TODO : line useful ?
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.sheet_code_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.structure_changed")

    def reset_model(self):
        self.beginResetModel()


        del self._root_node
        self._root_node = ListItem()
        self._root_node.id = -1


        self._node_list = []
        project_id_list = cfg.data.projectHub().getProjectIdList()

        for project_id in project_id_list:
            sheet_property_hub = cfg.data.sheetPropertyHub()
            self._id_list = sheet_property_hub.getAllIds(project_id)
            self._name_dict = sheet_property_hub.getAllNames(project_id)
            self._value_dict = sheet_property_hub.getAllValues(project_id)
            self._is_system_dict = sheet_property_hub.getAllIsSystems(project_id)
            self._paper_code_dict = sheet_property_hub.getAllPaperCodes(project_id)

            self._populate_item(self._root_node, project_id)

        self.endResetModel()

    def _populate_item(self, parent_item: ListItem, project_id: int):
        while self._id_list:
            item = ListItem(parent_item)
            _id = self._id_list.pop(0)
            item.id = _id
            item.project_id = project_id
            item.name = self._name_dict[_id]
            item.value = self._value_dict[_id]
            item.is_system = self._is_system_dict[_id]
            item.paper_code = self._paper_code_dict[_id]
            self._node_list.append(item)
            parent_item.append_child(item)
