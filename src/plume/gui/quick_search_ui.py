# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/quick_search.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_QuickSearch(object):
    def setupUi(self, QuickSearch):
        QuickSearch.setObjectName("QuickSearch")
        QuickSearch.resize(400, 300)
        self.horizontalLayout = QtWidgets.QHBoxLayout(QuickSearch)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.searchComboBox = QtWidgets.QComboBox(QuickSearch)
        self.searchComboBox.setEditable(True)
        self.searchComboBox.setObjectName("searchComboBox")
        self.horizontalLayout.addWidget(self.searchComboBox)
        self.searchPreviousToolButton = QtWidgets.QToolButton(QuickSearch)
        icon = QtGui.QIcon()
        icon.addPixmap(QtGui.QPixmap(":/pics/32x32/arrow-left.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.searchPreviousToolButton.setIcon(icon)
        self.searchPreviousToolButton.setObjectName("searchPreviousToolButton")
        self.horizontalLayout.addWidget(self.searchPreviousToolButton)
        self.searchNextToolButton = QtWidgets.QToolButton(QuickSearch)
        icon1 = QtGui.QIcon()
        icon1.addPixmap(QtGui.QPixmap(":/pics/32x32/arrow-right.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
        self.searchNextToolButton.setIcon(icon1)
        self.searchNextToolButton.setObjectName("searchNextToolButton")
        self.horizontalLayout.addWidget(self.searchNextToolButton)

        self.retranslateUi(QuickSearch)
        QtCore.QMetaObject.connectSlotsByName(QuickSearch)

    def retranslateUi(self, QuickSearch):

        QuickSearch.setWindowTitle(_("Form"))
        self.searchPreviousToolButton.setText(_("<-"))
        self.searchNextToolButton.setText(_("->"))

