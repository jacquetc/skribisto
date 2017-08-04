from PyQt5.QtWidgets import QWidget, QTabWidget, QStackedWidget, QMainWindow
from PyQt5.QtCore import Qt,  pyqtSlot
from .docks import DockSystem
from .models.note_tree_model import NoteTreeModel
from .models.note_list_model import NoteListModel
from .models.note_property_model import NotePropertyModel
from .models.note_system_property_model import NoteSystemPropertyModel
from . import cfg
from .sub_window import SubWindow
from .paper_manager import PaperManager, NotePaper

class NotePanel(SubWindow):

    '''
    classdocs
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor
        '''
        super(NotePanel, self).__init__(
            parent=parent, parent_window_system_controller=parent_window_system_controller)

        self.setWindowTitle(_("Note"))
        self.setObjectName("note_panel")

        self.dock_system = DockSystem(
            self, self,  DockSystem.DockTypes.NotePanelDock)

        self.setDockNestingEnabled(False)
        self.setDockOptions((self.dockOptions() | QMainWindow.AnimatedDocks) ^ QMainWindow.AllowTabbedDocks)

        # self.system_property_model = NoteSystemPropertyModel(self)
        # cfg.models["0_note_system_property_model"] = self.system_property_model
        # cfg.undo_group.addStack(self.system_property_model.undo_stack)
        # self.property_model = NotePropertyModel(self, 0)
        # cfg.models["0_note_property_model"] = self.property_model
        # cfg.undo_group.addStack(self.property_model.undo_stack)
        # init Note Tree Model
        self.tree_model = NoteTreeModel(self)
        cfg.models["0_note_tree_model"] = self.tree_model
        cfg.undo_group.addStack(self.tree_model.undo_stack)
        self.list_model = NoteListModel(self)
        cfg.models["0_note_list_model"] = self.list_model
        #cfg.undo_group.addStack(self.list_model.undo_stack)

        self.note_manager = PaperManager()

        # Project tree view dock :
        self.dock_system.add_dock("note-tree-dock", Qt.LeftDockWidgetArea)

        # set QStackWidget:
        self.stack_widget = QStackedWidget(self)
        self.setCentralWidget(self.stack_widget)

        # subscribe
        cfg.data.projectHub().projectClosed.connect(self._clear_from_project)
        cfg.data.projectHub().allProjectsClosed.connect(self._clear_from_all_projects)

    def open_note(self, project_id: int, note_id: int):
        """

        :param project_id:
        :param note_id:
        """

        new_window = NoteSubWindow(self, self)
        new_window.paper = NotePaper(project_id, note_id)
        self.stack_widget.addWidget(new_window)
        self.make_widget_current(note_id)

    def make_widget_current(self, note_id: int):
        for i in range(0, self.stack_widget.count()):
            if self.stack_widget.widget(i).paper.paper_id == note_id:
                self.stack_widget.setCurrentIndex(i)

    def _activate(self, value=True):
        if value is True:
            self.tree_model.reset_model()
        else:
            self.tree_model.clear_from_all_projects()

    @pyqtSlot()
    def _clear_from_all_projects(self):
        # TODO: make that cleaner
        for i in range(0, self.stack_widget.count()):
            widget = self.stack_widget.widget(i)
            widget.close()
            widget.deleteLater()
        self._activate(False)

    @pyqtSlot(int)
    def _clear_from_project(self, project_id: int):
        # TODO: make that cleaner
        for i in range(0, self.stack_widget.count()):
            widget = self.stack_widget.widget(i)
            if widget.project_id == project_id:
                widget.close()
                widget.deleteLater()
        self._activate(False)


from .write_tab_ui import Ui_WriteTab


class NoteSubWindow(SubWindow):

    def __init__(self, parent=None, parent_window_system_controller=None):

        super(NoteSubWindow, self).__init__(parent=parent)

        self.ui = Ui_WriteTab()
        central_widget = QWidget()
        self.ui.setupUi(central_widget)

        self._paper = None
        self.tab_title = "Error"

        self.setDockNestingEnabled(False)
        self.setDockOptions((self.dockOptions() | QMainWindow.AnimatedDocks) ^ QMainWindow.AllowTabbedDocks)
        self.dock_system = DockSystem(
            self, self, DockSystem.DockTypes.NoteSubWindowDock)

        self.setWindowTitle("NoteTab")
        self.ui.writeTabWritingZone.has_minimap = True
        self.ui.writeTabWritingZone.has_scrollbar = True
        self.ui.writeTabWritingZone.is_resizable = True

        self.setCentralWidget(central_widget)


    @property
    def paper_id(self):
        return self.paper.paper_id

    @property
    def project_id(self):
        return self.paper.project_id

    @property
    def paper(self):
        return self._paper

    @paper.setter
    def paper(self, paper_object):
        self._paper = paper_object
        self.ui.writeTabWritingZone.text_edit.textChanged.connect(
            self.save_content)
        self._load_from_paper(paper_object)

    @pyqtSlot()
    def save_content(self):
        self._paper.content = self.ui.writeTabWritingZone.text_edit.toHtml()

    def _load_from_paper(self, paper_object):
        self.ui.writeTabWritingZone.text_edit.textChanged.disconnect(
            self.save_content)
        self.tab_title = paper_object.title
        self.ui.writeTabWritingZone.set_rich_text(
            paper_object.content)
        self.dock_system.project_id = paper_object.project_id
        self.dock_system.paper_id = paper_object.paper_id
        self.ui.writeTabWritingZone.text_edit.textChanged.connect(
            self.save_content)
