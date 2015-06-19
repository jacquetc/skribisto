# -*- coding: utf-8 -*-
'''
Created on 27 may 2015

@author: Cyril Jacquet
'''
from PyQt5.QtGui import QPixmap,  QIcon
from PyQt5.QtWidgets import QDialog,  QWidget,  QToolButton
from PyQt5.QtCore import pyqtSlot,  QSize,  Qt
from .preferences_ui import Ui_Preferences
from enum import Enum


class Preferences(QDialog):

    PageCategories = Enum('PageCategory', 'Interface Editor Advanced')

    def __init__(self, parent=None):
        super(Preferences, self).__init__(parent)
        self.ui = Ui_Preferences()
        self.ui.setupUi(self)

        self.page_dict = {}

        self.ui.previousButton.clicked.connect(self.show_main_page)

        self.add_page(ThemesPreferences,  self.PageCategories.Interface)
        self.add_page(PluginsPreferences,  self.PageCategories.Advanced)

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
