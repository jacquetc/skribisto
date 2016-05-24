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

    def __init__(self, table_name: str, property_type: str, code_name: str, property_id):
        super(Property, self).__init__()

        self._table_name = table_name
        self._property_type = property_type
        self._code_name = code_name
        self.id = property_id


        # self._subscribe_to_data()

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

    def add(self, paper_id: int):
        new_list = cfg.data.database.get_table(self._table_name).add_new_properties(1, paper_id)
        cfg.data_subscriber.announce_update(0, self._property_type + ".structure_changed")
        return new_list[0]

    def remove(self):
        if cfg.data.database.get_table(self._table_name).remove_properties([self.id]):
            cfg.data_subscriber.announce_update(0, self._property_type + ".structure_changed")
            return True
        else:
            return False

    def _find_id(self, paper_id:int, name:str):
        """

        :param paper_id:
        :param name:
        :return: return None if nothing or id
        """
        return cfg.data.database.get_table(self._table_name)



class SheetProperty(Property):

    def __init__(self, property_id: int):
        super(SheetProperty, self).__init__(table_name="tbl_sheet_property", code_name="l_sheet_code"
                                            , property_type="sheet_property", property_id=property_id)

    def set_property(paper_id: int, name: str, value: str):
        property_id = SheetProperty(-1)._find_id(paper_id, name)
        if property_id is not None:
            property_id = SheetProperty(-1).add(paper_id)
        sheet_property = SheetProperty(property_id)
        sheet_property.name = name
        sheet_property.value = value


    def get_property(paper_id: int, name: str, default_value: str):
        property_id = SheetProperty(-1)._find_id(paper_id, name)
        if property_id is None:
            property_id = SheetProperty(-1).add(paper_id)
            sheet_property = SheetProperty(property_id)
            sheet_property.value = default_value

        return  SheetProperty(property_id).value



class NoteProperty(Property):

    def __init__(self, property_id: int):
        super(NoteProperty, self).__init__(table_name="tbl_note_property", code_name="l_note_code"
                                           , property_type="note_property", property_id=property_id)
    def set_property(paper_id: int, name: str, value: str):
        property_id = NoteProperty(-1)._find_id(paper_id, name)
        if property_id is not None:
            property_id = NoteProperty(-1).add(paper_id)
        sheet_property = NoteProperty(property_id)
        sheet_property.name = name
        sheet_property.value = value


    def get_property(paper_id: int, name: str, default_value: str):
        property_id = NoteProperty(-1)._find_id(paper_id, name)
        if property_id is None:
            property_id = NoteProperty(-1).add(paper_id)
            sheet_property = NoteProperty(property_id)
            sheet_property.value = default_value

        return  NoteProperty(property_id).value
