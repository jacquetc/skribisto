from PyQt5.QtCore import QObject, pyqtSignal


class SignalHub(QObject):
    item_value_returned = pyqtSignal(int, int, 'QString', object, name='item_value_returned')
    item_value_changed = pyqtSignal(int, int, 'QString', object, name='item_value_changed')

    error_sent = pyqtSignal('QString', name='error_sent')

    def __init__(self, parent):
        super(SignalHub, self).__init__(parent)
