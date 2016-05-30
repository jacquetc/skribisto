# -*- coding: utf-8 -*-
'''
Created on 12 mars 2016

@author: Cyril Jacquet
'''

from . import cfg

class PaperManager:

    def __init__(self):
        super(PaperManager, self).__init__()

    def clear(self):
        pass


class Paper:

    def __init__(self, table_name: str, paper_type: str, paper_id):
        super(Paper, self).__init__()

        self._table_name = table_name
        self._paper_type = paper_type
        self.id = paper_id
        self._title = _("unnamed")
        self._content = None
        self._content_type = ""
        self._creation_date = ""
        self._last_modification_date = ""
        self._properties = {}
        self.parent_id = None
        self.children_id = []
        self._version = -1

        # self._subscribe_to_data()

    @property
    def title(self):
        return cfg.data.database.get_tree(self._table_name).get_title(self.id)

    @title.setter
    def title(self, value: str):
        cfg.data.database.get_tree(self._table_name).set_title(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".title_changed", self.id)

    @property
    def content(self):
        value = cfg.data.database.get_tree(self._table_name).get_content(self.id)
        return value

    @content.setter
    def content(self, value):
        cfg.data.database.get_tree(self._table_name).set_content(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".content_changed", self.id)

    @property
    def creation_date(self):
        return cfg.data.database.get_tree(self._table_name).get_creation_date(self.id)

    @creation_date.setter
    def creation_date(self, value):
        cfg.data.database.get_tree(self._table_name).set_creation_date(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".creation_date_changed", self.id)

    @property
    def last_modification_date(self):
        return cfg.data.database.get_tree(self._table_name).get_modification_date(self.id)

    @last_modification_date.setter
    def last_modification_date(self, value):
        cfg.data.database.get_tree(self._table_name).set_modification_date(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".last_modification_changed", self.id)

    @property
    def deleted(self):
        return cfg.data.database.get_tree(self._table_name).get_deleted(self.id)

    @deleted.setter
    def deleted(self, value: bool):
        cfg.data.database.get_tree(self._table_name).set_deleted(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".structure_changed", self.id)


    # @property
    # def properties(self):
    #     return cfg.data.database.get_tree(self._table_name).get_properties(self.id)
    #
    # @properties.setter
    # def properties(self, value: dict):
    #     cfg.data.database.get_tree(self._table_name).set_properties(self.id, value)
    #     cfg.data_subscriber.announce_update(0, self._paper_type + ".properties_changed", self.id)

    @property
    def version(self):
        return cfg.data.database.get_tree(self._table_name).get_version(self.id)

    @version.setter
    def version(self, value: int):
        cfg.data.database.get_tree(self._table_name).set_version(self.id, value)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".version_changed", self.id)

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



    def add_paper_after(self, new_child_ids: list =()):
        new_list = cfg.data.database.get_tree(self._table_name).add_new_papers_by(self.id, 1, new_child_ids)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".structure_changed")
        return new_list

    def add_child_paper(self, new_child_ids: list =()):
        db = cfg.data.database
        new_list = db.get_tree(self._table_name).add_new_child_papers(self.id, 1, new_child_ids)
        cfg.data_subscriber.announce_update(0, self._paper_type + ".structure_changed")
        return new_list

    def remove_paper(self):
        cfg.data.database.get_tree(self._table_name).remove_papers([self.id])
        cfg.data_subscriber.announce_update(0, self._paper_type + ".structure_changed")



class SheetPaper(Paper):

    def __init__(self, sheet_id):
        super(SheetPaper, self).__init__(table_name="tbl_sheet", paper_type="sheet", paper_id=sheet_id)


class NotePaper(Paper):

    def __init__(self, note_id):
        super(NotePaper, self).__init__(table_name="tbl_note", paper_type="note", paper_id=note_id)
