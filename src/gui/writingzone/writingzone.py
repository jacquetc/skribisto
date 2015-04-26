from PyQt5.QtWidgets import QWidget,  QTextEdit, QGridLayout, QMenu, QApplication
from ..common import DataAndCoreSetter


class WritingZone(QWidget, DataAndCoreSetter):
    
    def __init__(self, parent=None, data=None, core=None):
        super(QWidget, self).__init__(parent=parent, data=data, core=core)
        
        text_edit = RichTextEdit(self, data, core)
        grid_layout = QGridLayout()
        grid_layout.addWidget(text_edit)
        self.setLayout(grid_layout)
        


class RichTextEdit(QTextEdit, DataAndCoreSetter):
    
    def __init__(self, parent=None, data=None, core=None):
        super(QTextEdit, self).__init__(parent=parent, data=data, core=core)
        
    def contextMenuEvent(self, event):
#        self.popMenu = QtWidgets.QMenu(self)
#        self.popMenu.addAction(QtWidgets.QAction('test0', None))
#        self.popMenu.addAction(QtWidgets.QAction('test1', None))
#        self.popMenu.addSeparator()
#        self.popMenu.addAction(QtWidgets.QAction('test2', None))
#        self.popMenu.exec_(event.globalPos())

        menu = QMenu(self)
        quitAction = menu.addAction("Quit")
        action = menu.exec_(self.mapToGlobal(event.pos()))
        if action == quitAction:
            QApplication.quit()
