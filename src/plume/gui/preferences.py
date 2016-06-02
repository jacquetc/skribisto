# -*- coding: utf-8 -*-
'''
Created on 27 may 2015

@author: Cyril Jacquet
'''
from PyQt5.QtGui import QPixmap,  QIcon
from PyQt5.QtWidgets import QDialog,  QWidget,  QToolButton, QDialogButtonBox
from PyQt5.QtCore import pyqtSlot,  QSize,  Qt, QSettings, pyqtSignal
from .preferences_ui import Ui_Preferences
from . import cfg
from enum import Enum


class Preferences(QDialog):

    PageCategories = Enum('PageCategory', 'Interface Editor Advanced')
    apply_settings_widely_sent = pyqtSignal(name = "apply_settings_widely_sent")

    def __init__(self, parent=None):
        super(Preferences, self).__init__(parent)
        self.ui = Ui_Preferences()
        self.ui.setupUi(self)

        self.page_dict = {}

        self.ui.previousButton.clicked.connect(self.show_main_page)

        self.add_page(ThemesPreferences,  self.PageCategories.Interface)
        self.add_page(PluginsPreferences,  self.PageCategories.Advanced)
        self.add_page(MiscellanousPreferences,  self.PageCategories.Advanced)

        for page in self.page_dict.values():
            page.load_settings()

        self.apply_settings_widely_sent.connect(cfg.signal_hub.apply_settings_widely_sent)

        self.show_main_page()

    def add_page(self,  class_name,  page_category):
        page = class_name()
        self.ui.stackedWidget.addWidget(page)
        self.page_dict[page.page_name] = page
        button = QToolButton(self)
        button.setIcon(page.icon)
        button.setIconSize(QSize(48, 48))
        button.setToolButtonStyle(Qt.ToolButtonTextUnderIcon)
        button.setText(page.title)
        button.setAutoRaise(True)
        button.clicked.connect(self.change_page)
        button.setProperty("page", page.page_name)
        if page_category is self.PageCategories.Interface:
            self.ui.interfaceLayout.addWidget(button)
        if page_category is self.PageCategories.Editor:
            self.ui.editorLayout.addWidget(button)
        if page_category is self.PageCategories.Advanced:
            self.ui.advancedLayout.addWidget(button)

    @pyqtSlot()
    def show_main_page(self):
        self.ui.stackedWidget.setCurrentIndex(0)
        self.ui.titleButton.hide()

    @pyqtSlot()
    def change_page(self):
        sender = self.sender()
        page_name = sender.property("page")
        widget = self.page_dict[page_name]
        self.ui.stackedWidget.setCurrentWidget(widget)
        self.ui.titleButton.show()
        self.ui.titleButton.setText(widget.title)
        self.ui.titleButton.setIcon(widget.icon)

    def on_buttonBox_clicked(self, button):
        role = self.ui.buttonBox.buttonRole(button)
        if role == QDialogButtonBox.ApplyRole:
            for page in self.page_dict.values():
                page.save_settings()
            self.apply_settings_widely_sent.emit()
        if role == QDialogButtonBox.Cancel:
            self.close()
        if role == QDialogButtonBox.AcceptRole:
            for page in self.page_dict.values():
                page.save_settings()
            self.apply_settings_widely_sent.emit()
            self.close()
        if role == QDialogButtonBox.ResetRole:
            for page in self.page_dict.values():
                page.load_settings()


class PreferencesPageTemplate(QWidget):

    def __init__(self, parent=None):
        super(PreferencesPageTemplate, self).__init__(parent)
        self.page_name = ""
        self._icon = ""
        self.title = _("")

    @property
    def icon(self):
        return self._icon

    @icon.setter
    def icon(self,  path):
        self._icon = QIcon(QPixmap(path))

    def save_settings(self):
        pass

    def load_settings(self):
        pass

#-------------------- Plugins

from .preferences_plugins_ui import Ui_PluginsPage


class PluginsPreferences(PreferencesPageTemplate):

    def __init__(self, parent=None):
        super(PluginsPreferences, self).__init__(parent)
        self.ui = Ui_PluginsPage()
        self.ui.setupUi(self)
        self.page_name = "plugins"
        self.icon = ":/pics/48x48/preferences-plugin.png"
        self.title = _("Plugins")


#-------------------- Miscellanous

from .preferences_miscellanous_ui import Ui_MiscellanousPage


class MiscellanousPreferences(PreferencesPageTemplate):

    def __init__(self, parent=None):
        super(MiscellanousPreferences, self).__init__(parent)
        self.ui = Ui_MiscellanousPage()
        self.ui.setupUi(self)
        self.page_name = "miscellanous"
        self.icon = ":/pics/48x48/preferences-other.png"
        self.title = _("Miscellanous")

    def save_settings(self):
        settings = QSettings()
        settings.setValue("settings/misc/dev_mode", self.ui.dev_mode_checkBox.isChecked())
        del settings

    def load_settings(self):
        settings = QSettings()
        self.ui.dev_mode_checkBox.setChecked(settings.value("settings/misc/dev_mode", True, type=bool))
        del settings


#-------------------- Themes

from .preferences_themes_ui import Ui_ThemesPage


class ThemesPreferences(PreferencesPageTemplate):

    def __init__(self, parent=None):
        super(ThemesPreferences, self).__init__(parent)
        self.ui = Ui_ThemesPage()
        self.ui.setupUi(self)
        self.page_name = "themes"
        self.icon = ":/pics/48x48/format-fill-color.png"
        self.title = _("Themes")
