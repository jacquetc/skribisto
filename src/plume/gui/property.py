# -*- coding: utf-8 -*-
'''
Created on 12 mars 2016

@author: Cyril Jacquet
'''

from . import cfg
# 
# class PaperManager:
# 
#     def __init__(self):
#         super(PaperManager, self).__init__()
# 
#     def clear(self):
#         pass


class Property:

    def __init__(self, property_type: str, project_id: int, property_id: int):
        super(Property, self).__init__()

        self._property_type = property_type
        self.id = property_id
        self.project_id = project_id

        if property_type == "sheet_property":
            self.hub = cfg.data.sheetPropertyHub()
        if property_type == "note_property":
            self.hub = cfg.data.notePropertyHub()

        # for deep copy :
        self._name = None
        self._value = None
        self._creation_date = None
        self._last_modification_date = None
        self._system = None

    def copy_deeply(self):
        """
        reapply all data to database. It does not reapply paper_code.
        """
        self._name = self.name
        self._value = self.value
        self._creation_date = self.creation_date
        self._last_modification_date = self.last_modification_date
        self._system = self.system

    def paste_deeply(self):
        """
        save all data from database. It does not save paper_code.
        """
        # self._subscribe_to_data()
        self.name = self._name
        self.value = self._value
        self.creation_date = self._creation_date
        self.last_modification_date = self._last_modification_date
        self.system = self._system


    @property
    def name(self):
        return self.hub.getName(self.project_id, self.id)

    @name.setter
    def name(self, value: str):
        self.hub.setName(self.project_id, self.id, value)


    @property
    def value(self):
        val = self.hub.getPropertyById(self.project_id, self.id)
        return val

    @value.setter
    def value(self, val):
        self.hub.setValue(self.project_id, self.id, val)

    @property
    def paper_code(self):
        return self.hub.getPaperCode(self.project_id, self.id)

    @paper_code.setter
    def paper_code(self, val):
        self.hub.setPaperCode(self.project_id, self.id, val)

    @property
    def creation_date(self):
        return self.hub.getCreationDate(self.project_id, self.id)

    @creation_date.setter
    def creation_date(self, value):
        self.hub.setCreationDate(self.project_id, self.id, value)

    @property
    def last_modification_date(self):
        return self.hub.getModificationDate(self.project_id, self.id)

    @last_modification_date.setter
    def last_modification_date(self, value):
        self.hub.setModificationDate(self.project_id, self.id, value)

    @property
    def system(self):
        return self.hub.getSystem(self.project_id, self.id)

    @system.setter
    def system(self, value):
        self.hub.setSystem(self.project_id, self.id, value)

    # @property
    # def properties(self):
    #     return cfg.data.database.get_table(self._table_name).get_properties(self.id)
    #
    # @properties.setter
    # def properties(self, value: dict):
    #     cfg.data.database.get_table(self._table_name).set_properties(self.id, value)
    #     cfg.data_subscriber.announce_update(0, self._property_type + ".properties_changed", self.id)
    # 
    # @property
    # def version(self):
    #     return cfg.data.database.get_table(self._table_name).get_version(self.id)
    # 
    # @version.setter
    # def version(self, value: int):
    #     cfg.data.database.get_table(self._table_name).set_version(self.id, value)
    #     cfg.data_subscriber.announce_update(0, self._property_type + ".version_changed", self.id)

    # def _subscribe_to_data(self,  is_subscribing=True):
    #
    #     list_ = [[self.get_title, "data.sheet_tree.title"],
    #              [self.get_content, "data.sheet_tree.content"],
    #              [self.get_content_type, "data.sheet_tree.content_type"],
    #              [self.get_properties, "data.sheet_tree.properties"],
    #              [self.get_modification_date, "data.sheet_tree.modification_date"],
    #              [self.get_creation_date, "data.sheet_tree.creation_date"],
    #              [self.get_properties, "data.sheet_tree.properties"],
    #              [self.get_version, "data.sheet_tree.version"],
    #              ]
    #     for func, domain in list_:
    #         if is_subscribing is True:
    #             cfg.data.subscriber.subscribe_update_func_to_domain(
    #                 0, func, domain)
    #         else:
    #             cfg.data.subscriber.unsubscribe_update_func_to_domain(0, func)

    def add(self, paper_id: int, imposed_property_ids: list = ()):
        hub = cfg.data.sheetPropertyHub()
        new_id = hub.getLastAddedId()
        self.id = new_id
        if imposed_property_ids:
            hub.addProperty(self.project_id, paper_id, imposed_property_ids[0])
        else:
            hub.addProperty(self.project_id, paper_id)

        return [new_id]

    def remove(self, property_ids: list = ()):
        if not property_ids:
            property_ids = [self.id]
        if self.hub.removeProperty(self.project_id, property_ids[0]):
            return True
        else:
            return False


class SheetProperty(Property):
    def __init__(self, project_id: int, property_id: int = -1):
        super(SheetProperty, self).__init__(property_type="sheet_property"
                                            , project_id=project_id, property_id=property_id)



class NoteProperty(Property):

    def __init__(self, project_id: int, property_id: int = -1):
        super(NoteProperty, self).__init__(property_type="note_property"
                                            , project_id=project_id, property_id=property_id)
