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
        self.paper_id = paper_id

        if paper_type == "sheet":
            self.hub = cfg.data.sheetHub()
        if paper_type == "note":
            self.hub = cfg.data.noteHub()

    @property
    def title(self):
        return self.hub.getTitle(self.project_id, self.paper_id)

    @title.setter
    def title(self, value: str):
        self.hub.setTitle(self.project_id, self.paper_id, value)

    @property
    def content(self):
        return self.hub.getContent(self.project_id, self.paper_id)

    @content.setter
    def content(self, value):
        self.hub.setContent(self.project_id, self.paper_id, value)

    @property
    def creation_date(self):
        return self.hub.getCreationDate(self.project_id, self.paper_id)

    @creation_date.setter
    def creation_date(self, value):
        self.hub.setCreationDate(self.project_id, self.paper_id, value)

    @property
    def last_modification_date(self):
        return self.hub.getUpdateDate(self.project_id, self.paper_id)

    @last_modification_date.setter
    def last_modification_date(self, value):
        self.hub.setUpdateDate(self.project_id, self.paper_id, value)

    @property
    def deleted(self):
        return self.hub.getDeleted(self.project_id, self.paper_id)

    @deleted.setter
    def deleted(self, value: bool):
        self.hub.setDeleted(self.project_id, self.paper_id, value)



    # @property
    # def properties(self):
    #     return cfg.data.database.get_tree(self._table_name).get_properties(self.paper_id)
    #
    # @properties.setter
    # def properties(self, value: dict):
    #     cfg.data.database.get_tree(self._table_name).set_properties(self.paper_id, value)
    #     cfg.data_subscriber.announce_update(0, self._paper_type + ".properties_changed", self.paper_id)

    @property
    def version(self):
        return self.hub.getVersion(self.project_id, self.paper_id)

    @version.setter
    def version(self, value: int):
        self.hub.setVersion(self.project_id, self.paper_id, value)


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

    def add_paper_after(self, new_ids: list =()):
        error = self.hub.addPaperBelow(self.project_id, self.paper_id)
        if error.isSuccess() and new_ids:
            error = self.hub.setId(self.project_id, self.hub.getLastAddedId(), new_ids[0])
        new_list = []
        if not error.isSuccess():
            return new_list
        if new_ids:
            new_list = new_ids
        else:
            new_list.append(self.hub.getLastAddedId())
        return new_list

    def add_child_paper(self, new_child_ids: list =()):
        error = self.hub.addChildPaper(self.project_id, self.paper_id)
        if error.isSuccess() and new_child_ids:
            error = self.hub.setId(self.project_id, self.hub.getLastAddedId(), new_child_ids[0])
        new_list = []
        if not error.isSuccess():
            return new_list
        if new_child_ids:
            new_list = new_child_ids
        else:
            new_list.append(self.hub.getLastAddedId())
        return new_list

    def remove_paper(self):
        self.hub.removePaper(self.project_id, self.paper_id)


class SheetPaper(Paper):

    def __init__(self, project_id: int, sheet_id: int):
        super(SheetPaper, self).__init__(paper_type="sheet", project_id=project_id, paper_id=sheet_id)


class NotePaper(Paper):

    def __init__(self, project_id: int, note_id: int):
        super(NotePaper, self).__init__(paper_type="note", project_id=project_id, paper_id=note_id)
