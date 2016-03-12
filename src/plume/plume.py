import ui_converter
import qrc_converter
from PyQt5.QtWidgets import QApplication
from data.data import Data
from data import subscriber
from gui.main_window import MainWindow
from gui import cfg, plugins,  pics_rc
import sys

if __name__ == '__main__':

    app = QApplication(sys.argv)

    # import is placed here because handler's dialog needs QApplication :
    import except_handler
    app.setApplicationName('Plume Creator')
    app.setApplicationVersion('1.5.0-alpha')
    app.setOrganizationDomain('http://www.plume-creator.eu')

    data = Data()
    cfg.data = data
    cfg.data_subscriber = subscriber
    cfg.gui_plugins = plugins.Plugins()

    window = MainWindow()
    arguments = sys.argv
    del arguments[0]
    window.set_application_arguments(arguments)  # load project
    window.show()

    sys.exit(app.exec_())
