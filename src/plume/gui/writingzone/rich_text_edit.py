'''
Created on 11 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QTextEdit, QMenu
from PyQt5.QtCore import pyqtSignal,  pyqtSlot,  Qt
from PyQt5.QtGui import QTextCharFormat


class RichTextEdit(QTextEdit):

    size_changed = pyqtSignal('QSize', name='size_changed')

    def __init__(self, parent=None):
        super(QTextEdit, self).__init__(parent=parent)

        self.cursorPositionChanged.connect(self.check_font_under_cursor)

    def contextMenuEvent(self, event):
        self.popMenu = QMenu(self)
        self.popMenu.addAction(self.action_set.actionCopy)
        self.popMenu.addAction(self.action_set.actionCut)
        self.popMenu.addAction(self.action_set.actionPaste)
        self.popMenu.addSeparator()
        self.popMenu.addAction(self.action_set.actionBold)
        self.popMenu.addAction(self.action_set.actionItalic)
        self.popMenu.addAction(self.action_set.actionStrikethrough)
        self.popMenu.addAction(self.action_set.actionUnderline)
        self.popMenu.addSeparator()
        for action in self.action_set.added_actions_list:
            self.popMenu.addAction(action)
        self.popMenu.exec_(event.globalPos())


#        menu = QMenu(self)
#        quitAction = menu.addAction(_("Quit"))
#        action = menu.exec_(self.mapToGlobal(event.pos()))
#        if action == quitAction:
#            pass
        # QApplication.quit()

    def resizeEvent(self, event):
        self.size_changed.emit(event.size())
        return QTextEdit.resizeEvent(self, event)

    @property
    def action_set(self):
        return self._action_set

    @action_set.setter
    def action_set(self,  action_set):
        self._action_set = action_set

        if self._action_set.actionCut.receivers(self._action_set.actionCut.triggered) != 1:
            return  # prevent exception if connection not unique...

        self._action_set.actionCut.triggered.connect(
            self._cut, Qt.UniqueConnection)
        self._action_set.actionCopy.triggered.connect(
            self._copy, Qt.UniqueConnection)
        self._action_set.actionPaste.triggered.connect(
            self._paste, Qt.UniqueConnection)
        self._action_set.actionBold.triggered.connect(
            self._bold, Qt.UniqueConnection)
        self._action_set.actionItalic.triggered.connect(
            self._italic, Qt.UniqueConnection)
        self._action_set.actionStrikethrough.triggered.connect(
            self._strikethrough, Qt.UniqueConnection)
        self._action_set.actionUnderline.triggered.connect(
            self._underline, Qt.UniqueConnection)

    def set_action_set(self,  action_set):
        self.action_set = action_set

    @pyqtSlot()
    def _cut(self):
        self.cut()

    @pyqtSlot()
    def _copy(self):
        self.copy()

    @pyqtSlot()
    def _paste(self):
        self.paste()

    @pyqtSlot(bool)
    def _bold(self,  value):
        if value is True:
            self.setFontWeight(75)
        else:
            self.setFontWeight(50)

    @pyqtSlot(bool)
    def _italic(self,  value):
        self.setFontItalic(value)

    @pyqtSlot(bool)
    def _strikethrough(self,  value):
        charFormat = QTextCharFormat()
        charFormat.setFontStrikeOut(value)
        self.mergeCurrentCharFormat(charFormat)

    @pyqtSlot(bool)
    def _underline(self,  value):
        self.setFontUnderline(value)

    def check_font_under_cursor(self):
        cursor = self.textCursor()
        charFormat = cursor.charFormat()
        self.action_set.actionItalic.setChecked(charFormat.fontItalic())
        self.action_set.actionUnderline.setChecked(charFormat.fontUnderline())
        self.action_set.actionStrikethrough.setChecked(
            charFormat.fontStrikeOut())
        if charFormat.fontWeight() > 50:
            self.action_set.actionBold.setChecked(True)
        else:
            self.action_set.actionBold.setChecked(False)
