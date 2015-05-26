# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/plugins/writetreedock/write_tree_dock.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WriteTreeDock(object):
    def setupUi(self, WriteTreeDock):
        WriteTreeDock.setObjectName("WriteTreeDock")
        WriteTreeDock.resize(400, 300)
        self.mainVerticalLayout = QtWidgets.QVBoxLayout(WriteTreeDock)
        self.mainVerticalLayout.setObjectName("mainVerticalLayout")
        self.topWidget = QtWidgets.QWidget(WriteTreeDock)
        self.topWidget.setObjectName("topWidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.topWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.filterLineEdit = QtWidgets.QLineEdit(self.topWidget)
        self.filterLineEdit.setObjectName("filterLineEdit")
        self.horizontalLayout.addWidget(self.filterLineEdit)
        self.removePropButton = QtWidgets.QToolButton(self.topWidget)
        self.removePropButton.setObjectName("removePropButton")
        self.horizontalLayout.addWidget(self.removePropButton)
        self.addPropButton = QtWidgets.QToolButton(self.topWidget)
        self.addPropButton.setObjectName("addPropButton")
        self.horizontalLayout.addWidget(self.addPropButton)
        self.mainVerticalLayout.addWidget(self.topWidget)
        self.treeView = QtWidgets.QTreeView(WriteTreeDock)
        self.treeView.setMouseTracking(False)
        self.treeView.setAlternatingRowColors(True)
        self.treeView.setObjectName("treeView")
        self.mainVerticalLayout.addWidget(self.treeView)

        self.retranslateUi(WriteTreeDock)
        QtCore.QMetaObject.connectSlotsByName(WriteTreeDock)

    def retranslateUi(self, WriteTreeDock):

        WriteTreeDock.setWindowTitle(_("Form"))
        self.filterLineEdit.setPlaceholderText(_("Filter"))
        self.removePropButton.setText(_("-"))
        self.addPropButton.setText(_("+"))

