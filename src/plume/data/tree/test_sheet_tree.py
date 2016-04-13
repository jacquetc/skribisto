from .sheet_tree import SheetTree
from ..importer import Importer


class TestSheetTree:

    def setup_class(self):
        _sqlite_db = Importer.create_sqlite_db_from("SQLITE", '../../../../resources/plume_test_project.sqlite')
        self.tree = SheetTree(_sqlite_db)

    def test_read_first(self):
        assert self.tree.get_title(1) == "first"


