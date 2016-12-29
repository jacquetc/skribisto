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

    def __init__(self, paper_type: str, project_id: int, paper_id: int):
        super(Paper, self).__init__()

        self.paper_type = paper_type
        self.project_id = project_id
        self.id = paper_id

        if paper_type == "sheet":
            self.hub = cfg.data.sheetHub()
        if paper_type == "note":
            self.hub = cfg.data.noteHub()

    @property
    def title(self):
        return self.hub.getTitle(self.project_id, self.id)

    @title.setter
    def title(self, value: str):
        self.hub.setTitle(self.project_id, self.id, value)

    @property
    def content(self):
        return self.hub.getContent(self.project_id, self.id)

    @content.setter
    def content(self, value):
        self.hub.setContent(self.project_id, self.id, value)

    @property
    def creation_date(self):
        return self.hub.getCreationDate(self.project_id, self.id)

    @creation_date.setter
    def creation_date(self, value):
        self.hub.setCreationDate(self.project_id, self.id, value)

    @property
    def last_modification_date(self):
        return self.hub.getUpdateDate(self.project_id, self.id)

    @last_modification_date.setter
    def last_modification_date(self, value):
        self.hub.setUpdateDate(self.project_id, self.id, value)

    @property
    def deleted(self):
        return self.hub.getDeleted(self.project_id, self.id)

    @deleted.setter
    def deleted(self, value: bool):
        self.hub.setDeleted(self.project_id, self.id, value)



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
        return self.hub.getVersion(self.project_id, self.id)

    @version.setter
    def version(self, value: int):
        self.hub.setVersion(self.project_id, self.id, value)


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

    def __init__(self, project_id: int, sheet_id: int):
        super(SheetPaper, self).__init__(paper_type="sheet", project_id=project_id, paper_id=sheet_id)


class NotePaper(Paper):

    def __init__(self, project_id: int, note_id: int):
        super(NotePaper, self).__init__(paper_type="note", project_id=project_id, paper_id=note_id)
