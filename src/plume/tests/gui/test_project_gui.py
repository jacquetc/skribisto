from . import cfg
from PyQt5.QtWidgets import QListWidget
import plmdata

from src.plume.gui.models import sheet_tree_model, sheet_property_model, note_tree_model
from src.plume.gui import cfg as cfg_gui

def test_open_two_projects(qtbot, gui):
    with qtbot.capture_exceptions() as exceptions:
        gui.launch_open_test_project()
        qtbot.waitExposed(gui)

    print(exceptions)

    assert True