'''
Created on 4 juillet 2015

@author: Cyril Jacquet
'''

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import pyqtSlot, Qt, pyqtSignal


class SizeHandle(QWidget):

    '''
    Size handle
    '''
    size_handle_moved = pyqtSignal(int, name="size_handle_moved")


    def __init__(self, parent=0):
        '''
        Constructor
        '''
        super(SizeHandle, self).__init__(parent=parent)



    def mousePressEvent(self, event):
        self.__mousePressPos = None
        self.__mouseMovePos = None
        if event.button() == Qt.LeftButton:
            self.__mousePressPos = event.globalPos()
            self.__mouseMovePos = event.globalPos()

        super(SizeHandle, self).mousePressEvent(event)

    def mouseMoveEvent(self, event):
        if event.buttons() == Qt.LeftButton:
            # adjust offset from clicked point to origin of widget
            currPos = self.mapToGlobal(self.pos())
            globalPos = event.globalPos()
            diff = globalPos - self.__mouseMovePos

            self.size_handle_moved.emit(diff.x())
            # self.move(newPos)

            self.__mouseMovePos = globalPos

        super(SizeHandle, self).mouseMoveEvent(event)

    def mouseReleaseEvent(self, event):
        if self.__mousePressPos is not None:
            moved = event.globalPos() - self.__mousePressPos
            if moved.manhattanLength() > 3:
                event.ignore()
                return

        super(SizeHandle, self).mouseReleaseEvent(event)