from PyQt5.QtWidgets import QStackedWidget, QMainWindow
from PyQt5.QtCore import Qt,  pyqtSlot, QUrl
from PyQt5.QtQuickWidgets import QQuickWidget
from .docks import DockSystem
from .window_system import WindowSystemController
from .models.sheet_tree_model import SheetTreeModel
from .models.sheet_property_model import SheetPropertyModel
from .models.sheet_system_property_model import SheetSystemPropertyModel
from . import cfg
from .sub_window import SubWindow
from .paper_manager import PaperManager, SheetPaper
import os, inspect
from .models.project_list_model import ProjectListModel


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

        self.project_list_model = ProjectListModel(self)
        cfg.models["0_project_list_model"] = self.project_list_model
        cfg.undo_group.addStack(self.project_list_model.undo_stack)

        mQQuickWidget = QQuickWidget(self)
        mQQuickWidget.setResizeMode(QQuickWidget.SizeRootObjectToView)
        mQQuickWidget.rootContext().setContextProperty("project_list_model", self.project_list_model)
        mQQuickWidget.rootContext().setContextProperty("window", cfg.window)
        # abspath = os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe())))
        # mQQuickWidget.setSource(QUrl(abspath + "/qml/WelcomePage.qml"))
        mQQuickWidget.setSource(QUrl("qrc:/qml/WelcomePage.qml"))
        self.setCentralWidget(mQQuickWidget)



