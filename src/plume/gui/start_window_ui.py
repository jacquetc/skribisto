# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/start_window.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_StartWindow(object):
    def setupUi(self, StartWindow):
        StartWindow.setObjectName("StartWindow")
        StartWindow.resize(400, 300)
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/pics/plume-creator.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        StartWindow.setWindowIcon(icon)
        StartWindow.setModal(True)
        self.horizontalLayout = QtWidgets.QHBoxLayout(StartWindow)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.verticalLayout = QtWidgets.QVBoxLayout()
        self.verticalLayout.setObjectName("verticalLayout")
        self.newProjectButton = QtWidgets.QToolButton(StartWindow)
        self.newProjectButton.setObjectName("newProjectButton")
        self.verticalLayout.addWidget(self.newProjectButton)
        self.openButton = QtWidgets.QToolButton(StartWindow)
        self.openButton.setObjectName("openButton")
        self.verticalLayout.addWidget(self.openButton)
        self.openOtherButton = QtWidgets.QToolButton(StartWindow)
        self.openOtherButton.setObjectName("openOtherButton")
        self.verticalLayout.addWidget(self.openOtherButton)
        self.closeButton = QtWidgets.QToolButton(StartWindow)
        self.closeButton.setObjectName("closeButton")
        self.verticalLayout.addWidget(self.closeButton)
        self.horizontalLayout.addLayout(self.verticalLayout)
        self.listWidget = QtWidgets.QListWidget(StartWindow)
        self.listWidget.setObjectName("listWidget")
        self.horizontalLayout.addWidget(self.listWidget)

        self.retranslateUi(StartWindow)
        QtCore.QMetaObject.connectSlotsByName(StartWindow)

    def retranslateUi(self, StartWindow):

        StartWindow.setWindowTitle(_("Start"))
        self.newProjectButton.setText(_("New project"))
        self.openButton.setText(_("Open"))
        self.openOtherButton.setText(_("Open other..."))
        self.closeButton.setText(_("Close"))

