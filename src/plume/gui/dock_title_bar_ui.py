# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eclipse/plume-creator/src/plume/gui/dock_title_bar.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_DockTitleBar(object):
    def setupUi(self, DockTitleBar):
        DockTitleBar.setObjectName("DockTitleBar")
        DockTitleBar.resize(243, 154)
        DockTitleBar.setLocale(QtCore.QLocale(QtCore.QLocale.English, QtCore.QLocale.UnitedStates))
        self.verticalLayout = QtWidgets.QVBoxLayout(DockTitleBar)
        self.verticalLayout.setSpacing(2)
        self.verticalLayout.setContentsMargins(2, 2, 2, 2)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.comboBox = QtWidgets.QComboBox(DockTitleBar)
        self.comboBox.setMinimumSize(QtCore.QSize(100, 0))
        self.comboBox.setObjectName("comboBox")
        self.horizontalLayout.addWidget(self.comboBox)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem)
        self.filterButton = QtWidgets.QToolButton(DockTitleBar)
        self.filterButton.setCheckable(True)
        self.filterButton.setAutoRaise(True)
        self.filterButton.setObjectName("filterButton")
        self.horizontalLayout.addWidget(self.filterButton)
        self.addDockButton = QtWidgets.QToolButton(DockTitleBar)
        self.addDockButton.setAutoRaise(True)
        self.addDockButton.setObjectName("addDockButton")
        self.horizontalLayout.addWidget(self.addDockButton)
        self.closeButton = QtWidgets.QToolButton(DockTitleBar)
        self.closeButton.setAutoRaise(True)
        self.closeButton.setObjectName("closeButton")
        self.horizontalLayout.addWidget(self.closeButton)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.emptyDrawerWidget = QtWidgets.QWidget(DockTitleBar)
        self.emptyDrawerWidget.setObjectName("emptyDrawerWidget")
        self.verticalLayout.addWidget(self.emptyDrawerWidget)

        self.retranslateUi(DockTitleBar)
        QtCore.QMetaObject.connectSlotsByName(DockTitleBar)

    def retranslateUi(self, DockTitleBar):

        DockTitleBar.setWindowTitle(_("Form"))
        self.filterButton.setText(_("F"))
        self.addDockButton.setText(_("+"))
        self.closeButton.setText(_("x"))

