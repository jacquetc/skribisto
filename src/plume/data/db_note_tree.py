'''
Created on 12 nov. 2015

@author:  Cyril Jacquet
'''
import sqlite3
from .db_note import DbNote
from .exceptions import DbErr


########################################################################################################################


class DbNoteTree:
    #
    # A class to allow notes to be listed, moved, copied etc. Operates on LISTs of notes. For individual note
    # functions see DbNote
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
        # If you are doing several note operations (or operations on severla notes) then you can set commit to
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
        # Renumber all non-deleted note in this version. DOES NOT COMMIT - Caller should
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
        where
            b_deleted = 0
            and l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_rows = a_qry.fetchall()

        i_dest = DbNoteTree.RENUM_INT
        for a_row in a_rows:
            # For each note to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_sh = DbNote(self.a_db, a_row[0], False)
            a_sh.set_sort_order(i_dest)
            i_dest += DbNoteTree.RENUM_INT

        return DbErr.R_OK

    def renum_test(self, i_interval):
        #
        # Renumber all non-deleted note in this version using the specifed interval. This function is meant to be for
        # testing only
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
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
            # For each note to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_sh = DbNote(self.a_db, a_row['l_note_id'], False)
            a_sh.set_sort_order(i_dest)
            i_dest += i_interval

        self.a_db.commit()
        return DbErr.R_OK

    def move_list(self, a_list: list, i_after_note: int) -> int:
        #
        # Move the list of notes to after i_after_note.
        #
        # If i_after_note is 0, then move the notes to the top. -1 = move notes to bottom
        #
        # The notes are moved in list order such at the first in the list ends up as the first after i_after_note.
        #
        # To move, we renumber notes from i_dest + 1. The max notes we can move is RENUM_INT -1.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type
        if type(i_after_note) != int:
            self.a_err.set_status(DbErr.E_INVTYPE, 2, 'Must be int')
            return DbErr.R_ERROR         # Invalid type

        # sanity check. Max move is RENUM_INT -1
        if len(a_list) > (DbNoteTree.RENUM_INT - 1):
            self.a_err.set_status(DbErr.E_INTERNERR, 0, "Can't move more than {0} notes".format(DbNoteTree.RENUM_INT - 1))
            return DbErr.R_ERROR         # Too many notes to move

        if i_after_note == 0:
            i_dest = 0                  # First note order
        elif i_after_note == -1:
            i_dest = 2**30              # hopefully greater than last note order!
        else:
            a_sh = DbNote(self.a_db, i_after_note, False)
            i_dest = a_sh.get_sort_order()
            if i_dest == DbErr.R_ERROR:
                self.a_err.set_status(DbErr.E_INVPARAM, 2, 'After Note ({0}) not found'.format(i_after_note))
                return DbErr.R_ERROR         # Invalid value

        # Renumber each note to +1 after dest
        i_dest += 1
        for i_note_id in a_list:
            a_sh = DbNote(self.a_db, i_note_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Note ({0}) not found'.format(i_note_id))
                return DbErr.R_ERROR         # Invalid value

            a_sh.set_sort_order(i_dest)
            i_dest += 1

        # Renumber all notes to restore default spacing
#??? self.renum_all()
        self.a_db.commit()

        return DbErr.R_OK

    def copy_list(self, a_list: list) -> int:
        #
        # Copy the list of notes. Each note is copied directly after its original note.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Copy each note to +1 after src
        for i_note_id in a_list:
            a_sh = DbNote(self.a_db, i_note_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Note ({0}) not found'.format(i_note_id))
                return DbErr.R_ERROR         # Invalid value
            # Copy
            a_sh.copy('Copy of ')

        # Renumber all notes to restore default spacing
#??? self.renum_all()
        self.a_db.commit()

        return DbErr.R_OK

    def version_list(self, a_list: list, i_new_version: int) -> int:
        #
        # Adds the list of notes to the specified version. Does not change the sort order of any note. Does not
        # check if the note is already in the version, any that are are effectively copies.
        #
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Copy each note to +1 after src
        for i_note_id in a_list:
            a_sh = DbNote(self.a_db, i_note_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Note ({0}) not found'.format(i_note_id))
                return DbErr.R_ERROR         # Invalid value
            # Copy
            a_sh.version(i_new_version)

        # Renumber all notes to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def delete_list(self, a_list: list) -> int:
        #
        # Mark a list of notes as deleted.
        # This function just sets the deleted flag in the record. Call empty_trash to really delete the records
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Delete each note
        for i_note_id in a_list:
            a_sh = DbNote(self.a_db, i_note_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Note ({0}) not found'.format(i_note_id))
                return DbErr.R_ERROR         # Invalid value
            # Delete
            a_sh.delete()

        # Renumber all notes to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def undelete_list(self, a_list: list) -> int:
        #
        # Mark a list of notes as undeleted.
        # This function just unsets the deleted flag in the record.
        #
        if type(a_list) != list:
            self.a_err.set_status(DbErr.E_INVTYPE, 1, 'Must be list')
            return DbErr.R_ERROR         # Invalid type

        # Delete each note
        for i_note_id in a_list:
            a_sh = DbNote(self.a_db, i_note_id, False)
            if not a_sh.exists():
                self.a_db.rollback()
                self.a_err.set_status(DbErr.E_INVPARAM, 1, 'Note ({0}) not found'.format(i_note_id))
                return DbErr.R_ERROR         # Invalid value
            # Undelete
            a_sh.undelete()

        # Renumber all notes to restore default spacing
#??? self.renum_all()
        self.a_db.commit()
        return DbErr.R_OK

    def empty_trash(self) -> int:
        #
        # Permanently remove any deleted notes for the CURRENT version
        #

        # Delete records
        s_sql = """
        delete from
            tbl_note
        where
            b_deleted = 1
            and l_version_code = :ver
            """
        self.a_db.execute(s_sql, {'ver': self.i_version})
        self.a_db.commit()

        return DbErr.R_OK

    def list_note(self):
        #
        #  Returns a list of visible note ids (i.e. not deleted) for the current version
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
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
            a_list.append(a_row['l_note_id'])

        return a_list

    def list_trash(self):
        #
        #  Returns a list of deleted note ids for the current version
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
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
            a_list.append(a_row['l_note_id'])

        return a_list

    def list_version(self):
        #
        #  Returns a list of note ids for the current version. Probably only useful for testing?
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
        where
            l_version_code = :ver
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_note_id'])

        return a_list

    def list_all(self):
        #
        #  Returns a list of ALL note ids. Probably only useful for testing?
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
        order by
            l_sort_order
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'ver': self.i_version})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_note_id'])

        return a_list


########################################################################################################################


