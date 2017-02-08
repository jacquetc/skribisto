# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/side_bar.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_SideBar(object):
    def setupUi(self, SideBar):
        SideBar.setObjectName("SideBar")
        SideBar.resize(104, 508)
        self.verticalLayout = QtWidgets.QVBoxLayout(SideBar)
        self.verticalLayout.setSpacing(2)
        self.verticalLayout.setContentsMargins(2, 2, 2, 2)
        self.verticalLayout.setObjectName("verticalLayout")

        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/pics/plume-creator.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.actionLayout = QtWidgets.QVBoxLayout()
        self.actionLayout.setSpacing(3)
        self.actionLayout.setObjectName("actionLayout")
        self.verticalLayout.addLayout(self.actionLayout)
        spacerItem = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout.addItem(spacerItem)

        self.retranslateUi(SideBar)
        QtCore.QMetaObject.connectSlotsByName(SideBar)

    def retranslateUi(self, SideBar):

        SideBar.setWindowTitle(_("Form"))

