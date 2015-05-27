# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/preferences_plugins.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_PluginsPage(object):
    def setupUi(self, PluginsPage):
        PluginsPage.setObjectName("PluginsPage")
        PluginsPage.resize(400, 300)
        self.gridLayout = QtWidgets.QGridLayout(PluginsPage)
        self.gridLayout.setObjectName("gridLayout")
        self.treeWidget = QtWidgets.QTreeWidget(PluginsPage)
        self.treeWidget.setObjectName("treeWidget")
        self.gridLayout.addWidget(self.treeWidget, 0, 0, 1, 1)

        self.retranslateUi(PluginsPage)
        QtCore.QMetaObject.connectSlotsByName(PluginsPage)

    def retranslateUi(self, PluginsPage):

        PluginsPage.setWindowTitle(_("Form"))
        PluginsPage.setProperty("title", _("Plugins"))
        self.treeWidget.headerItem().setText(0, _("Plugin name"))
        self.treeWidget.headerItem().setText(1, _("activated ?"))

