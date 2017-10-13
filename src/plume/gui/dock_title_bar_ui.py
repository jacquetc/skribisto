# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/plume/plume-creator-py/src/plume/gui/dock_title_bar.ui'
#
# Created by: PyQt5 UI code generator 5.5.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_DockTitleBar(object):
    def setupUi(self, DockTitleBar):
        DockTitleBar.setObjectName("DockTitleBar")
        DockTitleBar.resize(283, 41)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(DockTitleBar.sizePolicy().hasHeightForWidth())
        DockTitleBar.setSizePolicy(sizePolicy)
        DockTitleBar.setLocale(QtCore.QLocale(QtCore.QLocale.English, QtCore.QLocale.UnitedStates))
        self.verticalLayout = QtWidgets.QVBoxLayout(DockTitleBar)
        self.verticalLayout.setContentsMargins(2, 2, 2, 2)
        self.verticalLayout.setSpacing(2)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.comboBox = QtWidgets.QComboBox(DockTitleBar)
        self.comboBox.setMinimumSize(QtCore.QSize(100, 0))
        self.comboBox.setObjectName("comboBox")
        self.horizontalLayout.addWidget(self.comboBox)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem)
        self.displayOptionsButton = QtWidgets.QToolButton(DockTitleBar)
        self.displayOptionsButton.setCheckable(True)
        self.displayOptionsButton.setAutoRaise(True)
        self.displayOptionsButton.setObjectName("displayOptionsButton")
        self.horizontalLayout.addWidget(self.displayOptionsButton)
        self.addDockButton = QtWidgets.QToolButton(DockTitleBar)
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/pics/32x32/list-add.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.addDockButton.setIcon(icon)
        self.addDockButton.setCheckable(False)
        self.addDockButton.setAutoRepeat(False)
        self.addDockButton.setAutoRaise(True)
        self.addDockButton.setObjectName("addDockButton")
        self.horizontalLayout.addWidget(self.addDockButton)
        self.closeButton = QtWidgets.QToolButton(DockTitleBar)
        self.closeButton.setAutoRaise(True)
        self.closeButton.setObjectName("closeButton")
        self.horizontalLayout.addWidget(self.closeButton)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.emptyDrawerWidget = QtWidgets.QWidget(DockTitleBar)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.emptyDrawerWidget.sizePolicy().hasHeightForWidth())
        self.emptyDrawerWidget.setSizePolicy(sizePolicy)
        self.emptyDrawerWidget.setObjectName("emptyDrawerWidget")
        self.verticalLayout.addWidget(self.emptyDrawerWidget)

        self.retranslateUi(DockTitleBar)
        QtCore.QMetaObject.connectSlotsByName(DockTitleBar)

    def retranslateUi(self, DockTitleBar):

        DockTitleBar.setWindowTitle(_("Form"))
        self.displayOptionsButton.setText(_("O"))
        self.addDockButton.setText(_("+"))
        self.closeButton.setText(_("x"))

