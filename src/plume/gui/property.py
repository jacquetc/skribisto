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

    def __init__(self, table_name: str, property_type: str, property_id):
        super(Property, self).__init__()

        self._table_name = table_name
        self._property_type = property_type
        self.id = property_id

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
        return cfg.data.database.get_table(self._table_name).get_name(self.id)

    @name.setter
    def name(self, value: str):
        cfg.data.database.get_table(self._table_name).set_name(self.id, value)
        cfg.data_subscriber.announce_update(0, self._property_type + ".name_changed", self.id)

    @property
    def value(self):
        val = cfg.data.database.get_table(self._table_name).get_value(self.id)
        return val

    @value.setter
    def value(self, val):
        cfg.data.database.get_table(self._table_name).set_value(self.id, val)
        cfg.data_subscriber.announce_update(0, self._property_type + ".value_changed", self.id)

    @property
    def paper_code(self):
        val = cfg.data.database.get_table(self._table_name).get_paper_code(self.id)
        return val

    @paper_code.setter
    def paper_code(self, val):
        cfg.data.database.get_table(self._table_name).set_paper_code(self.id, val)
        cfg.data_subscriber.announce_update(0, self._property_type + ".paper_code_changed", self.id)


    @property
    def creation_date(self):
        return cfg.data.database.get_table(self._table_name).get_creation_date(self.id)

    @creation_date.setter
    def creation_date(self, value):
        cfg.data.database.get_table(self._table_name).set_creation_date(self.id, value)
        cfg.data_subscriber.announce_update(0, self._property_type + ".creation_date_changed", self.id)

    @property
    def last_modification_date(self):
        return cfg.data.database.get_table(self._table_name).get_modification_date(self.id)

    @last_modification_date.setter
    def last_modification_date(self, value):
        cfg.data.database.get_table(self._table_name).set_modification_date(self.id, value)
        cfg.data_subscriber.announce_update(0, self._property_type + ".last_modification_changed", self.id)

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
        new_list = cfg.data.database.get_table(self._table_name).add_new_properties(1, paper_id, imposed_property_ids)
        cfg.data_subscriber.announce_update(0, self._property_type + ".structure_changed")
        return new_list

    def remove(self, property_ids: list = ()):
        if property_ids == ():
            property_ids = [self.id]
        if cfg.data.database.get_table(self._table_name).remove_properties(property_ids):
            cfg.data_subscriber.announce_update(0, self._property_type + ".structure_changed")
            return True
        else:
            return False




class SheetProperty(Property):

    def __init__(self, property_id: int = -1):
        super(SheetProperty, self).__init__(table_name="tbl_sheet_property", property_type="sheet_property"
                                            , property_id=property_id)



class NoteProperty(Property):

    def __init__(self, property_id: int = -1):
        super(NoteProperty, self).__init__(table_name="tbl_note_property", property_type="note_property"
                                            , property_id=property_id)
