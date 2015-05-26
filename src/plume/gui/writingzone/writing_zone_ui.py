# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/gui/writingzone/writing_zone.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_WritingZone(object):
    def setupUi(self, WritingZone):
        WritingZone.setObjectName("WritingZone")
        WritingZone.resize(816, 522)
        self.verticalLayout = QtWidgets.QVBoxLayout(WritingZone)
        self.verticalLayout.setObjectName("verticalLayout")
        self.widget = QtWidgets.QWidget(WritingZone)
        self.widget.setMaximumSize(QtCore.QSize(16777215, 40))
        self.widget.setObjectName("widget")
        self.horizontalLayout_4 = QtWidgets.QHBoxLayout(self.widget)
        self.horizontalLayout_4.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout_4.setObjectName("horizontalLayout_4")
        self.horizontalLayout_2 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_2.setObjectName("horizontalLayout_2")
        self.historyPreviousToolButton = QtWidgets.QToolButton(self.widget)
        self.historyPreviousToolButton.setObjectName("historyPreviousToolButton")
        self.horizontalLayout_2.addWidget(self.historyPreviousToolButton)
        self.historyNextToolButton = QtWidgets.QToolButton(self.widget)
        self.historyNextToolButton.setObjectName("historyNextToolButton")
        self.horizontalLayout_2.addWidget(self.historyNextToolButton)
        self.browserComboBox = QtWidgets.QComboBox(self.widget)
        self.browserComboBox.setObjectName("browserComboBox")
        self.horizontalLayout_2.addWidget(self.browserComboBox)
        self.horizontalLayout_4.addLayout(self.horizontalLayout_2)
        spacerItem = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout_4.addItem(spacerItem)
        self.horizontalLayout_3 = QtWidgets.QHBoxLayout()
        self.horizontalLayout_3.setObjectName("horizontalLayout_3")
        self.findComboBox = QtWidgets.QComboBox(self.widget)
        self.findComboBox.setEditable(True)
        self.findComboBox.setObjectName("findComboBox")
        self.horizontalLayout_3.addWidget(self.findComboBox)
        self.findPreviousToolButton = QtWidgets.QToolButton(self.widget)
        self.findPreviousToolButton.setObjectName("findPreviousToolButton")
        self.horizontalLayout_3.addWidget(self.findPreviousToolButton)
        self.findNextToolButton = QtWidgets.QToolButton(self.widget)
        self.findNextToolButton.setObjectName("findNextToolButton")
        self.horizontalLayout_3.addWidget(self.findNextToolButton)
        self.horizontalLayout_4.addLayout(self.horizontalLayout_3)
        self.verticalLayout.addWidget(self.widget)
        self.horizontalLayout = QtWidgets.QHBoxLayout()
        self.horizontalLayout.setObjectName("horizontalLayout")
        spacerItem1 = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem1)
        self.verticalLayout_2 = QtWidgets.QVBoxLayout()
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        spacerItem2 = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout_2.addItem(spacerItem2)
        self.toolBar = QtWidgets.QWidget(WritingZone)
        self.toolBar.setMinimumSize(QtCore.QSize(20, 0))
        self.toolBar.setObjectName("toolBar")
        self.verticalLayout_2.addWidget(self.toolBar)
        spacerItem3 = QtWidgets.QSpacerItem(20, 40, QtWidgets.QSizePolicy.Minimum, QtWidgets.QSizePolicy.Expanding)
        self.verticalLayout_2.addItem(spacerItem3)
        self.horizontalLayout.addLayout(self.verticalLayout_2)
        self.richTextEdit = RichTextEdit(WritingZone)
        self.richTextEdit.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        self.richTextEdit.setObjectName("richTextEdit")
        self.horizontalLayout.addWidget(self.richTextEdit)
        self.sizehandle = QtWidgets.QWidget(WritingZone)
        self.sizehandle.setMinimumSize(QtCore.QSize(4, 0))
        self.sizehandle.setCursor(QtGui.QCursor(QtCore.Qt.SizeHorCursor))
        self.sizehandle.setMouseTracking(True)
        self.sizehandle.setLocale(QtCore.QLocale(QtCore.QLocale.English, QtCore.QLocale.UnitedStates))
        self.sizehandle.setObjectName("sizehandle")
        self.horizontalLayout.addWidget(self.sizehandle)
        spacerItem4 = QtWidgets.QSpacerItem(40, 20, QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Minimum)
        self.horizontalLayout.addItem(spacerItem4)
        self.minimap = Minimap2(WritingZone)
        self.minimap.setObjectName("minimap")
        self.horizontalLayout.addWidget(self.minimap)
        self.minimap_old = Minimap(WritingZone)
        self.minimap_old.setObjectName("minimap_old")
        self.horizontalLayout.addWidget(self.minimap_old)
        self.verticalScrollBar = QtWidgets.QScrollBar(WritingZone)
        self.verticalScrollBar.setOrientation(QtCore.Qt.Vertical)
        self.verticalScrollBar.setObjectName("verticalScrollBar")
        self.horizontalLayout.addWidget(self.verticalScrollBar)
        self.verticalLayout.addLayout(self.horizontalLayout)

        self.retranslateUi(WritingZone)
        QtCore.QMetaObject.connectSlotsByName(WritingZone)

    def retranslateUi(self, WritingZone):

        WritingZone.setWindowTitle(_("Form"))
        self.historyPreviousToolButton.setText(_("<-"))
        self.historyNextToolButton.setText(_("->"))
        self.findPreviousToolButton.setText(_("<-"))
        self.findNextToolButton.setText(_("->"))

from .minimap import Minimap
from .minimap_text_browser import Minimap2
from .rich_text_edit import RichTextEdit
