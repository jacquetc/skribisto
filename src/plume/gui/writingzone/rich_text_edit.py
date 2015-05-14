'''
Created on 11 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QTextEdit, QMenu
from PyQt5.QtCore import pyqtSignal

class RichTextEdit(QTextEdit):
    
    size_changed = pyqtSignal('QSize', name='size_changed')
    
    def __init__(self, parent=None):
        super(QTextEdit, self).__init__(parent=parent)
        
    def contextMenuEvent(self, event):
#        self.popMenu = QtWidgets.QMenu(self)
#        self.popMenu.addAction(QtWidgets.QAction('test0', None))
#        self.popMenu.addAction(QtWidgets.QAction('test1', None))
#        self.popMenu.addSeparator()
#        self.popMenu.addAction(QtWidgets.QAction('test2', None))
#        self.popMenu.exec_(event.globalPos())

        menu = QMenu(self)
        quitAction = menu.addAction(_("Quit"))
        action = menu.exec_(self.mapToGlobal(event.pos()))
        if action == quitAction:
            pass
            #QApplication.quit()
            
    def resizeEvent(self, event):
        self.size_changed.emit(event.size())
        return QTextEdit.resizeEvent(self, event)
