# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/plugins/propertiesdock/properties_dock.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_PropertiesDock(object):
    def setupUi(self, PropertiesDock):
        PropertiesDock.setObjectName("PropertiesDock")
        PropertiesDock.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(PropertiesDock)
        self.verticalLayout.setObjectName("verticalLayout")
        self.topWidget = QtWidgets.QWidget(PropertiesDock)
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
        self.tableView = QtWidgets.QTableView(PropertiesDock)
        self.tableView.setAlternatingRowColors(True)
        self.tableView.setSortingEnabled(True)
        self.tableView.setObjectName("tableView")
        self.tableView.horizontalHeader().setCascadingSectionResizes(True)
        self.tableView.horizontalHeader().setDefaultSectionSize(70)
        self.tableView.horizontalHeader().setHighlightSections(False)
        self.tableView.horizontalHeader().setMinimumSectionSize(50)
        self.tableView.horizontalHeader().setSortIndicatorShown(True)
        self.tableView.horizontalHeader().setStretchLastSection(True)
        self.tableView.verticalHeader().setVisible(False)
        self.tableView.verticalHeader().setDefaultSectionSize(20)
        self.tableView.verticalHeader().setMinimumSectionSize(20)
        self.verticalLayout.addWidget(self.tableView)

        self.retranslateUi(PropertiesDock)
        QtCore.QMetaObject.connectSlotsByName(PropertiesDock)

    def retranslateUi(self, PropertiesDock):

        PropertiesDock.setWindowTitle(_("Form"))
        self.filterLineEdit.setPlaceholderText(_("Filter"))
        self.removePropButton.setText(_("-"))
        self.addPropButton.setText(_("+"))

