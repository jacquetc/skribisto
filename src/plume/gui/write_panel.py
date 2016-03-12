
from PyQt5.QtWidgets import QTabWidget
from PyQt5.QtCore import Qt,  pyqtSlot
from .docks import DockSystem
from .window_system import WindowSystemController
from .models.sheet_tree_model import SheetTreeModel
from . import cfg
from .sub_window import SubWindow


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
        self.setObjectName("write_sub_window")
        self.dock_system = DockSystem(
            self, self,  DockSystem.DockTypes.WritePanelDock)

        # init Sheet Tree Model
        self.tree_model = SheetTreeModel(self, 0)
        cfg.models["0_sheet_tree_model"] = self.tree_model

        # connect core signal to open sheets :
        #cfg.core.tree_sheet_manager.sheet_is_opening.connect(self.open_write_tab)

        # Project tree view dock :
        self.dock_system.add_dock("write-tree-dock",  Qt.LeftDockWidgetArea)

        # set TabWidget:
        self.tab_widget = QTabWidget(self)
        self.setCentralWidget(self.tab_widget)

        # subscribe
        cfg.data_subscriber.subscribe_update_func_to_domain(0, self._clear_project,  "database_closed")

    def open_write_tab(self, tree_sheet):

        new_write_tab = WriteTab(self, self)
        new_write_tab.tree_sheet = tree_sheet
        self.tab_widget.addTab(new_write_tab, new_write_tab.tab_title)
        # temp for test:
        new_write_tab.dock_system.add_dock("properties-dock")

    def _activate(self, value=True):
        if value is True:
            self.tree_model.reset_model()
        else:
            self.tree_model.clear()

    def _clear_project(self):
        # TODO: make that cleaner
        for i in range(0, self.tab_widget.count()):
            widget = self.tab_widget.widget(i)
            widget.close()
            widget.deleteLater()
        self.tab_widget.clear()
        self._activate(False)


from PyQt5.QtWidgets import QWidget
from .write_tab_ui import Ui_WriteTab


class WriteTab(SubWindow):

    '''
    Inner tab in the Write panel. Detachable
    '''

    def __init__(self, parent=None, parent_window_system_controller=None):
        '''
        Constructor= None
        '''
        super(WriteTab, self).__init__(parent=parent)

        self.ui = Ui_WriteTab()
        central_widget = QWidget()
        self.ui.setupUi(central_widget)

        self._tree_sheet = None
        self.tab_title = "Error"

        self.dock_system = DockSystem(
            self, self, DockSystem.DockTypes.WriteTabDock)

        self.setWindowTitle("WriteTab")
        self.ui.writeTabWritingZone.has_minimap = True
        self.ui.writeTabWritingZone.has_scrollbar = True
        self.ui.writeTabWritingZone.is_resizable = True

        self.setCentralWidget(central_widget)

    @property
    def tree_sheet(self):
        return self._tree_sheet

    @tree_sheet.setter
    def tree_sheet(self, tree_sheet_object):
        self._tree_sheet = tree_sheet_object
        self._load_from_tree_sheet(tree_sheet_object)
        self.ui.writeTabWritingZone.text_edit.textChanged.connect(
            self.save_content)

    @pyqtSlot()
    def save_content(self):
        self._tree_sheet.set_content(
            self.ui.writeTabWritingZone.text_edit.toHtml())

    def _load_from_tree_sheet(self, tree_sheet_object):
        cfg.core.subscriber.disable_func(self.save_content)
        self.tab_title = tree_sheet_object.get_title()
        self.ui.writeTabWritingZone.set_rich_text(
            tree_sheet_object.get_content())
        self.dock_system.sheet_id = tree_sheet_object.sheet_id
        cfg.core.subscriber.enable_func(self.save_content)

    def change_tab_title(self, new_title):
        tab_widget = self.parent().tab_widget
        index = tab_widget.indexOf(self)
        tab_widget.setTabText(index, new_title)
