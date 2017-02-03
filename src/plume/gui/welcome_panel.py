from PyQt5.QtWidgets import QStackedWidget, QMainWindow
from PyQt5.QtCore import Qt,  pyqtSlot
from .docks import DockSystem
from .window_system import WindowSystemController
from .models.sheet_tree_model import SheetTreeModel
from .models.sheet_property_model import SheetPropertyModel
from .models.sheet_system_property_model import SheetSystemPropertyModel
from . import cfg
from .sub_window import SubWindow
from .paper_manager import PaperManager, SheetPaper


class WelcomePanel(SubWindow, WindowSystemController):

    '''
    'Welcome panel. Not detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''

        '''
        super(WelcomePanel, self).__init__(
            parent=parent, parent_window_system_controller=parent_window_system_controller)

        self.setWindowTitle(_("Welcome"))
        self.setObjectName("welcome_panel")
        self.dock_system = DockSystem(
            self, self,  DockSystem.DockTypes.WritePanelDock)