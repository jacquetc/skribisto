from .task import Task
from . import cfg


class ProjectHub:
    def __init__(self):
        super(ProjectHub, self).__init__()

    @staticmethod
    def load_project(project_path: str):
        task = LoadProject(project_path)
        cfg.data.task_manager.append(task)

    @staticmethod
    def close_project(project_id: int):
        task = CloseProject(project_id)
        cfg.data.task_manager.append(task)

    @staticmethod
    def close_all_projects():
        keys = list(cfg.data.database_manager.database_for_int_dict.keys())
        for project_id in keys:
            task = CloseProject(project_id)
            cfg.data.task_manager.append(task)


class LoadProject(Task):
    def __init__(self, project_path: str):
        super(LoadProject, self).__init__()
        self.project_path = project_path

    def do_task(self):
        self.task_started.emit()

        project_id = cfg.data.database_manager.load_database(self.project_path)
        self.item_value_returned.emit(project_id, None, "project_loaded", project_id)

        self.task_finished.emit(0)


class CloseProject(Task):
    def __init__(self, project_id: int):
        super(CloseProject, self).__init__()
        self.project_id = project_id

    def do_task(self):
        self.task_started.emit()

        cfg.data.database_manager.close_database(self.project_id)
        self.item_value_returned.emit(self.project_id, None, "project_closed", self.project_id)

        self.task_finished.emit(0)

