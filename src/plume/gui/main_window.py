# -*- coding: utf-8 -*-
'''
Created on 25 avr. 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import (QMainWindow, QWidget, QActionGroup,
                             QHBoxLayout,  QFileDialog, QMessageBox,  QApplication)
from PyQt5.QtCore import Qt,  QDir
from .window_system import WindowSystemController
from .sub_window import WritePanel
from .binder import BinderPanel
from PyQt5.Qt import QToolButton, pyqtSlot
from . import cfg
from .main_window_ui import Ui_MainWindow
from .preferences import Preferences
from .start_window import StartWindow


class MainWindow(QMainWindow, WindowSystemController):

    def __init__(self, parent):
        super(MainWindow, self).__init__()
        self.init_ui()

        cfg.core.subscriber.subscribe_update_func_to_domain(
            self._clear_project,  "core.project.close")
        cfg.core.subscriber.subscribe_update_func_to_domain(
            self._enable_actions,  "core.project.load")
        cfg.core.subscriber.subscribe_update_func_to_domain(
            self.set_project_is_saved, "core.project.saved")
        cfg.core.subscriber.subscribe_update_func_to_domain(
            self.set_project_is_not_saved, "core.project.notsaved")

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

        # window system :
        self.window_system_parent_widget = self.stackWidget
        self.side_bar.window_system_controller = self

        # sub_windows :
        self._sub_window_action_group = QActionGroup(self)

        # write window
        self.write_sub_window = WritePanel(
            parent=self, parent_window_system_controller=self)
        self.attach_sub_window(self.write_sub_window)
        self.ui.actionWrite.setProperty(
            "sub_window_object_name", "write_sub_window")
        self.add_action_to_window_system(self.ui.actionWrite)
        self._sub_window_action_group.addAction(self.ui.actionWrite)

        # binder window
        self.binder_sub_window = BinderPanel(
            parent=self, parent_window_system_controller=self)
        self.attach_sub_window(self.binder_sub_window)
        self.ui.actionBinder.setProperty(
            "sub_window_object_name", "binder_sub_window")
        self.add_action_to_window_system(self.ui.actionBinder)
        self._sub_window_action_group.addAction(self.ui.actionBinder)

        self.ui.actionWrite.trigger()

        # menu bar actions
        self.ui.actionOpen_test_project.triggered.connect(
            self.launch_open_test_project)
        self.ui.actionPreferences.triggered.connect(self.launch_preferences)
        self.ui.actionStart_window.triggered.connect(self.launch_start_window)
        self.ui.actionSave.triggered.connect(cfg.core.project.save)
        self.ui.actionSave_as.triggered.connect(self.launch_save_as_dialog)
        self.ui.actionOpen.triggered.connect(self.launch_open_dialog)
        self.ui.actionClose_project.triggered.connect(self.launch_close_dialog)
        self.ui.actionExit.triggered.connect(self.launch_exit_dialog)

        self._enable_actions(False)

    @pyqtSlot()
    def launch_open_test_project(self):
        if cfg.core.project.is_open() == True:
            if self.launch_close_dialog() == QMessageBox.Cancel:
                return
        cfg.core.project.open_test_project()
        self.setWindowTitle("Plume Creator - TEST")

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
        cfg.core.project.save_as(fileName,  selectedFilter)

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
        if cfg.core.project.is_open() == True:
            if self.launch_close_dialog() == QMessageBox.Cancel:
                return
        cfg.core.project.open(fileName)

        self.setWindowTitle("Plume Creator - " + fileName)

    def launch_close_dialog(self):
        if cfg.core.project.is_open() == False:
            return

        result = QMessageBox.question(
            self,
            _("Close the current project"),
            _("The last changes are not yet saved. Do you really want to close the current project ?"),
            QMessageBox.StandardButtons(
                QMessageBox.Cancel |
                QMessageBox.Discard |
                QMessageBox.Save),
            QMessageBox.Cancel)

        if result == QMessageBox.Cancel:
            return QMessageBox.Cancel
        elif result == QMessageBox.Discard:
            cfg.core.project.close_project()
        elif result == QMessageBox.Save:
            cfg.core.project.save()
            cfg.core.project.close_project()

    def launch_exit_dialog(self):
        if cfg.core.project.is_open() == False:
            QApplication.quit()

        result = self.launch_close_dialog()
        if result == QMessageBox.Cancel:
            return QMessageBox.Cancel
        QApplication.quit()

    def set_project_is_saved(self):
        self.ui.actionSave.setEnabled(False)

    def set_project_is_not_saved(self):
        self.ui.actionSave.setEnabled(True)

    def _clear_project(self):
        self.setWindowTitle("Plume Creator")
        self._enable_actions(False)

    def _enable_actions(self,  value=True):
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
