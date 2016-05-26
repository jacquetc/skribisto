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

        super(WriteTreeProxyModel, self).__init__(parent=None)
        self._current_filter_text = ""

    def node_from_index(self, index):
        return self.sourceModel().node_from_index(self.mapToSource(index))

    @property
    def id_of_last_created_node(self):
        return self.sourceModel().id_of_last_created_node

    def find_index_from_id(self, id_):
        index = self.sourceModel().find_index_from_id(id_)
        return self.mapFromSource(index)

    def insert_child_node(self, parent_index):
        return self.sourceModel().insert_child_node(self.mapToSource(parent_index))

    def insert_node_after(self, parent_index):
        return self.sourceModel().insert_node_by(self.mapToSource(parent_index))

    def remove_node(self, index):
        return self.sourceModel().remove_node(self.mapToSource(index))

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
            print(source_index.data(self.sourceModel().TitleRole) + "  " + str(deleted))
            if deleted == True:
                return False
            key = self.sourceModel().data(source_index, self.filterRole())
            print(self.filterRegExp().indexIn(key))
            self.filterRegExp().indexIn(key)
            if self.filterRegExp().captureCount() > 0:
                return True



        # parent call for initial behaviour
        return QSortFilterProxyModel.filterAcceptsRow(self, source_row, source_parent)



    @pyqtSlot(str)
    def filterByText(self, filter_text: str):

        self._current_filter_text = filter_text
        self.invalidateFilter()


