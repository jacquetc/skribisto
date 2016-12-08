'''
Created on 26 may 2015

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QSortFilterProxyModel, pyqtSlot, QModelIndex, QVariant, Qt
from PyQt5.QtGui import QBrush, QColor
from gui.models.tree_model import TreeItem

class WriteWastebinProxyModel(QSortFilterProxyModel):

    '''
    WriteWastebinProxyModel
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(WriteWastebinProxyModel, self).__init__(parent)
        self._root_node = TreeItem()

    def data(self, index, role):

        row = index.row()
        col = index.column()

        if not index.isValid():
            return QVariant()

        node = self.sourceModel().node_from_index(self.mapToSource(index))

        if role == Qt.ForegroundRole and col == 0 and node.deleted == False:
            return QBrush(QColor(Qt.darkGreen))

        return QSortFilterProxyModel.data(self, index, role)

    def filterAcceptsRow(self, source_row, source_parent):

        # get source model index for current row


        source_index = self.sourceModel().index(source_row, self.filterKeyColumn(), source_parent)
        if source_index.isValid():

            # if any of children matches the filter,
            # then current index matches the filter as well

            row_count = self.sourceModel().rowCount(source_index)
            for i in range(row_count):
                self.filterAcceptsRow(i, source_index)

            # check current index itself:

            deleted = source_index.data(self.sourceModel().DeletedRole)
            if deleted:
                key = self.sourceModel().data(source_index, self.filterRole())
                self.filterRegExp().indexIn(key)
                if self.filterRegExp().captureCount() > 0:
                    return True




        # parent call for initial behaviour
        return QSortFilterProxyModel.filterAcceptsRow(self, source_row, source_parent)
