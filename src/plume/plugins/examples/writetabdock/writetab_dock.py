'''
Created on 4 juin 2015

@author:  Cyril Jacquet
'''
from gui import plugins as gui_plugins
# from core import cfg as core_cfg
from PyQt5.Qt import pyqtSlot
from PyQt5.QtCore import Qt


# class WriteTabDockPlugin(core_plugins.CoreWriteTabDockPlugin, gui_plugins.GuiWriteTabDockPlugin):
class WriteSubWindowDockPlugin(gui_plugins.GuiWriteSubWindowDockPlugin):

    '''
    WriteSubWindowDockPlugin

    This is an example of dock for the WriteSubWindow, in the "Write" panel. This is a minimal dock with two parts : Core and Gui .
    The Gui part calls a writetab_dock.ui form. The compiled form writetab_dock_ui.py is build automaticaly if the environement variable PLUME_DEVELOP_FROM points to a parent directory of this plugin.

    The goal of this dock is to display and modify the title of the current sheet. Nothing really useful...
    '''
    is_builtin_plugin = False
    ignore = True
    def __init__(self):
        '''
        Constructor
        '''

        super(WriteSubWindowDockPlugin, self).__init__()

    def core_class(self):
        '''
        Write your core class without '()'
        '''
        return CoreWriteTabDock

    def gui_class(self):
        '''
        Write your gui class without '()'
        '''
        return GuiWriteTabDock


class CoreWriteTabDock():

    '''
    CoreWriteTabDock
    '''

    dock_name = "writetab-example-dock"

    def __init__(self):
        '''
        Constructor
        '''

        super(CoreWriteTabDock, self).__init__()
        self._sheet_id = None
        self.tree_sheet = None
        # property variable :
        self._title_text = None

    @property
    def sheet_id(self):
        return self._sheet_id

    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            return
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            # you fetch the current sheet :
            self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
                self.sheet_id)
            # initialize title_text()
            _ = self.title_text

    @property
    def title_text(self):  # rename the property as you see fit
        if self._title_text is None:
            self._title_text = ""
            if self._sheet_id is not None:
                self._title_text = self.tree_sheet.get_title()

        return self._title_text

    @title_text.setter  # rename the property here too !
    def title_text(self,  text):  # rename the property here too !
        if self.sheet_id is not None:
            # you fetch the current sheet :
            self.tree_sheet = core_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
                self.sheet_id)
            self.tree_sheet.set_title(text)  # apply the change
            self._title_text = text


from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QObject
from gui import cfg as gui_cfg
from plugins.examples.writetabdock import writetab_dock_ui


class GuiWriteTabDock(QObject):

    '''
    GuiWriteTabDock
    '''
    dock_name = "writetab-example-dock"
    dock_displayed_name = _("Writetab Example")

    def __init__(self,  parent=None):
        '''
        Constructor
        '''
        super(GuiWriteTabDock, self).__init__(parent)
        self.widget = None
        self.core_part = None  # CoreWriteTabDock
        self._sheet_id = None
        self.tree_sheet = None

    @property
    def sheet_id(self):
        return self._sheet_id

    @sheet_id.setter
    def sheet_id(self, sheet_id):
        if self._sheet_id == sheet_id:
            pass
        self._sheet_id = sheet_id
        if self.sheet_id is not None:
            self.tree_sheet = gui_cfg.core.tree_sheet_manager.get_tree_sheet_from_sheet_id(
                self.sheet_id)
            self.core_part = self.tree_sheet.get_instance_of(self.dock_name)
            self.core_part.sheet_id = sheet_id
            core_cfg.data.subscriber.unsubscribe_update_func(self.get_update)
            # subscribe to an updater, so to be up to date when changes occur :
            core_cfg.data.subscriber.subscribe_update_func_to_domain(
                self.get_update, "data.sheet_tree.title", self._sheet_id)

    def get_widget(self):

        if self.widget is None:
            self.widget = QWidget()
            self.ui = writetab_dock_ui.Ui_WriteTabDock()
            self.ui.setupUi(self.widget)

            if self.tree_sheet is not None and self.core_part is not None:
                self.get_update()

                # connect :
                self.ui.lineEdit.editingFinished.connect(
                    self.apply_text_change, type=Qt.UniqueConnection)

            self.widget.gui_part = self
        return self.widget

    def get_update(self):
        self.ui.lineEdit.blockSignals(True)
        if self.tree_sheet is not None and self.core_part is not None:
            text = self.core_part.title_text
            self.ui.lineEdit.setText(text)
        self.ui.lineEdit.blockSignals(False)

    @pyqtSlot()
    def apply_text_change(self):
        core_cfg.data.subscriber.disable_func(self.get_update)
        self.core_part.title_text = self.ui.lineEdit.text()
        core_cfg.data.subscriber.enable_func(self.get_update)
