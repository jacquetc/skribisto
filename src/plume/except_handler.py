from PyQt5 import QtCore
from PyQt5.QtWidgets import QMessageBox
from PyQt5.QtGui import QGuiApplication

import sys
import traceback

class ExceptionHandler(QtCore.QObject):

    errorSignal = QtCore.pyqtSignal()
    silentSignal = QtCore.pyqtSignal()

    def __init__(self):
        super(ExceptionHandler, self).__init__()

    def handler(self, exctype, value, traceback_):
        self.errorSignal.emit()
        print("ERROR CAPTURED")
        if QGuiApplication.instance() is not None:
            box = QMessageBox()
            tb = traceback.format_list(traceback.extract_tb(traceback_))
            box.setWindowTitle(str(value))
            box.setText("".join(tb))
            box.exec_()
        else:
            print(exctype, value, traceback_)

        sys._excepthook(exctype, value, traceback_)

exceptionHandler = ExceptionHandler()
sys._excepthook = sys.excepthook
sys.excepthook = exceptionHandler.handler
