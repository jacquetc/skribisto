'''
Created on 22 may 2016

@author:  Cyril Jacquet
'''


from PyQt5.QtCore import Qt, QObject
from .property_model import PropertyModel
from .. import cfg


class NotePropertyModel(PropertyModel):
    '''
    classdocs
    '''

    def __init__(self, parent: QObject, project_id: int):
        super(NotePropertyModel, self).__init__("tbl_note_property", "l_property_id", "l_note_code"
                                                , "note_property", parent, project_id)

        '''
        Constructor
        '''
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.name_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.value_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.note_code_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.structure_changed")
