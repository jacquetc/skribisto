from PyQt5.QtWidgets import QWidget, QGridLayout
from ..cfg import core
from .writing_zone_ui import Ui_WritingZone


class WritingZone(QWidget):
    
    def __init__(self, parent=None):
        super(QWidget, self).__init__(parent=parent)
        self.ui = Ui_WritingZone()
        self.ui.setupUi(self)
        self.ui.minimap.text_edit = self.ui.richTextEdit
        

        
    def set_rich_text(self, text):
        self.ui.richTextEdit.setText(text)
            

