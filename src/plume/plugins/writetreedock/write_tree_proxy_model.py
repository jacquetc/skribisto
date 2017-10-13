'''
Created on 26 may 2015

@author:  Cyril Jacquet
'''

from PyQt5.QtCore import QSortFilterProxyModel, pyqtSlot


class WriteTreeProxyModel(QSortFilterProxyModel):

    '''
    WriteTreeProxyModel
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(WriteTreeProxyModel, self).__init__(parent)

    def filterAcceptsRow(self, source_row, source_parent):

        # get source model index for current row


        source_index = self.sourceModel().index(source_row, self.filterKeyColumn(), source_parent)
        if source_index.isValid():

            # if any of children matches the filter,
            # then current index matches the filter as well

            row_count = self.sourceModel().rowCount(source_index)
            for i in range(row_count):
                is_matching = self.filterAcceptsRow(i, source_index)
                if is_matching:
                    return True


            # check current index itself:

            deleted = source_index.data(self.sourceModel().DeletedRole)
            if deleted == True:
                return False
            key = self.sourceModel().data(source_index, self.filterRole())
            self.filterRegExp().indexIn(key)
            if self.filterRegExp().captureCount() > 0:
                return True



        # parent call for initial behaviour
        return QSortFilterProxyModel.filterAcceptsRow(self, source_row, source_parent)


