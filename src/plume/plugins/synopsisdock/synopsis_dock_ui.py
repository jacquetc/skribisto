# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/workspace_eric/plume-creator/src/plume/plugins/synopsisdock/synopsis_dock.ui'
#
# Created by: PyQt5 UI code generator 5.4.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_SynopsisDock(object):
    def setupUi(self, SynopsisDock):
        SynopsisDock.setObjectName("SynopsisDock")
        SynopsisDock.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(SynopsisDock)
        self.verticalLayout.setObjectName("verticalLayout")
        self.topWidget = QtWidgets.QWidget(SynopsisDock)
        self.topWidget.setObjectName("topWidget")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.topWidget)
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.verticalLayout.addWidget(self.topWidget)
        self.writingZone = WritingZone(SynopsisDock)
        self.writingZone.setEnabled(True)
        self.writingZone.setObjectName("writingZone")
        self.verticalLayout.addWidget(self.writingZone)

        self.retranslateUi(SynopsisDock)
        QtCore.QMetaObject.connectSlotsByName(SynopsisDock)

    def retranslateUi(self, SynopsisDock):

        SynopsisDock.setWindowTitle(_("Form"))

from gui.writingzone.writing_zone import WritingZone
