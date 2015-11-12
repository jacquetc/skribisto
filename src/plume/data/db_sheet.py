import sqlite3
import time


i_ver = 0   # Version to operate on



########################################################################################################################


class DbErr:
    #
    # Common Db error codes and status handling
    #
    R_ERROR = 0                # Error return - check get_status(). MUST BE ZERO e.g. get_sheet_order
    R_OK = 1                   # Success return (some functions return another non zero value e.g. a sheet_id)

    # Error codes
    E_INVTYPE = 1              # Invalid parameter type.
    E_INVPARAM = 2             # Invalid parameter value.
    E_INTERNERR = 3            # Internal logic error or unexpected error
    E_RECNOTFOU = 4            # Requested record not found

    def __init__(self):
        # Class variables  All vars are considered private - use the setter / getter to access.
        self.i_status = 0               # Error status
        self.i_param = 0                # Error parameter number (self = 0)
        self.s_err_msg = ''

    def get_status(self):
        #
        # Returns a tuple with the last error status, message, and parameter number (if any).
        # If is only meaningful to call this if a function returns R_ERROR
        #
        if self.i_status == DbErr.E_INVTYPE:
            s_msg = 'Invalid parameter type (parameter {0})'.format(self.i_param)
        elif self.i_status == DbErr.E_INVPARAM:
            s_msg = 'Invalid parameter value (parameter {0})'.format(self.i_param)
        elif self.i_status == DbErr.E_INTERNERR:
            s_msg = 'Internal logic error'
        else:
            s_msg = 'Unhandled error'

        if self.s_err_msg != '':
            s_msg += '. ' + self.s_err_msg

        return (self.i_status, self.i_param, self.s_err_msg)

    def status(self) -> str:
        #
        # Returns get_status, nicely formatted
        #
        t_err = self.get_status()
        s_msg = 'Error: {0}. '.format(t_err[0]) + t_err[2]

        return s_msg

    def set_status(self, i_status: int, i_param: int, s_err_msg: str):
        #
        # Set the error status.
        #
        self.i_status = i_status
        self.i_param = i_param
        self.s_err_msg = s_err_msg


########################################################################################################################







class DbTree:
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

        i_dest = DbTree.RENUM_INT
        for a_row in a_rows:
            # For each sheet to renumber, pass it to the renum function.. For speed we commit after all rows renumbered
            a_sh = DbSheet(self.a_db, a_row['l_sheet_id'], False)
            a_sh.set_sort_order(i_dest)
            i_dest += DbTree.RENUM_INT

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
        if len(a_list) > (DbTree.RENUM_INT - 1):
            self.a_err.set_status(DbErr.E_INTERNERR, 0, "Can't move more than {0} sheets".format(DbTree.RENUM_INT - 1))
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
        self.renum_all()
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
        self.renum_all()
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
        self.renum_all()
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
        self.renum_all()
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
        self.renum_all()
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



