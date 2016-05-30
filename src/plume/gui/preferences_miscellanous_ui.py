# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/cyril/Devel/plume/plume-creator/src/plume/gui/preferences_miscellanous.ui'
#
# Created by: PyQt5 UI code generator 5.5.1
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_MiscellanousPage(object):
    def setupUi(self, MiscellanousPage):
        MiscellanousPage.setObjectName("MiscellanousPage")
        MiscellanousPage.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(MiscellanousPage)
        self.verticalLayout.setObjectName("verticalLayout")
        self.formLayout = QtWidgets.QFormLayout()
        self.formLayout.setObjectName("formLayout")
        self.groupBox = QtWidgets.QGroupBox(MiscellanousPage)
        self.groupBox.setObjectName("groupBox")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.groupBox)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.dev_mode_checkBox = QtWidgets.QCheckBox(self.groupBox)
        self.dev_mode_checkBox.setObjectName("dev_mode_checkBox")
        self.verticalLayout_2.addWidget(self.dev_mode_checkBox)
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.groupBox)
        self.verticalLayout.addLayout(self.formLayout)

        self.retranslateUi(MiscellanousPage)
        QtCore.QMetaObject.connectSlotsByName(MiscellanousPage)

    def retranslateUi(self, MiscellanousPage):

        MiscellanousPage.setWindowTitle(_("Form"))
        self.groupBox.setTitle(_("GroupBox"))
        self.dev_mode_checkBox.setText(_("Developer mode"))

