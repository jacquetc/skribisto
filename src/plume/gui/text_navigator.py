from PyQt5.QtWidgets import QWidget
from .text_navigator_ui import Ui_TextNavigator


class TextNavigator(QWidget):
    '''
    TextNavigator
    '''
    
    def __init__(self, parent=None):
        super(TextNavigator, self).__init__(parent=parent)
        self.ui = Ui_TextNavigator()
        self.ui.setupUi(self)