class DbSheet:
    #
    # A class to manipulate single sheets
    #
    def __init__(self, a_db: sqlite3.Connection, i_sheet_id: int, b_commit: bool):
        # You need a sheet id to access this class, with the exception of add()
        # b_commit - set True to have this class do the commit for you. Set False to do the commit
        # yourself (see set_commit() for details)
        #
        # Class variables (consts in UPPERCASE). All vars are considered private - use the setter / getter to access.
        self.a_db = a_db                # db connection
        self.a_err = DbErr()            # Error status
        self.i_sheet_id = i_sheet_id
        self.b_commit = b_commit        # auto commit

    def set_commit(self, b_commit: bool):
        #
        # If you are doing several sheet operations (or operations on several sheets) then you can set commit to
        # False to speed up execution. Sqlite is fast to execute, but slow to commit, so batch commits if you can.
        # (We only make the default True as a fail-safe for users who don't know the rules :) )
        #
        self.b_commit = b_commit

    def get_commit(self):
        #
        return self.b_commit

    def exists(self) -> bool:
        #
        # Returns True if the sheet id exists, False otherwise
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})
        a_row = a_qry.fetchone()
        if a_row is None:
            return False
        else:
            return True

    def get_sort_order(self) -> int:
        #
        # Returns the sort_order of this sheet.
        # Returns R_ERROR (0) if the i_sheet_id is not found.
        #
        s_sql = """
        select
            l_sort_order
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # sheet does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Sheet does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row['l_sort_order']

    def set_sort_order(self, i_sort_order: int):
        #
        # Changes l_sort_order for the sheet. Does not check the version or the existence of the sheet.
        #
        s_sql = """
        update
            tbl_sheet
        set
            l_sort_order = :sort,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'sort': i_sort_order})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def get_sheet(self):
        #
        # Returns the column (except m_content) of a sheet record as a dict. {'name': value}
        #
        pass
        s_sql = """
        select
            l_sheet_id,
            t_status_code,
            l_sort_order,
            l_indent,
            l_version_code,
            l_dna_code,
            t_title,
            t_badge,
            t_synopsis,
            l_char_count,
            l_word_count,
            dt_created,
            dt_updated,
            dt_content,
            b_deleted
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # sheet does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Sheet does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row

    def get_content(self) -> str:
        #
        # Returns the content of the sheet
        #
        s_sql = """
        select
            m_content
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # sheet does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Sheet does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row['m_content']

    def set_content(self, s_content: str, i_char_count: int, i_word_count: int):
        #
        # Updates the content of the sheet + the cword/char count + timestamp
        #
        s_sql = """
        update
            tbl_sheet
        set
            m_content = :content,
            l_char_count = :charc,
            l_word_count = :wordc
            dt_content = CURRENT_TIMESTAMP
        where
            l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'content': s_content, 'charc': i_char_count, 'wordc': i_word_count})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def set_title(self, s_title: str):
        s_sql = """
        update
            tbl_sheet
        set
            t_title = :title,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'title': s_title})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def set_indent(self, i_indent: int):
        s_sql = """
        update
            tbl_sheet
        set
            l_indent = :indent,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'indent': i_indent})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def set_version(self, i_version: int):
        s_sql = """
        update
            tbl_sheet
        set
            l_version_code = :version,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'version': i_version})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def set_sheet(self, a_rec: dict):
        #
        # Changes multiple columns in the sheet. a_rec is a dict with one or more keys that are the column
        # names to change. You can't change l_sheet_id. dt_updated is set automatically
        #
        # Remove undesirable keys
        a_rec['l_sheet_id'] = self.i_sheet_id
        a_rec.pop('dt_updated', None)

        if len(a_rec) < 1:
            return DbErr.E_INVPARAM     # Nothing in dict!

        s_upd = ''
        # build update clause of the form t_col = :t_col,
        for s_col in a_rec:
            s_upd += s_col + ' = :' + s_col + ','

        # Remove final ',' before using - [:-1]

        s_sql = 'update tbl_sheet set dt_updated = CURRENT_TIMESTAMP, ' + s_upd[:-1] + ' where l_sheet_id = :l_sheet_id'
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, a_rec)

        if self.b_commit:
            self.a_db.commit()
        return DbErr.R_OK

    def get_count(self):
        #
        # Returns a tuple with the simple character and word counts for this sheet. (The values stored in the record)
        #
        s_sql = """
        select
            l_char_count,
            l_word_count
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # sheet does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Sheet does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return (a_row['l_char_count'], a_row['l_word_count'])

    def get_count_children(self):
        #
        # Returns a tuple with the character and word counts for all children of this sheet. If the sheets are sorted
        # sort order, then this is the sum of all values for any sheet with a higher indent that this sheet. The last
        # sheet added is the sheet that has the same indent level as this sheet.
        #
        # NOTE: We deliberately don't include the current sheet, since we expect this function to be called once,
        # (when the current sheet starts to be edited), and the count for the current sheet will either already be
        # known, or will be changing, so this functions returns the values needed to ADD to the current sheet to get
        # the total count.
        #

        # Get current sheet values. I'm sure you can do this in a single query...
        s_sql = """
        select
            l_indent,
            l_sort_order
        from
            tbl_sheet
        where
            l_sheet_id = :sheet
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # sheet does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Sheet does not exist')
            return DbErr.R_ERROR  # Failed,

        i_parent_order = a_row['l_sort_order']
        i_parent_indent = a_row['l_indent']

        # Now get children that appear AFTER this sheet (sort Order > current)
        s_sql = """
        select
            l_indent,
            l_char_count,
            l_word_count
        from
            tbl_sheet
        where
            l_sort_order > :sort
        order by
            l_sort_order
        """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sort': i_parent_order})


        i_char = 0
        i_word = 0
        for a_row in a_curs:
            if a_row['l_indent'] <= i_parent_indent:
                break       # No more children
            i_char += a_row['l_char_count']
            i_word += a_row['l_word_count']

        return (i_char, i_word)

    def add(self) -> int:
        #
        # Adds a blank sheet, Returns the sheet id, and sets the internal sheet id to the new one.
        #
        s_sql = """
            insert into
                tbl_sheet
            (
                t_title,
                dt_created,
                dt_updated,
                dt_content,
                l_dna_code,
                b_deleted
            )
            values(
                :title,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                0,
                0
            )
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'title': 'new'})

        self.i_sheet_id = a_curs.lastrowid
        if self.b_commit:
            self.a_db.commit()
        return self.i_sheet_id

    def copy(self, s_title_prefix: str) -> int:
        #
        # Copy the current sheet to a new sheet.
        # If s_title_prefix is not blank then:
        #   The sheet title will have s_title_prefix added before, so if s_title _prefix = 'Copy of ' then the
        #           next title will be 'Copy of <old title>'
        #       The sort order will be original + 1.
        #   Sort order is NOT changed if title prefix = ''
        # Returns the sheet id of the new sheet id
        # Does NOT alter the internal sheet id (this instance remains for the original sheet)
        #

        # The columns will need updating if the schema changes!
        # We don't copy the key, nor the created date. We add the title prefix if requested, and
        # the sort_order increment
        s_sql = """
            insert into
                tbl_sheet
            (
                t_status_code,
                l_sort_order,
                l_indent,
                l_version_code,
                l_dna_code,
                t_title,
                t_badge,
                m_content,
                t_synopsis,
                l_char_count,
                l_word_count,
                dt_created,
                dt_updated,
                dt_content,
                b_deleted
            )
            select
                t_status_code,
                l_sort_order + :incr,
                l_indent,
                l_version_code,
                l_dna_code,
                :prefix || t_title,
                t_badge,
                m_content,
                t_synopsis,
                l_char_count,
                l_word_count,
                CURRENT_TIMESTAMP,
                dt_updated,
                dt_content,
                b_deleted
            from
                tbl_sheet
            where
                l_sheet_id = :sheet
            """

        a_curs = self.a_db.cursor()

        if s_title_prefix == '':
            # No title prefix implies this is not a copy, so leave the sort order unchanged
            i_incr = 0
        else:
            # It's a copy, add 1 to sort order to position the new sheet under the old one
            i_incr = 1
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id, 'prefix': s_title_prefix, 'incr': i_incr})

        i_new_sheet_id = a_curs.lastrowid

        if self.b_commit:
            self.a_db.commit()

        return i_new_sheet_id

    def version(self, i_version: int) -> int:
        #
        # Create a new version of the sheet:
        # 1. If the dna of the src sheet is null, set it to the sheet_id (if it isn't null it means the src sheet is
        #   is a version of another sheet. The dna is the sheet id of it's original ancestor)
        # 2. Copy the current (src) sheet to a new sheet (this also copies the dna.)
        #
        # DNA Note: Although the dna codde is originally a sheet id, it is not intended to be used as a sheet
        # id. Instead, it is simply a value that is the same for all versions of the same ancestor, even if the
        # ancestor is deleted.
        #
        # Returns the sheet id of the new sheet id
        # Does NOT alter the internal sheet id (this instance remains for the original sheet)
        #

        # Turn off commit until the end of this function
        b_commit = self.get_commit()
        self.set_commit(False)

        # change dna if null
        s_sql = """
        update
            tbl_sheet
        set
            l_dna_code = l_sheet_id
        where
            l_sheet_id = :sheet
            and l_dna_code = 0
            """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sheet': self.i_sheet_id})

        # Now copy the sheet
        i_sheet_id = self.copy('')

        # Change the version of the new sheet
        s_sql = """
        update
            tbl_sheet
        set
            l_version_code = :ver
        where
            l_sheet_id = :sheet
            """
        a_curs.execute(s_sql, {'sheet': i_sheet_id, 'ver': i_version})

        # Restore commit
        self.set_commit(b_commit)  # May be True or False

        if self.b_commit:
            self.a_db.commit()

        return i_sheet_id

    def list_version(self):
        #
        #  Returns a list of all sheets that are a version of this sheet. Sorted by version number?
        #
        s_sql = """
        select
            l_sheet_id
        from
            tbl_sheet
        where
            l_dna_code = (
                select
                    l_dna_code
                from
                    tbl_sheet
                where
                    l_sheet_id = :sheet
                )
            and l_dna_code <> 0
        order by
            l_version_code
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'sheet': self.i_sheet_id})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_sheet_id'])

        return a_list

    def delete(self) -> int:
        #
        # Mark the sheet as deleted
        #
        s_sql = """
        update
            tbl_sheet
        set
            b_deleted = 1
        where
            l_sheet_id = :sheet
            """
        self.a_db.execute(s_sql, {'sheet': self.i_sheet_id})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def undelete(self) -> int:
        #
        # Mark the sheet as undeleted
        #
        s_sql = """
        update
            tbl_sheet
        set
            b_deleted = 0
        where
            l_sheet_id = :sheet
            """
        self.a_db.execute(s_sql, {'sheet': self.i_sheet_id})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def commit(self):
        #
        # Commits db. Only needed after all other calls if you set b_commit = False in __init__
        #
        self.a_db.commit()
        return DbErr.R_OK
