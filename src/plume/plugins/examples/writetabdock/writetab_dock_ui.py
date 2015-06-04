# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/plugins/examples/writetabdock/writetab_dock.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WriteTabDock(object):
    def setupUi(self, WriteTabDock):
        WriteTabDock.setObjectName("WriteTabDock")
        WriteTabDock.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(WriteTabDock)
        self.verticalLayout.setObjectName("verticalLayout")
        self.topWidget = QtWidgets.QWidget(WriteTabDock)
        self.topWidget.setObjectName("topWidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.topWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.pushButton = QtWidgets.QPushButton(self.topWidget)
        self.pushButton.setObjectName("pushButton")
        self.horizontalLayout.addWidget(self.pushButton)
        self.label = QtWidgets.QLabel(self.topWidget)
        self.label.setObjectName("label")
        self.horizontalLayout.addWidget(self.label)
        self.verticalLayout.addWidget(self.topWidget)
        self.gridLayout = QtWidgets.QGridLayout()
        self.gridLayout.setObjectName("gridLayout")
        self.lineEdit = QtWidgets.QLineEdit(WriteTabDock)
        self.lineEdit.setObjectName("lineEdit")
        self.gridLayout.addWidget(self.lineEdit, 2, 0, 1, 1)
        self.label_2 = QtWidgets.QLabel(WriteTabDock)
        self.label_2.setObjectName("label_2")
        self.gridLayout.addWidget(self.label_2, 1, 0, 1, 1)
        spacerItem = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.gridLayout.addItem(spacerItem, 0, 0, 1, 1)
        spacerItem1 = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.gridLayout.addItem(spacerItem1, 3, 0, 1, 1)
        self.verticalLayout.addLayout(self.gridLayout)

        self.retranslateUi(WriteTabDock)
        QtCore.QMetaObject.connectSlotsByName(WriteTabDock)

    def retranslateUi(self, WriteTabDock):

        WriteTabDock.setWindowTitle(_("Form"))
        self.pushButton.setText(_("PushButton"))
        self.label.setText(_("<html><head/><body><p><span style=\" font-weight:600; text-decoration: underline;\">Do not remove</span> this \'topWidget\'. </p><p>Use it to add functionalities, options...</p></body></html>"))
        self.label_2.setText(_("Edit the title :"))

