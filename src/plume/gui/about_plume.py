# -*- coding: utf-8 -*-
'''
Created on 27 augustus 2015

@author: Jan-Willem van der Veen
'''

from PyQt5.QtWidgets import QDialog, QApplication
from PyQt5.QtCore import QDate, QUrl
from PyQt5.QtGui import QPixmap
from .about_plume_ui import Ui_AboutPlume


class AboutPlume(QDialog):
    def __init__(self, parent=None):
        super(AboutPlume, self).__init__(parent)
        self.ui = Ui_AboutPlume()
        self.ui.setupUi(self)

        plumeDescr = _('A Project Manager and Rich Text Editor for Writers.')
        plumeName = QApplication.applicationName()
        plumeVersion = QApplication.applicationVersion()
        plumeDomain = QApplication.organizationDomain()

        self.setWindowTitle(_("About ") + plumeName)

        self.ui.label_logo.setPixmap(QPixmap(":/pics/plume-creator.png"))

        self.ui.label_about.setText("<p><center><b>" + plumeName + "</b></center></p>"
        "<p><center><b>" + plumeDescr + "</b></center></p>"
        "<p><center>Version " + plumeVersion + "</center></p>"
        "<p><center><address><a href=" + plumeDomain + ">"
        + plumeDomain + "</a></address></center></p>"
        "<p><center>Copyright (C)" + str(QDate.currentDate().year()) + ", "
        + _("created by") + " Cyril Jacquet</center></p>"
        "<p><center>cyril.jacquet@plume-creator.eu</center></p>")

        self.ui.browser_credits.setSource(QUrl.fromLocalFile("./gui/html/credits.html"))
        self.ui.browser_license.setSource(QUrl.fromLocalFile("./gui/html/license.html"))
