import ui_converter
import qrc_converter
from PyQt5.Qt import QApplication
from core.core import Core
from data.data import Data
import sys


def launch_gui(core):
    from gui import gui
    gui = gui.Gui(core)
    gui.init_gui()

    return gui

if __name__ == '__main__':

    app = QApplication(sys.argv)
    import except_handler
    app.setApplicationName('Plume Creator')
    app.setApplicationVersion('1.5.0-alpha')
    app.setOrganizationDomain('http://www.plume-creator.eu')
    data = Data()
    core = Core(app, data)

    gui = launch_gui(core)

    sys.exit(app.exec_())
