from PyQt5.QtCore import QObject, pyqtSignal
from . import cfg

class BaseQObject(QObject):

    error_sent = pyqtSignal("QString", name="error_sent")
    #use this one to send error from a QObject

    def __init__(self, parent = None):
        super(BaseQObject, self).__init__(parent)

        self.error_sent.connect(cfg.data.signal_hub.send_error)
