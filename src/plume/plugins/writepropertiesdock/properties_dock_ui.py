# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/plume/plume-creator-py/src/plume/plugins/writepropertiesdock/properties_dock.ui'
#
# Created by: PyQt5 UI code generator 5.7.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WritePropertiesDock(object):
    def setupUi(self, WritePropertiesDock):
        WritePropertiesDock.setObjectName("WritePropertiesDock")
        WritePropertiesDock.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(WritePropertiesDock)
        self.verticalLayout.setObjectName("verticalLayout")
        self.topWidget = QtWidgets.QWidget(WritePropertiesDock)
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
        self.verticalLayout.addWidget(self.topWidget)
        self.tableView = QtWidgets.QTableView(WritePropertiesDock)
        self.tableView.setProperty("showDropIndicator", False)
        self.tableView.setDragDropOverwriteMode(False)
        self.tableView.setAlternatingRowColors(True)
        self.tableView.setSortingEnabled(True)
        self.tableView.setObjectName("tableView")
        self.tableView.horizontalHeader().setCascadingSectionResizes(True)
        self.tableView.horizontalHeader().setDefaultSectionSize(70)
        self.tableView.horizontalHeader().setHighlightSections(False)
        self.tableView.horizontalHeader().setMinimumSectionSize(50)
        self.tableView.horizontalHeader().setStretchLastSection(True)
        self.tableView.verticalHeader().setVisible(False)
        self.tableView.verticalHeader().setDefaultSectionSize(20)
        self.tableView.verticalHeader().setMinimumSectionSize(20)
        self.verticalLayout.addWidget(self.tableView)

        self.retranslateUi(WritePropertiesDock)
        QtCore.QMetaObject.connectSlotsByName(WritePropertiesDock)

    def retranslateUi(self, WritePropertiesDock):

        WritePropertiesDock.setWindowTitle(_("Form"))
        self.filterLineEdit.setPlaceholderText(_("Filter"))
        self.removePropButton.setText(_("-"))
        self.addPropButton.setText(_("+"))

