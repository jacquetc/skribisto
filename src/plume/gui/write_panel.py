
from PyQt5.QtWidgets import QStackedWidget, QMainWindow
from PyQt5.QtCore import Qt,  pyqtSlot
from .docks import DockSystem
from .window_system import WindowSystemController
from .models.sheet_tree_model import SheetTreeModel
from . import cfg
from .sub_window import SubWindow
from .paper_manager import PaperManager, SheetPaper


class WritePanel(SubWindow, WindowSystemController):

    '''
    'Write' main panel. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''

        '''
        super(WritePanel, self).__init__(
            parent=parent, parent_window_system_controller=parent_window_system_controller)

        self.setWindowTitle(_("Write"))
        self.setObjectName("write_panel")
        self.dock_system = DockSystem(
            self, self,  DockSystem.DockTypes.WritePanelDock)

        self.setDockNestingEnabled(False)
        self.setDockOptions(self.dockOptions() ^ QMainWindow.AnimatedDocks)

        # init Sheet Tree Model
        self.tree_model = SheetTreeModel(self, 0)
        cfg.models["0_sheet_tree_model"] = self.tree_model

        self.sheet_manager = PaperManager()

        # Project tree view dock :
        self.dock_system.add_dock("write-tree-dock",  Qt.LeftDockWidgetArea)

        # set QStackWidget:
        self.stack_widget = QStackedWidget(self)
        self.setCentralWidget(self.stack_widget)

        # subscribe
        cfg.data_subscriber.subscribe_update_func_to_domain(0, self._clear_project,  "database_closed")

    def open_sheet(self, sheet_id: int):
        """

        :param sheet_id:
        """

        new_window = WriteSubWindow(self, self)
        paper = SheetPaper(sheet_id)
        new_window.paper = paper
        self.stack_widget.addWidget(new_window)
        self.make_widget_current(sheet_id)
        # temp for test:
        new_window.dock_system.add_dock("synopsis-dock")

    def make_widget_current(self, sheet_id: int):
        for i in range(0, self.stack_widget.count()):
            if self.stack_widget.widget(i).paper.id == sheet_id:
                self.stack_widget.setCurrentIndex(i)

    def _activate(self, value=True):
        if value is True:
            self.tree_model.reset_model()
        else:
            self.tree_model.clear()

    def _clear_project(self):
        # TODO: make that cleaner
        for i in range(0, self.stack_widget.count()):
            widget = self.stack_widget.widget(i)
            widget.close()
            widget.deleteLater()
        self.sheet_manager.clear()
        self._activate(False)


from PyQt5.QtWidgets import QWidget
from .write_tab_ui import Ui_WriteTab


class WriteSubWindow(SubWindow):

    '''
    Detachable ???
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor= None
        '''
        super(WriteSubWindow, self).__init__(parent=parent)

        self.ui = Ui_WriteTab()
        central_widget = QWidget()
        self.ui.setupUi(central_widget)

        self._paper = None
        self.tab_title = "Error"

        self.dock_system = DockSystem(
            self, self, DockSystem.DockTypes.WriteSubWindowDock)

        self.setWindowTitle("WriteTab")
        self.ui.writeTabWritingZone.has_minimap = True
        self.ui.writeTabWritingZone.has_scrollbar = True
        self.ui.writeTabWritingZone.is_resizable = True

        self.setCentralWidget(central_widget)

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
        self.dock_system.paper_id = paper_object.id
        self.ui.writeTabWritingZone.text_edit.textChanged.connect(
            self.save_content)

