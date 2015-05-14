from PyQt5.QtWidgets import QWidget, QGridLayout
from ..cfg import core
from .writing_zone_ui import Ui_WritingZone


class WritingZone(QWidget):
    
    def __init__(self, parent=None):
        super(QWidget, self).__init__(parent=parent)
        self.ui = Ui_WritingZone()
        self.ui.setupUi(self)
        #self.ui.minimap.text_edit = self.ui.richTextEdit
        self.ui.minimap.hide()
        self.ui.richTextEdit.setFixedWidth(500)

        #the necessary for minimap :
        self.ui.richTextEdit.size_changed.connect(self.ui.minimap_graphicsView.update_size)
        self.ui.minimap_graphicsView.text_edit = self.ui.richTextEdit
        
        #connect textedit to scrollbar
        baseScrollBar = self.ui.richTextEdit.verticalScrollBar()
        baseScrollBar.rangeChanged.connect(self._set_scrollBar_range)
        baseScrollBar.valueChanged.connect(self.ui.verticalScrollBar.setValue)         
        self.ui.verticalScrollBar.valueChanged.connect(baseScrollBar.setValue)
        self.ui.richTextEdit.verticalScrollBar().hide()
    
    def set_rich_text(self, text):
        self.ui.richTextEdit.setText(text)
   

    def _set_scrollBar_range(self, min_, max_):

        self.ui.verticalScrollBar.setMinimum(min_)
        self.ui.verticalScrollBar.setMaximum(max_)

        if min_ == 0 and max_ == 0:
            self.ui.verticalScrollBar.hide()
        else:
            self.ui.verticalScrollBar.show()

