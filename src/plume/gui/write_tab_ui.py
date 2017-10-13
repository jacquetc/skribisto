# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/plume/plume-creator-py/src/plume/gui/write_tab.ui'
#
# Created by: PyQt5 UI code generator 5.5.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WriteTab(object):
    def setupUi(self, WriteTab):
        WriteTab.setObjectName("WriteTab")
        WriteTab.resize(403, 303)
        self.verticalLayout = QtWidgets.QVBoxLayout(WriteTab)
        self.verticalLayout.setObjectName("verticalLayout")
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.textNavigator = TextNavigator(WriteTab)
        self.textNavigator.setObjectName("textNavigator")
        self.horizontalLayout.addWidget(self.textNavigator)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem)
        self.quickSearch = QuickSearch(WriteTab)
        self.quickSearch.setObjectName("quickSearch")
        self.horizontalLayout.addWidget(self.quickSearch)
        self.verticalLayout.addLayout(self.horizontalLayout)
        self.writeTabWritingZone = WritingZone(WriteTab)
        self.writeTabWritingZone.setObjectName("writeTabWritingZone")
        self.verticalLayout.addWidget(self.writeTabWritingZone)
        self.bottomWidget = QtWidgets.QWidget(WriteTab)
        self.bottomWidget.setObjectName("bottomWidget")
        self.verticalLayout.addWidget(self.bottomWidget)

        self.retranslateUi(WriteTab)
        QtCore.QMetaObject.connectSlotsByName(WriteTab)

    def retranslateUi(self, WriteTab):

        WriteTab.setWindowTitle(_("Form"))

from .quick_search import QuickSearch
from .text_navigator import TextNavigator
from .writingzone.writing_zone import WritingZone
