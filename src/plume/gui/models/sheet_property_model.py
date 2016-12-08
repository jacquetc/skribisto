'''
Created on 22 may 2016

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import Qt
from .property_model import PropertyModel
from .. import cfg
from PyQt5.QtCore import QObject

class SheetPropertyModel(PropertyModel):
    '''
    classdocs
    '''

    def __init__(self, parent: QObject, project_id: int):
        super(SheetPropertyModel, self).__init__("tbl_sheet_property"
                                                 , "l_property_id", "l_sheet_code"
                                                 , "sheet_property", parent, project_id)

        '''
        Constructor
        '''
        cfg.data.projectHub().projectClosed.connect(self.clear_from_project)
        cfg.data.projectHub().allProjectsClosed.connect(self.clear_from_all_projects)

        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.name_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.value_changed")
        # # TODO : line useful ?
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.sheet_code_changed")
        # cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "sheet_property.structure_changed")

