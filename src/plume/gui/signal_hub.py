'''
Created on 27 mai 2016

@author: cyril
'''


from PyQt5.QtCore import pyqtSignal, QObject
from .start_window_ui import Ui_StartWindow


class SignalHub(QObject):

    apply_settings_widely_sent = pyqtSignal(name = "apply_settings_widely_sent")

    def __init__(self, parent=None):
        super(SignalHub, self).__init__(parent)