from PyQt5.QtCore import QSortFilterProxyModel, QByteArray
from gui.models.note_list_model import NoteListModel

class NoteListProxyModel(QSortFilterProxyModel):
    def __init__(self,  parent=None):
        super(NoteListProxyModel, self).__init__(parent)

        self.setDynamicSortFilter(False)
        self.setFilterKeyColumn(0)
        self._current_sheet_id = -1
        self._current_project_id = -1

    def columnCount(self, parent=None):
        return 1

    def filterAcceptsRow(self, sourceRow, sourceParent):

        #get source-model index for current row
        sourceIndex = self.sourceModel().index(sourceRow, self.filterKeyColumn(), sourceParent)

        if sourceIndex.isValid():
            #check current index itself :
            sheet_code = self.sourceModel().data(sourceIndex, NoteListModel.SheetCodeRole)
            project_id = self.sourceModel().data(sourceIndex, NoteListModel.ProjectIdRole)
            if self._current_sheet_id == sheet_code and self._current_project_id == project_id:
                return True
            return False

        return False
        #parent call for initial behaviour
 #       return QSortFilterProxyModel.filterAcceptsRow(self, sourceRow, sourceParent)

    def filterNoteBySheet(self, project_id:int, sheet_id:int):

        self._current_sheet_id = sheet_id
        self._current_project_id = project_id
        self.invalidateFilter()


    def roleNames(self):
        roles = {}
        roles[NoteListModel.IdRole] = QByteArray().append("id")
        roles[NoteListModel.TitleRole] = QByteArray().append("title")
        roles[NoteListModel.ContentRole] = QByteArray().append("content")
        return roles

