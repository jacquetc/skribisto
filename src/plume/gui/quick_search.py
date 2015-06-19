from PyQt5.QtWidgets import QWidget
from .quick_search_ui import Ui_QuickSearch


class QuickSearch(QWidget):

    '''
    QuickSearch
    '''

    def __init__(self, parent=None):
        super(QuickSearch, self).__init__(parent=parent)
        self.ui = Ui_QuickSearch()
        self.ui.setupUi(self)
