# -*- coding: utf-8 -*-
'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import (QMainWindow, QWidget, QActionGroup,
                             QHBoxLayout,  QFileDialog, QMessageBox,  QApplication, QUndoView)
from PyQt5.QtCore import Qt,  QDir, QDate
from .window_system import WindowSystemController
from .welcome_panel import WelcomePanel
from .write_panel import WritePanel
from .note_panel import NotePanel
from PyQt5.Qt import QToolButton, pyqtSlot, QUndoGroup
from . import cfg
from .main_window_ui import Ui_MainWindow
from .preferences import Preferences
from .start_window import StartWindow
from .about_plume import AboutPlume
from .signal_hub import SignalHub
import os.path


class MainWindow(QMainWindow, WindowSystemController):

    def __init__(self, parent=None):
        super(MainWindow, self).__init__(parent)
        self.init_ui()
        cfg.window = self


        cfg.data.projectHub().allProjectsClosed.connect(self._clear_from_all_projects)
        cfg.data.projectHub().projectLoaded.connect(self._activate)
    # TODO : add saved and not saved connection

    def init_ui(self):
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)

        widget = QWidget()
        layout = QHBoxLayout()

        self.stackWidget = SubWindowStack(self)
        self.side_bar = SideBar(self)
        # self.side_bar.detach_signal.connect(self.detach_window)

        layout.addWidget(self.side_bar)
        layout.addWidget(self.stackWidget)
        # layout.addWidget(self.sub_window)

        widget.setLayout(layout)
        self.setCentralWidget(widget)

        cfg.undo_group = QUndoGroup(self)
        cfg.signal_hub = SignalHub(self)

        # window system :
        self.window_system_parent_widget = self.stackWidget
        self.side_bar.window_system_controller = self

        # sub_windows :
        self._sub_window_action_group = QActionGroup(self)

        # welcome window
        self.welcome_panel = WelcomePanel(
            parent=self, parent_window_system_controller=self)
        self.attach_sub_window(self.welcome_panel)
        self.ui.actionWelcome.setProperty(
            "sub_window_object_name", "welcome_panel")
        self.add_action_to_window_system(self.ui.actionWelcome)
        self._sub_window_action_group.addAction(self.ui.actionWelcome)

        # write window
        self.write_panel = WritePanel(
            parent=self, parent_window_system_controller=self)
        self.attach_sub_window(self.write_panel)
        self.ui.actionWrite.setProperty(
            "sub_window_object_name", "write_panel")
        self.add_action_to_window_system(self.ui.actionWrite)
        self._sub_window_action_group.addAction(self.ui.actionWrite)

        # # note window
        self.note_panel = NotePanel(
            parent=self, parent_window_system_controller=self)
        self.attach_sub_window(self.note_panel)
        self.ui.actionNote.setProperty(
            "sub_window_object_name", "note_panel")
        self.add_action_to_window_system(self.ui.actionNote)
        self._sub_window_action_group.addAction(self.ui.actionNote)

        # open on this panel :
        self.ui.actionWelcome.trigger()




        # menu bar actions
        self.ui.actionOpen_test_project.triggered.connect(
            self.launch_open_test_project)
        self.ui.actionPreferences.triggered.connect(self.launch_preferences)
        self.ui.actionStart_window.triggered.connect(self.launch_start_window)
        self.ui.actionSave.triggered.connect(self.save)
        self.ui.actionSave_as.triggered.connect(self.launch_save_as_dialog)
        self.ui.actionOpen.triggered.connect(self.launch_open_dialog)
        self.ui.actionClose_project.triggered.connect(self.launch_close_dialog)
        self.ui.actionExit.triggered.connect(self.launch_exit_dialog)

        # Help menu actions
        self.ui.actionAbout_Plume_Creator.triggered.connect(self.about_Plume)
        self.ui.actionAbout_Qt.triggered.connect(QApplication.instance().aboutQt)

        self._activate(False)

    def set_application_arguments(self, arguments):
        project_opened_in_arg = False
        for arg in arguments:
            if arg[0] is "-":  # actions if other arguments
                pass
            elif os.path.exists(arg):
                cfg.data.load_database(0, arg)
                project_opened_in_arg = True

        # TODO open default project
        # if not project_opened_in_arg:
        #     cfg.data.load_database(0, "")  # opens empty default project


    @pyqtSlot()
    def launch_open_test_project(self):
        """

        """
        cfg.data.projectHub().loadProject('../../resources/plume_test_project.sqlite')
        # bug : first one of the two following functions will fail !
        # id_list = cfg.data.projectHub().getProjectIdList()
        # print(id_list)
        last_loaded = cfg.data.projectHub().getLastLoaded()
        # print(last_loaded)
        # id_list = cfg.data.projectHub().getProjectIdList()
        # print(id_list)
        # print("last : " + str(last_loaded))
        # print("list : " + repr(id_list))

        # force a default save location other than "resources/"
        from os.path import expanduser
        home = expanduser("~")
        path = os.path.join(home, "plume_test_project.sqlite")
        cfg.data.projectHub().setPath(last_loaded, path)

        self.setWindowTitle("Plume Creator - TEST")

        self.undo_view = QUndoView(cfg.undo_group, None)
        self.undo_view.show()

    @pyqtSlot()
    def launch_preferences(self):
        pref = Preferences(self)
        pref.exec_()

    @pyqtSlot()
    def launch_start_window(self):
        pref = StartWindow(self)
        pref.exec_()

    @pyqtSlot()
    def launch_save_as_dialog(self):
        working_directory = QDir.homePath()
        fileName, selectedFilter = QFileDialog.getSaveFileName(
            self,
            _("Save as"),
            working_directory,
            _("Databases (*.sqlite *.plume);;All files (*)"),
            _(".sqlite"))
        if fileName is None:
            return
        cfg.data.save_database(0, fileName) # ,  selectedFilter

    @pyqtSlot()
    def save(self):
        for projectId in cfg.data.projectHub().getProjectIdList():
            cfg.data.projectHub().saveProject(projectId)

    @pyqtSlot()
    def launch_open_dialog(self):
        working_directory = QDir.homePath()
        fileName, selectedFilter = QFileDialog.getOpenFileName(
            self,
            _("Open"),
            working_directory,
            _("Databases (*.sqlite *.plume);;All files (*)"),
            _(".sqlite"))

        if fileName is None:
            return
        # check if not already open
        project_id_list = cfg.data.projectHub().getProjectIdList()
        for project_id in project_id_list:
            path = cfg.data.projectHub().project(project_id).getPath()
            if os.path.realpath(path) == os.path.realpath(fileName):
                QMessageBox.warning(self, "Warning", "Project already opened")
                return
        cfg.data.projectHub().loadProject(fileName)

        self.setWindowTitle("Plume Creator - " + fileName)

    @pyqtSlot()
    def launch_close_dialog(self):
        project_id_list = cfg.data.projectHub().getProjectIdList()
        if not project_id_list:
            return

        result = QMessageBox.question(
            self,
            _("Close the current projects"),
            _("Do you really want to close the projects which are currently opened ?"),
            QMessageBox.StandardButtons(
                QMessageBox.Cancel |
                QMessageBox.Discard |
                QMessageBox.Save),
            QMessageBox.Cancel)

        if result == QMessageBox.Cancel:
            return QMessageBox.Cancel
        elif result == QMessageBox.Discard:
            cfg.data.projectHub().closeAllProjects()
        elif result == QMessageBox.Save:
            for project_id in project_id_list:
                cfg.data.projectHub().saveProject(project_id)
            cfg.data.projectHub().closeAllProjects()

    @pyqtSlot()
    def launch_exit_dialog(self):
        id_list = cfg.data.projectHub().getProjectIdList()
        if not id_list:
            QApplication.quit()

        result = self.launch_close_dialog()
        if result == QMessageBox.Cancel:
            return QMessageBox.Cancel
        QApplication.quit()

    @pyqtSlot()
    def about_Plume(self):
        about = AboutPlume(self)
        about.exec_()


    def set_project_is_saved(self):
        self.ui.actionSave.setEnabled(False)

    def set_project_is_not_saved(self):
        self.ui.actionSave.setEnabled(True)

    @pyqtSlot()
    def _clear_from_all_projects(self):
        self.setWindowTitle("Plume Creator")
        self._activate(False)

    def _activate(self, value=True):
        self.ui.actionSave_as.setEnabled(value)
        self.ui.actionSave.setEnabled(value)
        self.ui.actionClose_project.setEnabled(value)
        self.ui.actionImport.setEnabled(value)
        self.ui.actionExport.setEnabled(value)
        self.ui.actionPrint.setEnabled(value)

    def closeEvent(self,  event):
        result = self.launch_exit_dialog()
        if result == QMessageBox.Cancel:
            event.ignore()
        else:
            event.accept()

