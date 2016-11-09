from .task import Task
from . import cfg


class SheetHub:
    def __init__(self):
        super(SheetHub, self).__init__()

    @staticmethod
    def get_all(project_id: int):
        task = GetAll(project_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def get_all_headers(project_id: int):
        task = GetAllHeaders(project_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def get_title(project_id: int, sheet_id: int) -> str:
        task = GetTitle(project_id, sheet_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def set_title(project_id: int, sheet_id: int, new_title: str):
        task = SetTitle(project_id, sheet_id, new_title)
        cfg.data.task_manager.append(task)

    @staticmethod
    def get_content(project_id: int, sheet_id: int) -> str :
        task = GetContent(project_id, sheet_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def set_content(project_id: int, sheet_id: int, new_content: str):
        task = SetContent(project_id, sheet_id, new_content)
        cfg.data.task_manager.append(task)

    @staticmethod
    def get_indent(project_id: int, sheet_id: int) -> int:
        task = GetIndent(project_id, sheet_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def set_indent(project_id: int, sheet_id: int, new_indent: int):
        task = SetIndent(project_id, sheet_id, new_indent)
        cfg.data.task_manager.append(task)

    @staticmethod
    def set_property(project_id: int, sheet_id: int, property_name: str, property_value: str):
        task = SetProperty(project_id, sheet_id, property_name, property_value)
        cfg.data.task_manager.append(task)

    @staticmethod
    def get_property(project_id: int, sheet_id: int, property_name: str):
        task = GetProperty(project_id, sheet_id, property_name)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

    @staticmethod
    def get_all_properties(project_id: int):
        task = GetAllProperties(project_id)
        cfg.data.task_manager.append_and_wait_for_reply(task)
        return cfg.data.task_manager.return_value

class GetAll(Task):
    def __init__(self, project_id: int):
        super(GetAll, self).__init__()
        self.project_id = project_id

    def do_task(self):
        self.task_started.emit()

        value = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_all_headers()
        self.return_value = value

        self.task_finished.emit(0)


class GetAllHeaders(Task):
    def __init__(self, project_id: int):
        super(GetAllHeaders, self).__init__()
        self.project_id = project_id

    def do_task(self):
        self.task_started.emit()

        value = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_all_headers()
        self.return_value = value

        self.task_finished.emit(0)



class GetTitle(Task):
    def __init__(self, project_id: int, sheet_id: int):
        super(GetTitle, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id

    def do_task(self):
        self.task_started.emit()

        value = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_title(self.sheet_id)
        self.return_value = value

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

        value = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_content(self.sheet_id)
        self.return_value = value

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


class GetIndent(Task):
    def __init__(self, project_id: int, sheet_id: int):
        super(GetIndent, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id

    def do_task(self):
        self.task_started.emit()

        value = cfg.data.database_manager.get_database(self.project_id).sheet_tree.get_indent(self.sheet_id)
        self.return_value = value

        self.task_finished.emit(0)


class SetIndent(Task):
    def __init__(self, project_id: int, sheet_id: int, new_indent: int):
        super(SetIndent, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id
        self.new_indent = new_indent

    def do_task(self):
        self.task_started.emit()

        cfg.data.database_manager.get_database(self.project_id).sheet_tree.set_indent(self.sheet_id, self.new_indent)
        self.item_value_changed.emit(self.project_id, self.sheet_id, "sheet_set_indent", self.new_indent)

        self.task_finished.emit(0)


class GetAllProperties(Task):
    def __init__(self, project_id: int):
        super(GetAllProperties, self).__init__()
        self.project_id = project_id

    def do_task(self):
        self.task_started.emit()

        property_list = cfg.data.database_manager.get_database(self.project_id).sheet_property_list

        value = property_list.get_all()

        self.return_value = value

        self.task_finished.emit(0)

class GetProperty(Task):
    def __init__(self, project_id: int, sheet_id: int, property_name: str):
        super(GetProperty, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id
        self.property_name = property_name

    def do_task(self):
        self.task_started.emit()

        property_list = cfg.data.database_manager.get_database(self.project_id).sheet_property_list
        # check property's existence :
        property_id = property_list.find_id(self.sheet_id, self.property_name)
        # get value

        value = property_list.get_value(property_id)
        self.return_value = value

        self.task_finished.emit(0)


class SetProperty(Task):
    def __init__(self, project_id: int, sheet_id: int, property_name: str, property_value: str):
        super(SetProperty, self).__init__()
        self.project_id = project_id
        self.sheet_id = sheet_id
        self.property_name = property_name
        self.property_value = property_value

    def do_task(self):
        self.task_started.emit()

        property_list = cfg.data.database_manager.get_database(self.project_id).sheet_property_list
        # check property's existence :
        property_id = property_list.find_id(self.sheet_id, self.property_name)
        # create property if none
        if not property_id:
            property_id = property_list.add_new_property(self.sheet_id)
        # set value
        property_list.set_name(property_id, self.property_name)
        property_list.set_value(property_id, self.property_value)

        cfg.signal_hub.sheet_property_changed.emit(self.project_id, self.sheet_id, self.property_name, self.property_value)

        self.task_finished.emit(0)
