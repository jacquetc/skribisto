from PyQt5.QtCore import Qt
from .property_model import PropertyModel
from .. import cfg


class NotePropertyModel(PropertyModel):
    '''
    classdocs
    '''
    SheetCodeRole = Qt.UserRole + 20

    def __init__(self, parent, project_id):
        super(NotePropertyModel, self).__init__("tbl_note_property", "l_property_id", "l_note_code", parent, project_id)

        '''
        Constructor
        '''
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.clear, "database_closed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "database_loaded")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.name_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.value_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.note_code_changed")
        cfg.data.subscriber.subscribe_update_func_to_domain(project_id, self.reset_model, "note_property.list_structure_modified")
