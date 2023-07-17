# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'mainwindow.ui'
##
## Created by: Qt User Interface Compiler version 6.5.1
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QAction, QBrush, QColor, QConicalGradient,
    QCursor, QFont, QFontDatabase, QGradient,
    QIcon, QImage, QKeySequence, QLinearGradient,
    QPainter, QPalette, QPixmap, QRadialGradient,
    QTransform)
from PySide6.QtWidgets import (QApplication, QHBoxLayout, QListWidget, QListWidgetItem,
    QMainWindow, QMenu, QMenuBar, QPushButton,
    QSizePolicy, QStatusBar, QTextEdit, QToolButton,
    QVBoxLayout, QWidget)

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(800, 600)
        self.actionload = QAction(MainWindow)
        self.actionload.setObjectName(u"actionload")
        self.actionsave_as = QAction(MainWindow)
        self.actionsave_as.setObjectName(u"actionsave_as")
        self.actionsave = QAction(MainWindow)
        self.actionsave.setObjectName(u"actionsave")
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.verticalLayout = QVBoxLayout(self.centralwidget)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.horizontalLayout = QHBoxLayout()
        self.horizontalLayout.setObjectName(u"horizontalLayout")
        self.undoToolButton = QToolButton(self.centralwidget)
        self.undoToolButton.setObjectName(u"undoToolButton")

        self.horizontalLayout.addWidget(self.undoToolButton)

        self.redoToolButton = QToolButton(self.centralwidget)
        self.redoToolButton.setObjectName(u"redoToolButton")

        self.horizontalLayout.addWidget(self.redoToolButton)


        self.verticalLayout.addLayout(self.horizontalLayout)

        self.horizontalLayout_2 = QHBoxLayout()
        self.horizontalLayout_2.setObjectName(u"horizontalLayout_2")
        self.loadSystemPushButton = QPushButton(self.centralwidget)
        self.loadSystemPushButton.setObjectName(u"loadSystemPushButton")

        self.horizontalLayout_2.addWidget(self.loadSystemPushButton)

        self.saveSystemPushButton = QPushButton(self.centralwidget)
        self.saveSystemPushButton.setObjectName(u"saveSystemPushButton")

        self.horizontalLayout_2.addWidget(self.saveSystemPushButton)


        self.verticalLayout.addLayout(self.horizontalLayout_2)

        self.addAsyncPushButton = QPushButton(self.centralwidget)
        self.addAsyncPushButton.setObjectName(u"addAsyncPushButton")

        self.verticalLayout.addWidget(self.addAsyncPushButton)

        self.removePushButton = QPushButton(self.centralwidget)
        self.removePushButton.setObjectName(u"removePushButton")

        self.verticalLayout.addWidget(self.removePushButton)

        self.listWidget = QListWidget(self.centralwidget)
        self.listWidget.setObjectName(u"listWidget")

        self.verticalLayout.addWidget(self.listWidget)

        self.textEdit = QTextEdit(self.centralwidget)
        self.textEdit.setObjectName(u"textEdit")

        self.verticalLayout.addWidget(self.textEdit)

        MainWindow.setCentralWidget(self.centralwidget)
        self.menubar = QMenuBar(MainWindow)
        self.menubar.setObjectName(u"menubar")
        self.menubar.setGeometry(QRect(0, 0, 800, 37))
        self.menuFile = QMenu(self.menubar)
        self.menuFile.setObjectName(u"menuFile")
        MainWindow.setMenuBar(self.menubar)
        self.statusbar = QStatusBar(MainWindow)
        self.statusbar.setObjectName(u"statusbar")
        MainWindow.setStatusBar(self.statusbar)

        self.menubar.addAction(self.menuFile.menuAction())
        self.menuFile.addAction(self.actionload)
        self.menuFile.addAction(self.actionsave_as)
        self.menuFile.addAction(self.actionsave)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"MainWindow", None))
        self.actionload.setText(QCoreApplication.translate("MainWindow", u"load", None))
        self.actionsave_as.setText(QCoreApplication.translate("MainWindow", u"save as", None))
        self.actionsave.setText(QCoreApplication.translate("MainWindow", u"save", None))
        self.undoToolButton.setText(QCoreApplication.translate("MainWindow", u"Undo", None))
        self.redoToolButton.setText(QCoreApplication.translate("MainWindow", u"Redo", None))
        self.loadSystemPushButton.setText(QCoreApplication.translate("MainWindow", u"Load", None))
        self.saveSystemPushButton.setText(QCoreApplication.translate("MainWindow", u"Save", None))
        self.addAsyncPushButton.setText(QCoreApplication.translate("MainWindow", u"Add Async", None))
        self.removePushButton.setText(QCoreApplication.translate("MainWindow", u"Remove Async", None))
        self.menuFile.setTitle(QCoreApplication.translate("MainWindow", u"File", None))
    # retranslateUi

