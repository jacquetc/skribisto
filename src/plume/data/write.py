from . import cfg
from .task import Task
from . import cfg

class Write:
    def __init__(self):
        super(Write, self).__init__()

    def get_title(project_id: int, sheet_id: int):
        task = GetTitle(project_id, sheet_id)
        cfg.data.task_manager.append(task)

class GetTitle(Task):
    def __init__(self, project_id: int, sheet_id: int):
        super(GetTitle, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id

    def do_task(self):
        self.task_started.emit()

        title = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_title(self.sheet_id)
        self.item_value_returned.emit(self.project_id, self.sheet_id, "write_get_title", title)

        self.task_finished.emit(0)
