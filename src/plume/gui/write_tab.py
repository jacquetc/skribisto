from PyQt5.QtWidgets import QWidget
from .write_tab_ui import Ui_WriteTab


class WriteTab(QWidget):

    '''
    WriteTab
    '''

    def __init__(self, parent=None):
        super(WriteTab, self).__init__(parent=parent)
        self.ui = Ui_WriteTab()
        self.ui.setupUi(self)
