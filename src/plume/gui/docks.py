'''
Created on 7 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QDockWidget, QWidget
from PyQt5.Qt import pyqtSlot

class DockTemplate(QDockWidget):
    '''
    DockTemplate
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(DockTemplate, self).__init__(parent=None)
        
        title_widget = DockTitleWidget(self, self)
        self.setTitleBarWidget(title_widget)



from .dock_title_bar_ui import Ui_DockTitleBar


class DockTitleWidget(QWidget):
    '''
    DockTitleWidget
    '''

    def __init__(self, parent_dock, parent=None):
        '''
        Constructor
        '''

        super(DockTitleWidget, self).__init__(parent=None)
        self.parent_dock = parent_dock
        ui = Ui_DockTitleBar()
        ui.setupUi(self)
        
    @pyqtSlot()
    def on_closeButton_clicked(self):
        self.parent_dock.close()
