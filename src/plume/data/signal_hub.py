from PyQt5.QtCore import QObject, pyqtSignal

class SignalHub(QObject):
    item_value_returned = pyqtSignal(int, int, 'QString', 'QVariant', name='item_value_returned')

    def __init__(self, parent):
        super(SignalHub, self).__init__(parent)
