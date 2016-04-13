# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/plume/plume-creator/src/plume/plugins/notetreedock/note_tree_dock.ui'
#
# Created by: PyQt5 UI code generator 5.5.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_NoteTreeDock(object):
    def setupUi(self, NoteTreeDock):
        NoteTreeDock.setObjectName("NoteTreeDock")
        NoteTreeDock.resize(400, 300)
        self.mainVerticalLayout = QtWidgets.QVBoxLayout(NoteTreeDock)
        self.mainVerticalLayout.setObjectName("mainVerticalLayout")
        self.topWidget = QtWidgets.QWidget(NoteTreeDock)
        self.topWidget.setObjectName("topWidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.topWidget)
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
        self.treeView = QtWidgets.QTreeView(NoteTreeDock)
        self.treeView.setMouseTracking(False)
        self.treeView.setAlternatingRowColors(True)
        self.treeView.setObjectName("treeView")
        self.mainVerticalLayout.addWidget(self.treeView)

        self.retranslateUi(NoteTreeDock)
        QtCore.QMetaObject.connectSlotsByName(NoteTreeDock)

    def retranslateUi(self, NoteTreeDock):

        NoteTreeDock.setWindowTitle(_("Form"))
        self.filterLineEdit.setPlaceholderText(_("Filter"))
        self.removePropButton.setText(_("-"))
        self.addPropButton.setText(_("+"))