from PyQt5.QtWidgets import QStackedWidget
from .window_system import WindowSystemParentWidget


class SubWindowStack(QStackedWidget, WindowSystemParentWidget):

    '''
    classdocs
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(SubWindowStack, self).__init__(parent)

    def attach_sub_window(self, sub_window):
        WindowSystemParentWidget.attach_sub_window(self, sub_window)
        self.addWidget(sub_window)

    def detach_sub_window(self, sub_window):
        WindowSystemParentWidget.detach_sub_window(self, sub_window)

    def set_sub_window_visible(self, sub_window):
        WindowSystemParentWidget.set_sub_window_visible(self, sub_window)
        if sub_window.isWindow():
            sub_window.setWindowState(
                sub_window.windowState() & ~Qt.WindowMinimized | Qt.WindowActive)
            sub_window.activateWindow()
        else:
            self.setCurrentWidget(sub_window)


from .window_system import WindowSystemActionHandler
from PyQt5.QtWidgets import QToolButton, QMenu,  QSizePolicy
from PyQt5.QtCore import QSize
from .side_bar_ui import Ui_SideBar


class SideBar(QWidget, WindowSystemActionHandler):

    # detach_signal = QtCore.pyqtSignal(QMainWindow)

    def __init__(self, parent=None):
        super(SideBar, self).__init__(parent)
        self.ui = Ui_SideBar()
        self.ui.setupUi(self)

        self.side_bar_button_list = []

    @pyqtSlot()
    def onDetachClicked(self):
        '''
        Must only be used as a slot for Qt signals !
        '''
        self.detach_sub_window()
        for button in self.side_bar_button_list:
            if button.property("sub_window_object_name") != self.sender().property("sub_window_object_name"):
                button.click()
                break

    @pyqtSlot()
    def onAttachClicked(self):
        '''
        Must only be used as a slot for Qt signals !
        '''
        self.attach_sub_window()
        for button in self.side_bar_button_list:
            if button.property("sub_window_object_name") == self.sender().property("sub_window_object_name"):
                button.click()

    @pyqtSlot(bool)
    def action_toggled_slot(self, value):
        '''
        Must only be used as a slot for Qt signals !
        '''
        if value is True:
            self.set_sub_window_visible()
#            for button in self.side_bar_button_list:
#                if button.property("sub_window_object_name") != self.sender().property("sub_window_object_name"):
#                    button.setChecked(True)
#                    break

    @pyqtSlot()
    def set_sub_window_visible(self):
        '''
        Must only be used as a slot for Qt signals !
        '''
        sub_window = self.get_sub_window_linked_to_action(self.sender())
        self.window_system_controller.window_system_parent_widget.set_sub_window_visible(
            sub_window)

    def clear(self):
        for button in self.side_bar_button_list:
            self.side_bar_button_list.remove(button)
            button.deleteLater()
            del button
        self.side_bar_button_list = []

    def update_from_window_system_ctrl(self):

        class SideBarButton(QToolButton):

            def __init__(self, parent, action):
                super(SideBarButton, self).__init__(parent)
                self.setCheckable(True)
                self.setDefaultAction(action)
                self._prop = action.property("sub_window_object_name")
                self.setProperty("sub_window_object_name", self._prop)

            def contextMenuEvent(self, event):

                if self._prop == "welcome_panel":
                    return QToolButton.contextMenuEvent(self, event)

                menu = QMenu(self)
                attachAction = menu.addAction("Attach")
                attachAction.setProperty("sub_window_object_name", self._prop)
                attachAction.triggered.connect(self.parent().onAttachClicked)
                detachAction = menu.addAction("Detach")
                detachAction.setProperty("sub_window_object_name", self._prop)
                detachAction.triggered.connect(self.parent().onDetachClicked)
                menu.exec_(self.mapToGlobal(event.pos()))

                return QToolButton.contextMenuEvent(self, event)

        WindowSystemActionHandler.update_from_window_system_ctrl(self)
        self.clear()

        for action in self._sub_window_action_list:
            action.setCheckable(True)
            action.toggled.connect(self.action_toggled_slot)

            button = SideBarButton(self, action)
            button.setIconSize(QSize(48, 48))
            self.side_bar_button_list.append(button)
            self.ui.actionLayout.addWidget(button)
            self.side_bar_button_list.append(button)
