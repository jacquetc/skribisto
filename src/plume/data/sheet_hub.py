from .task import Task
from . import cfg


class SheetHub:
    def __init__(self):
        super(SheetHub, self).__init__()

    @staticmethod
    def get_title(project_id: int, sheet_id: int):
        task = GetTitle(project_id, sheet_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def set_title(project_id: int, sheet_id: int, new_title: str):
        task = SetTitle(project_id, sheet_id, new_title)
        cfg.data.task_manager.append(task)

    @staticmethod
    def get_content(project_id: int, sheet_id: int):
        task = GetContent(project_id, sheet_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def set_content(project_id: int, sheet_id: int, new_content: str):
        task = SetContent(project_id, sheet_id, new_content)
        cfg.data.task_manager.append(task)

class GetTitle(Task):
    def __init__(self, project_id: int, sheet_id: int):
        super(GetTitle, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id

    def do_task(self):
        self.task_started.emit()

        title = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_title(self.sheet_id)
        self.return_value = title
        #self.item_value_returned.emit(self.project_id, self.sheet_id, "sheet_get_title", title)

        self.task_finished.emit(0)


class SetTitle(Task):
    def __init__(self, project_id: int, sheet_id: int, new_title: str):
        super(SetTitle, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id
        self.new_title = new_title

    def do_task(self):
        self.task_started.emit()

        cfg.data.database_manager.get_database(self.project_id).sheet_tree.set_title(self.sheet_id, self.new_title)
        self.item_value_changed.emit(self.project_id, self.sheet_id, "sheet_set_title", self.new_title)

        self.task_finished.emit(0)

class GetContent(Task):
    def __init__(self, project_id: int, sheet_id: int):
        super(GetContent, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id

    def do_task(self):
        self.task_started.emit()

        title = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_content(self.sheet_id)
        self.return_value = title
        #self.item_value_returned.emit(self.project_id, self.sheet_id, "sheet_get_title", title)

        self.task_finished.emit(0)


class SetContent(Task):
    def __init__(self, project_id: int, sheet_id: int, new_content: str):
        super(SetContent, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id
        self.new_content = new_content

    def do_task(self):
        self.task_started.emit()

        cfg.data.database_manager.get_database(self.project_id).sheet_tree.set_content(self.sheet_id, self.new_content)
        self.item_value_changed.emit(self.project_id, self.sheet_id, "sheet_set_content", self.new_content)

        self.task_finished.emit(0)