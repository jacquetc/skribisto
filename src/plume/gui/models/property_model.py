'''
Created on 17 february 2016

@author:  Cyril Jacquet
'''

from PyQt5.Qt import QAbstractItemModel, QVariant, QModelIndex
from PyQt5.QtCore import Qt, QObject

class PropertyModel(QAbstractItemModel):
    '''
    Tree
    '''

    def __init__(self, table_name: str, id_name: str, parent: QObject, project_id: int):
        super(PropertyModel, self).__init__(parent)

        self.IdRole = Qt.UserRole
        self.TitleRole = Qt.UserRole + 1
        self.ContentRole = Qt.UserRole + 2
        self.SortOrderRole = Qt.UserRole + 3
        self.IndentRole = Qt.UserRole + 4
        self.DateCreatedRole = Qt.UserRole + 5
        self.DateUpdatedRole = Qt.UserRole + 6
        self.DateContentRole = Qt.UserRole + 7
        self.DeletedRole = Qt.UserRole + 8
        self.VersionRole = Qt.UserRole + 9
        self.SynopsisRole = Qt.UserRole + 10
        self.SheetCodeRole = Qt.UserRole + 11