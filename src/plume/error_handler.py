from PyQt5.QtCore import QObject, pyqtSlot
from .gui import cfg

class ErrorHandler(QObject):

    def __init__(self, parent=None):
        super(ErrorHandler, self).__init__(parent=parent)

        cfg.data.errorHub().errorSent.connect(self.print_error)

    # @pyqtSlot('QString', 'QString', 'QString')
    # def print_error(self, code:str, origin:str, message:str):
    #     print("ERROR : " + code + "/n" + origin + "/n" + message

    @pyqtSlot()
    def print_error(self, error):
        if not error:
            print("error")
        # print("ERROR : " + code + "/n" + origin + "/n" + message)