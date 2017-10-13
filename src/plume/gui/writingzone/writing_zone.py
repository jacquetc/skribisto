from PyQt5.QtWidgets import QWidget, QGridLayout,  QSizePolicy
from PyQt5.Qt import QTimer
from .writing_zone_ui import Ui_WritingZone


class WritingZone(QWidget):

    def __init__(self, parent=None):
        super(WritingZone, self).__init__(parent=parent)
        self.ui = Ui_WritingZone()
        self.ui.setupUi(self)
        # self.ui.minimap_old.text_edit = self.ui.richTextEdit
        self.ui.minimap_old.hide()
        self.ui.richTextEdit.setFixedWidth(500)

        # the necessary for minimap :
        self.ui.richTextEdit.size_changed.connect(self.ui.minimap.update_size)
        self.ui.minimap.text_edit = self.ui.richTextEdit

        # connect textedit to scrollbar
        baseScrollBar = self.ui.richTextEdit.verticalScrollBar()
        baseScrollBar.rangeChanged.connect(self._set_scrollBar_range)
        baseScrollBar.valueChanged.connect(self.ui.verticalScrollBar.setValue)
        self.ui.verticalScrollBar.valueChanged.connect(baseScrollBar.setValue)
        self.ui.richTextEdit.verticalScrollBar().hide()

        # private
        self._has_minimap = False
        self._has_scrollbar = False
        self._has_side_tool_bar = True
        self._is_resizable = False

        # initial setup
        self.has_minimap = False
        self.has_scrollbar = False
        self.has_side_tool_bar = True
        self.is_resizable = False

        self._init_actions()

        self.ui.sizeHandle.size_handle_moved.connect(self.widen_textedit)

    def _init_actions(self):

        class ActionSet():

            def __init__(self,  basic_actions_list):
                super(ActionSet, self).__init__()
                self._basic_actions_list = basic_actions_list
                self.added_actions_list = []
                self._action_set_setters = []
                for action in self._basic_actions_list:
                    setattr(self, action.objectName(), action)

            def add_action(self,  action):
                self.added_actions_list.append(action)
                setattr(self, action.objectName(), action)

            def subscribe_action_set_setter(self,  property_func):
                self._action_set_setters.append(property_func)

            def update_action_set_setters(self):
                for property_func in self._action_set_setters:
                    property_func(self)

        # actions
        action_list = [self.ui.actionCopy,
                       self.ui.actionCut,
                       self.ui.actionPaste,
                       self.ui.actionBold,
                       self.ui.actionItalic,
                       self.ui.actionStrikethrough,
                       self.ui.actionUnderline
                       ]
        self._action_set = ActionSet(action_list)
        self._action_set.subscribe_action_set_setter(
            self.ui.toolBar.set_action_set)
        self._action_set.subscribe_action_set_setter(
            self.ui.richTextEdit.set_action_set)
        self._action_set.update_action_set_setters()


#        test :
        self._action_set.add_action(self.ui.actionPrint_directly)

    def set_rich_text(self, text):
        self.ui.richTextEdit.setText(text)

    @property
    def text_edit(self):
        return self.ui.richTextEdit

    def _set_scrollBar_range(self, min_, max_):

        self.ui.verticalScrollBar.setMinimum(min_)
        self.ui.verticalScrollBar.setMaximum(max_)

        if min_ == 0 and max_ == 0:
            self.ui.verticalScrollBar.hide()
        else:
            self.ui.verticalScrollBar.show()

    @property
    def has_minimap(self):
        return self._has_minimap

    @has_minimap.setter
    def has_minimap(self,  value):
        self._has_minimap = value
        if value is True:
            self.ui.minimap.show()
            self.ui.minimap.set_activated(True)
        else:
            self.ui.minimap.hide()
            self.ui.minimap.set_activated(False)

    @property
    def has_scrollbar(self):
        return self._has_scrollbar

    @has_scrollbar.setter
    def has_scrollbar(self,  value):
        self._has_scrollbar = value
        if value is True:
            self.ui.verticalScrollBar.show()
        else:
            self.ui.verticalScrollBar.hide()

    @property
    def has_side_tool_bar(self):
        return self._has_side_tool_bar

    @has_side_tool_bar.setter
    def has_side_tool_bar(self,  value):
        self._has_side_tool_bar = value
        if value is True:
            self.ui.toolBar.show()
        else:
            self.ui.toolBar.hide()

    @property
    def is_resizable(self):
        return self._has_size_handle

    @is_resizable.setter
    def is_resizable(self,  value):
        self._is_resizable = value
        if value is True:
            self.ui.sizeHandle.show()
            self.ui.sizeHorizontalSpacer_left.show()
            self.ui.sizeHorizontalSpacer_right.show()
            # TODO: fetch value from settings file (use objectName)
            self.ui.richTextEdit.setFixedWidth(500)
        else:
            self.ui.sizeHandle.hide()
            self.ui.sizeHorizontalSpacer_left.hide()
            self.ui.sizeHorizontalSpacer_right.hide()
            # workaround for QWIDGETSIZE_MAX
            self.ui.richTextEdit.setFixedWidth(((1 << 24) - 1))

    def add_action(self,  action):
        self._action_set.add_action(action)

    def widen_textedit(self, diff: int):
        self.ui.richTextEdit.setMaximumWidth(self.ui.richTextEdit.width() + diff * 2)
