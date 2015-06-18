from PyQt5.QtWidgets import QToolBar
from PyQt5.QtCore import Qt
# from ..cfg import core


class ToolBar(QToolBar):

    def __init__(self, parent=None):
        super(ToolBar, self).__init__(parent=parent)
        self._action_set = None

        self.setFloatable(False)
        self.setMovable(False)
        self.setOrientation(Qt.Vertical)

    @property
    def action_set(self):
        return self._action_set

    @action_set.setter
    def action_set(self,  action_set):
        self._action_set = action_set

        self.addAction(self.action_set.actionCopy)
        self.addAction(self.action_set.actionCut)
        self.addAction(self.action_set.actionPaste)
        self.addSeparator()
        self.addAction(self.action_set.actionBold)
        self.addAction(self.action_set.actionItalic)
        self.addAction(self.action_set.actionStrikethrough)
        self.addAction(self.action_set.actionUnderline)
        self.addSeparator()
        for action in self.action_set.added_actions_list:
            self.addAction(action)

    def set_action_set(self,  action_set):
        self.action_set = action_set

#    def addAction(self,  action):
#
#        QToolBar.addAction(action)
#
