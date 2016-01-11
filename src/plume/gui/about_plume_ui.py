# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file '/home/jan-willem/Werkmap/Python/plume-creator/src/plume/gui/about_plume.ui'
#
# Created by: PyQt5 UI code generator 5.4.2
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_AboutPlume(object):
    def setupUi(self, AboutPlume):
        AboutPlume.setObjectName("AboutPlume")
        AboutPlume.resize(400, 300)
        self.verticalLayout = QtWidgets.QVBoxLayout(AboutPlume)
        self.verticalLayout.setObjectName("verticalLayout")
        self.tabWidget = QtWidgets.QTabWidget(AboutPlume)
        self.tabWidget.setObjectName("tabWidget")
        self.tab_about = QtWidgets.QWidget()
        self.tab_about.setObjectName("tab_about")
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(self.tab_about)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.frame_logo = QtWidgets.QFrame(self.tab_about)
        self.frame_logo.setMaximumSize(QtCore.QSize(16777215, 50))
        self.frame_logo.setObjectName("frame_logo")
        self.horizontalLayout = QtWidgets.QHBoxLayout(self.frame_logo)
        self.horizontalLayout.setContentsMargins(-1, 0, -1, 0)
        self.horizontalLayout.setObjectName("horizontalLayout")
        self.label_logo = QtWidgets.QLabel(self.frame_logo)
        self.label_logo.setMaximumSize(QtCore.QSize(48, 48))
        self.label_logo.setText("")
        self.label_logo.setScaledContents(True)
        self.label_logo.setObjectName("label_logo")
        self.horizontalLayout.addWidget(self.label_logo)
        self.verticalLayout_4.addWidget(self.frame_logo)
        self.label_about = QtWidgets.QLabel(self.tab_about)
        self.label_about.setText("")
        self.label_about.setObjectName("label_about")
        self.verticalLayout_4.addWidget(self.label_about)
        self.tabWidget.addTab(self.tab_about, "")
        self.tab_credits = QtWidgets.QWidget()
        self.tab_credits.setObjectName("tab_credits")
        self.verticalLayout_2 = QtWidgets.QVBoxLayout(self.tab_credits)
        self.verticalLayout_2.setObjectName("verticalLayout_2")
        self.browser_credits = QtWidgets.QTextBrowser(self.tab_credits)
        self.browser_credits.setObjectName("browser_credits")
        self.verticalLayout_2.addWidget(self.browser_credits)
        self.tabWidget.addTab(self.tab_credits, "")
        self.tab_license = QtWidgets.QWidget()
        self.tab_license.setObjectName("tab_license")
        self.verticalLayout_3 = QtWidgets.QVBoxLayout(self.tab_license)
        self.verticalLayout_3.setObjectName("verticalLayout_3")
        self.browser_license = QtWidgets.QTextBrowser(self.tab_license)
        self.browser_license.setObjectName("browser_license")
        self.verticalLayout_3.addWidget(self.browser_license)
        self.tabWidget.addTab(self.tab_license, "")
        self.verticalLayout.addWidget(self.tabWidget)
        self.buttonBox = QtWidgets.QDialogButtonBox(AboutPlume)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Ok)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(AboutPlume)
        self.tabWidget.setCurrentIndex(0)
        self.buttonBox.accepted.connect(AboutPlume.accept)
        self.buttonBox.rejected.connect(AboutPlume.reject)
        QtCore.QMetaObject.connectSlotsByName(AboutPlume)

    def retranslateUi(self, AboutPlume):

        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_about), _("About"))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_credits), _("Credits"))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_license), _("License"))

