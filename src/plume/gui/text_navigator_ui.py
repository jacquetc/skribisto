# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/text_navigator.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_TextNavigator(object):
    def setupUi(self, TextNavigator):
        TextNavigator.setObjectName("TextNavigator")
        TextNavigator.resize(336, 255)
        self.horizontalLayout = QtWidgets.QHBoxLayout(TextNavigator)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.navPreviousToolButton = QtWidgets.QToolButton(TextNavigator)
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/pics/32x32/arrow-left.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.navPreviousToolButton.setIcon(icon)
        self.navPreviousToolButton.setObjectName("navPreviousToolButton")
        self.horizontalLayout.addWidget(self.navPreviousToolButton)
        self.navNextToolButton = QtWidgets.QToolButton(TextNavigator)
        icon1 = QtGui.QIcon()
        icon1.addPixmap(QtGui.QPixmap(":/pics/32x32/arrow-right.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.navNextToolButton.setIcon(icon1)
        self.navNextToolButton.setObjectName("navNextToolButton")
        self.horizontalLayout.addWidget(self.navNextToolButton)
        self.navigatorComboBox = QtWidgets.QComboBox(TextNavigator)
        self.navigatorComboBox.setObjectName("navigatorComboBox")
        self.horizontalLayout.addWidget(self.navigatorComboBox)

        self.retranslateUi(TextNavigator)
        QtCore.QMetaObject.connectSlotsByName(TextNavigator)

    def retranslateUi(self, TextNavigator):

        TextNavigator.setWindowTitle(_("Form"))
        self.navPreviousToolButton.setText(_("<-"))
        self.navNextToolButton.setText(_("->"))

