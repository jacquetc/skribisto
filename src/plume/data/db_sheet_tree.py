'''
Created on 12 nov. 2015

@author:  Cyril Jacquet
'''
import sqlite3
from .db_sheet import DbSheet
from .exceptions import DbErr


########################################################################################################################


class DbSheetTree:
    #
    # A class to allow sheets to be listed, moved, copied etc. Operates on LISTs of sheets. For individual sheet
    # functions see DbSheet
    #

    # Other constants
    RENUM_INT = 1000                    # l_sort_order is renumbered starting with this value, incr by this value.

    def __init__(self, a_db: sqlite3.Connection):
        # Class variables (consts in UPPERCASE). All vars are considered private - use the setter / getter to access.
        self.a_db = a_db                # db connection
        self.a_err = DbErr()            # Error Info Object
        self.i_version = 0              # Document version we are working on
        self.b_commit = True            # auto commit

    def set_commit(self, b_commit: bool):
        #
        # If you are doing several sheet operations (or operations on severla sheets) then you can set commit to
        # False to speed up execution. Sqlite is fast to execute, but slow to commit, so batch commits if you can.
        # (We only make the default True as a fail-safe for users who don't know the rules :) )
        #
        self.b_commit = b_commit

    def get_commit(self):
        return self.b_commit

    def set_version(self, i_version: int):
        #
        # Most methods in this class operate on a single version of the tree. You can set that version here.
        # Ideally this version number should also exist in the version table, but we don't care here - a version is
        # just a number to us.
        #
        self.i_version = i_version

    def get_version(self) -> int:
        return self.i_version



    def renum_all(self):
        #
        # Renumber all non-deleted sheet in this version. DOES NOT COMMIT - Caller should
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            b_deleted = 0
            and l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_rows = a_qry.fetchall()

        i_dest = DbSheetTree.RENUM_INT
        for a_row in a_rows:
            # For each sheet to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_sh = DbSheet(self.a_db, a_row[0], False)
            a_sh.set_sort_order(i_dest)
            i_dest += DbSheetTree.RENUM_INT

        return DbErr.R_OK

    def renum_test(self, i_interval):
        #
        # Renumber all non-deleted sheet in this version using the specifed interval. This function is meant to be for
        # testing only
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            b_deleted = 0
            and l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_rows = a_qry.fetchall()

        i_dest = i_interval
        for a_row in a_rows:
            # For each sheet to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_sh = DbSheet(self.a_db, a_row['l_sheet_id'], False)
            a_sh.set_sort_order(i_dest)
            i_dest += i_interval

        self.a_db.commit()
        return DbErr.R_OK

    def move_list(self, a_list: list, i_after_sheet: int) -> int:
        #
        # Move the list of sheets to after i_after_sheet.
        #
        # If i_after_sheet is 0, then move the sheets to the top. -1 = move sheets to bottom
        #
        # The sheets are moved in list order such at the first in the list ends up as the first after i_after_sheet.
        #
        # To move, we renumber sheets from i_dest + 1. The max sheets we can move is RENUM_INT -1.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type
        if type(i_after_sheet) != int:
            self.a_err.set_status(DbErr.E_INVTYPE, 2, 'Must be int')
            return DbErr.R_ERROR         # Invalid type

        # sanity check. Max move is RENUM_INT -1
        if len(a_list) > (DbSheetTree.RENUM_INT - 1):
            self.a_err.set_status(DbErr.E_INTERNERR, 0, "Can't move more than {0} sheets".format(DbSheetTree.RENUM_INT - 1))
            return DbErr.R_ERROR         # Too many sheets to move

        if i_after_sheet == 0:
            i_dest = 0                  # First sheet order
        elif i_after_sheet == -1:
            i_dest = 2**30              # hopefully greater than last sheet order!
        else:
            a_sh = DbSheet(self.a_db, i_after_sheet, False)
            i_dest = a_sh.get_sort_order()
            if i_dest == DbErr.R_ERROR:
                self.a_err.set_status(DbErr.E_INVPARAM, 2, 'After Sheet ({0}) not found'.format(i_after_sheet))
                return DbErr.R_ERROR         # Invalid value

        # Renumber each sheet to +1 after dest
        i_dest += 1
        for i_sheet_id in a_list:
            a_sh = DbSheet(self.a_db, i_sheet_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Sheet ({0}) not found'.format(i_sheet_id))
                return DbErr.R_ERROR         # Invalid value

            a_sh.set_sort_order(i_dest)
            i_dest += 1

        # Renumber all sheets to restore default spacing
#??? self.renum_all()
        self.a_db.commit()

        return DbErr.R_OK

    def copy_list(self, a_list: list) -> int:
        #
        # Copy the list of sheets. Each sheet is copied directly after its original sheet.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Copy each sheet to +1 after src
        for i_sheet_id in a_list:
            a_sh = DbSheet(self.a_db, i_sheet_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Sheet ({0}) not found'.format(i_sheet_id))
                return DbErr.R_ERROR         # Invalid value
            # Copy
            a_sh.copy('Copy of ')

        # Renumber all sheets to restore default spacing
#??? self.renum_all()
        self.a_db.commit()

        return DbErr.R_OK

    def version_list(self, a_list: list, i_new_version: int) -> int:
        #
        # Adds the list of sheets to the specified version. Does not change the sort order of any sheet. Does not
        # check if the sheet is already in the version, any that are are effectively copies.
        #
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Copy each sheet to +1 after src
        for i_sheet_id in a_list:
            a_sh = DbSheet(self.a_db, i_sheet_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Sheet ({0}) not found'.format(i_sheet_id))
                return DbErr.R_ERROR         # Invalid value
            # Copy
            a_sh.version(i_new_version)

        # Renumber all sheets to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def delete_list(self, a_list: list) -> int:
        #
        # Mark a list of sheets as deleted.
        # This function just sets the deleted flag in the record. Call empty_trash to really delete the records
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Delete each sheet
        for i_sheet_id in a_list:
            a_sh = DbSheet(self.a_db, i_sheet_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Sheet ({0}) not found'.format(i_sheet_id))
                return DbErr.R_ERROR         # Invalid value
            # Delete
            a_sh.delete()

        # Renumber all sheets to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def undelete_list(self, a_list: list) -> int:
        #
        # Mark a list of sheets as undeleted.
        # This function just unsets the deleted flag in the record.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Delete each sheet
        for i_sheet_id in a_list:
            a_sh = DbSheet(self.a_db, i_sheet_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Sheet ({0}) not found'.format(i_sheet_id))
                return DbErr.R_ERROR         # Invalid value
            # Undelete
            a_sh.undelete()

        # Renumber all sheets to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def empty_trash(self) -> int:
        #
        # Permanently remove any deleted sheets for the CURRENT version
        #

        # Delete records
        s_sql = """
        delete from
            tbl_sheet
        where
            b_deleted = 1
            and l_version_code = :ver
            """
        self.a_db.execute(s_sql, {'ver': self.i_version})
        self.a_db.commit()

        return DbErr.R_OK

    def list_sheet(self):
        #
        #  Returns a list of visible sheet ids (i.e. not deleted) for the current version
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            b_deleted = 0
            and l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_sheet_id'])

        return a_list

    def list_trash(self):
        #
        #  Returns a list of deleted sheet ids for the current version
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            b_deleted = 1
            and l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_sheet_id'])

        return a_list

    def list_version(self):
        #
        #  Returns a list of sheet ids for the current version. Probably only useful for testing?
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_sheet_id'])

        return a_list

    def list_all(self):
        #
        #  Returns a list of ALL sheet ids. Probably only useful for testing?
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_sheet_id'])

        return a_list


########################################################################################################################


