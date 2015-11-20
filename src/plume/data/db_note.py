import sqlite3
from .exceptions import DbErr
from .db_property import DbProperty

class DbNote:
    #
    # A class to manipulate single notes
    #
    def __init__(self, a_db: sqlite3.Connection, i_note_id: int, b_commit: bool):
        # You need a note id to access this class, with the exception of add()
        # b_commit - set True to have this class do the commit for you. Set False to do the commit
        # yourself (see set_commit() for details)
        #
        # Class variables (consts in UPPERCASE). All vars are considered private - use the setter / getter to access.
        self.a_db = a_db                # db connection
        self.a_err = DbErr()            # Error status
        self.i_note_id = i_note_id
        self.b_commit = b_commit        # auto commit

    def set_commit(self, b_commit: bool):
        #
        # If you are doing several note operations (or operations on several notes) then you can set commit to
        # False to speed up execution. Sqlite is fast to execute, but slow to commit, so batch commits if you can.
        # (We only make the default True as a fail-safe for users who don't know the rules :) )
        #
        self.b_commit = b_commit

    def get_commit(self):
        #
        return self.b_commit

    def exists(self) -> bool:
        #
        # Returns True if the note id exists, False otherwise
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
        where
            l_note_id = :note
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})
        a_row = a_qry.fetchone()
        if a_row is None:
            return False
        else:
            return True

    def get_sort_order(self) -> int:
        #
        # Returns the sort_order of this note.
        # Returns R_ERROR (0) if the i_note_id is not found.
        #
        s_sql = """
        select
            l_sort_order
        from
            tbl_note
        where
            l_note_id = :note
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row[0]

    def set_sort_order(self, i_sort_order: int):
        #
        # Changes l_sort_order for the note. Does not check the version or the existence of the note.
        #
        s_sql = """
        update
            tbl_note
        set
            l_sort_order = :sort,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_note_id = :note
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id, 'sort': i_sort_order})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def get_note(self):
        #
        # Returns the column (except m_content) of a note record as a dict. {'name': value}
        #
        pass
        s_sql = """
        select
            l_note_id,
            t_status_code,
            l_sort_order,
            l_indent,
            l_version_code
            t_title
            t_synopsis,
            dt_created,
            dt_updated,
            dt_content,
            b_deleted
        from
            tbl_note
        where
            l_note_id = :note
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row

    def get_content(self) -> str:
        #
        # Returns the content of the note
        #
        s_sql = """
        select
            m_content
        from
            tbl_note
        where
            l_note_id = :note
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return a_row[0]

    def set_content(self, s_content: str):
        #
        # Updates the content of the note + the cword/char count + timestamp
        #
        s_sql = """
        update
            tbl_note
        set
            m_content = :content,
            dt_content = CURRENT_TIMESTAMP
        where
            l_note_id = :note
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id, 'content': s_content})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def get_title(self) -> str:
        s_sql = """
            SELECT
                t_title
            FROM
                tbl_note
            WHERE
                l_note_id=:id
        """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {"id": self.i_note_id})
        result = a_curs.fetchone()
        for row in result:
            title = row
        return title

    def set_title(self, s_title: str):
        s_sql = """
        update
            tbl_note
        set
            t_title = :title,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_note_id = :note
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id, 'title': s_title})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def get_indent(self) -> str:
        s_sql = """
            SELECT
                l_indent
            FROM
                tbl_note
            WHERE
                l_note_id=:id
        """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {"id": self.i_note_id})
        result = a_curs.fetchone()
        for row in result:
            indent = row
        return indent

    def set_indent(self, i_indent: int):
        s_sql = """
        update
            tbl_note
        set
            l_indent = :indent,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_note_id = :note
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id, 'indent': i_indent})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def set_version(self, i_version: int):
        s_sql = """
        update
            tbl_note
        set
            l_version_code = :version,
            dt_updated = CURRENT_TIMESTAMP
        where
            l_note_id = :note
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id, 'version': i_version})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def get_version(self) -> int:

        s_sql = """
            SELECT
                l_version_code
            FROM
                tbl_note
            WHERE
                l_note_id=:id
        """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {"id": self.i_note_id})
        result = a_curs.fetchone()
        for row in result:
            version = row
        return version


    def get_properties(self) -> dict:
        s_sql = """
            SELECT
                *
            FROM
                tbl_note_property
            WHERE
                l_note_code = :note_code
                """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {"note_code": self.i_note_id})
        result = a_curs.fetchall()
        dict_ = {}
        for row in result:
            dict_[row[1]] = row[2]
        return dict_

    def set_property(self, name, value):
        s_sql = """
            SELECT
                *
            FROM
                tbl_note_property
            WHERE
                l_note_code = :note_code
                and t_name = :name
                """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {"note_code": self.i_note_id, "name": name})
        a_row = a_curs.fetchone()
        if a_row is None:
            s_sql = """
                insert into
                    tbl_note_property
                (
                    l_note_code,
                    t_name,
                    t_value,
                    dt_created,
                    dt_updated
                )
                values(
                    :note_code,
                    :name,
                    :value,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                )
                """

            a_curs = self.a_db.cursor()
            a_curs.execute(s_sql, {'note_code': self.i_note_id, 'name': name, 'value': value})

        else:
            s_sql = """
                UPDATE
                    tbl_note_property
                SET
                    t_name = :name,
                    t_value = :value,
                    dt_updated = CURRENT_TIMESTAMP
                WHERE
                    l_note_code = :note_code
                    and t_name = :name
                """

            a_curs = self.a_db.cursor()
            a_curs.execute(s_sql, {'name': name, 'value': value, 'note_code': self.i_note_id})

        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK



    def set_note(self, a_rec: dict):
        #
        #  Changes multiple columns in the note. a_rec is a dict with one or more keys that are the column
        # names to change. You can't change l_note_id. dt_updated is set automatically
        #
        # Remove undesirable keys
        a_rec['l_note_id'] = self.i_note_id
        a_rec.pop('dt_updated', None)

        if len(a_rec) < 1:
            return DbErr.E_INVPARAM     # Nothing in dict!

        s_upd = ''
        # build update clause of the form t_col = :t_col,
        for s_col in a_rec:
            s_upd += s_col + ' = :' + s_col + ','

        # Remove final ',' before using - [:-1]

        s_sql = 'update tbl_note set dt_updated = CURRENT_TIMESTAMP, ' + s_upd[:-1] + ' where l_note_id = :l_note_id'
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, a_rec)

        if self.b_commit:
            self.a_db.commit()
        return DbErr.R_OK

    def get_count(self):
        #
        # Returns a tuple with the simple character and word counts for this note. (The values stored in the record)
        #
        s_sql = """
        select
            l_char_count,
            l_word_count
        from
            tbl_note
        where
            l_note_id = :note
        """
        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,
        else:
            return (a_row['l_char_count'], a_row['l_word_count'])

    def get_count_children(self):
        #
        # Returns a tuple with the character and word counts for all children of this note. If the notes are sorted
        # sort order, then this is the sum of all values for any note with a higher indent that this note. The last
        # note added is the note that has the same indent level as this note.
        #
        # NOTE: We deliberately don't include the current note, since we expect this function to be called once,
        # (when the current note starts to be edited), and the count for the current note will either already be
        # known, or will be changing, so this functions returns the values needed to ADD to the current note to get
        # the total count.
        #

        # Get current note values. I'm sure you can do this in a single query...
        s_sql = """
        select
            l_indent,
            l_sort_order
        from
            tbl_note
        where
            l_note_id = :note
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,

        i_parent_order = a_row['l_sort_order']
        i_parent_indent = a_row['l_indent']

        # Now get children that appear AFTER this note (sort Order > current)
        s_sql = """
        select
            l_indent,
            l_char_count,
            l_word_count
        from
            tbl_note
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

    def get_list_of_child_id(self) -> list:
        children = []
        # Get current note values. I'm sure you can do this in a single query...
        s_sql = """
        select
            l_indent,
            l_sort_order
        from
            tbl_note
        where
            l_note_id = :note
        """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})

        a_row = a_qry.fetchone()
        if a_row is None:
            # note does not exist
            self.a_err.set_status(DbErr.E_RECNOTFOU, 1, 'Note does not exist')
            return DbErr.R_ERROR  # Failed,

        i_parent_order = a_row[1]
        i_parent_indent = a_row[0]

        # Now get children that appear AFTER this note (sort Order > current)
        s_sql = """
        select
            l_note_id,
            l_indent
        from
            tbl_note
        where
            l_sort_order > :sort
        order by
            l_sort_order
        """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'sort': i_parent_order})

        for a_row in a_curs:
            if a_row[1] <= i_parent_indent:
                break       # No more children
            children.append(a_row[0])

        return children

    def add(self) -> int:
        #
        # Adds a blank note, Returns the note id, and sets the internal note id to the new one.
        #
        s_sql = """
            insert into
                tbl_note
            (
                t_title,
                dt_created,
                dt_updated,
                dt_content,
                l_version_code,
                b_deleted
            )
            values(
                :title,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                :version_code,
                0
            )
            """

        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'title': 'new', 'version_code': 0})

        self.i_note_id = a_curs.lastrowid
        if self.b_commit:
            self.a_db.commit()
        return self.i_note_id

    def copy(self, s_title_prefix: str) -> int:
        #
        # Copy the current note to a new note.
        # If s_title_prefix is not blank then:
        #   The note title will have s_title_prefix added before, so if s_title _prefix = 'Copy of ' then the
        #           next title will be 'Copy of <old title>'
        #       The sort order will be original + 1.
        #   Sort order is NOT changed if title prefix = ''
        # Returns the note id of the new note id
        # Does NOT alter the internal note id (this instance remains for the original note)
        #

        # The columns will need updating if the schema changes!
        # We don't copy the key, nor the created date. We add the title prefix if requested, and
        # the sort_order increment
        s_sql = """
            insert into
                tbl_note
            (
                l_sort_order,
                l_indent,
                t_title,
                m_content,
                l_version_code,
                dt_created,
                dt_updated,
                dt_content,
                b_deleted
            )
            select
                l_sort_order + :incr,
                l_indent,
                :prefix || t_title,
                m_content,
                l_version_code,
                CURRENT_TIMESTAMP,
                dt_updated,
                dt_content,
                b_deleted
            from
                tbl_note
            where
                l_note_id = :note
            """

        a_curs = self.a_db.cursor()

        if s_title_prefix == '':
            # No title prefix implies this is not a copy, so leave the sort order unchanged
            i_incr = 0
        else:
            # It's a copy, add 1 to sort order to position the new note under the old one
            i_incr = 1
        a_curs.execute(s_sql, {'note': self.i_note_id, 'prefix': s_title_prefix,
                               'version_code': self.get_version(), 'incr': i_incr})

        i_new_note_id = a_curs.lastrowid

        if self.b_commit:
            self.a_db.commit()

        return i_new_note_id

    def version(self, i_version: int) -> int:
        #
        # Create a new version of the note:
        # 1. If the dna of the src note is null, set it to the note_id (if it isn't null it means the src note is
        #   is a version of another note. The dna is the note id of it's original ancestor)
        # 2. Copy the current (src) note to a new note (this also copies the dna.)
        #
        # DNA Note: Although the dna codde is originally a note id, it is not intended to be used as a note
        # id. Instead, it is simply a value that is the same for all versions of the same ancestor, even if the
        # ancestor is deleted.
        #
        # Returns the note id of the new note id
        # Does NOT alter the internal note id (this instance remains for the original note)
        #

        # Turn off commit until the end of this function
        b_commit = self.get_commit()
        self.set_commit(False)

        # change dna if null
        s_sql = """
        update
            tbl_note
        set
            l_dna_code = l_note_id
        where
            l_note_id = :note
            and l_dna_code = 0
            """
        a_curs = self.a_db.cursor()
        a_curs.execute(s_sql, {'note': self.i_note_id})

        # Now copy the note
        i_note_id = self.copy('')

        # Change the version of the new note
        s_sql = """
        update
            tbl_note
        set
            l_version_code = :ver
        where
            l_note_id = :note
            """
        a_curs.execute(s_sql, {'note': i_note_id, 'ver': i_version})

        # Restore commit
        self.set_commit(b_commit)  # May be True or False

        if self.b_commit:
            self.a_db.commit()

        return i_note_id

    def list_version(self):
        #
        #  Returns a list of all notes that are a version of this note. Sorted by version number?
        #
        s_sql = """
        select
            l_note_id
        from
            tbl_note
        where
            l_dna_code = (
                select
                    l_dna_code
                from
                    tbl_note
                where
                    l_note_id = :note
                )
            and l_dna_code <> 0
        order by
            l_version_code
            """

        a_curs = self.a_db.cursor()
        a_qry = a_curs.execute(s_sql, {'note': self.i_note_id})
        a_list = []
        for a_row in a_qry:
            a_list.append(a_row['l_note_id'])

        return a_list

    def delete(self) -> int:
        #
        # Mark the note as deleted
        #
        s_sql = """
        update
            tbl_note
        set
            b_deleted = 1
        where
            l_note_id = :note
            """
        self.a_db.execute(s_sql, {'note': self.i_note_id})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def undelete(self) -> int:
        #
        # Mark the note as undeleted
        #
        s_sql = """
        update
            tbl_note
        set
            b_deleted = 0
        where
            l_note_id = :note
            """
        self.a_db.execute(s_sql, {'note': self.i_note_id})
        if self.b_commit:
            self.a_db.commit()

        return DbErr.R_OK

    def commit(self):
        #
        # Commits db. Only needed after all other calls if you set b_commit = False in __init__
        #
        self.a_db.commit()
        return DbErr.R_OK

########################################################################################################################


