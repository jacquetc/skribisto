'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from .base import DatabaseBaseClass
import ast
from .db_note_tree import DbNoteTree, DbNote


class NoteTree(DatabaseBaseClass):
    '''
    classdocs
    '''
    _tree_name = "note_tree"

    def __init__(self, database_subscriber):
        super(NoteTree, self).__init__(database_subscriber)
        '''
        Constructor
        '''
        self.DBTree = DbNoteTree
        self.DBTreeItem = DbNote
        self.a_db = None
        self.b_renum = False
        self.i_ver = 0

    def renum_all(self):
        # Renum
        a_tree = self.DbTree(self.a_db)
        #a_tree.set_version(i_ver)
        a_tree.renum_all()
        self.a_db.commit()
        self.b_renum = False

    def get_tree_model_necessities(self):
        '''
        :param tree_type: restrict to a given tree. Ex : write
        Quick way to get the necessary to build a Qt treeModel

        return [(note_id, title, parent_id, children_id, properties), (...)]
        '''

        db = self.a_db

        if db is None:  # closed
            return []

        cur = db.cursor()
        # if tree_type is None:  # select only designated tree type
        cur.execute("SELECT l_note_id, t_title, l_sort_order, l_indent, b_deleted FROM tbl_note "
                    "        order by l_sort_order")
        # else:  # take all
        #     cur.execute(
        #         "SELECT note_id, title, parent_id, children_id, properties FROM main_table")

        result = cur.fetchall()
        final_result = []
        for row in result:
            l_note_id, t_title, l_sort_order, l_indent, b_deleted = row
            tuple_ = (l_note_id, t_title, l_sort_order, l_indent, b_deleted)
            final_result.append(tuple_)

        return final_result


    # def get_root_id(self, tree_type):
    #     db = self.sqlite_db
    #     if db is None:  # closed
    #         return -1
    #     cur = db.cursor()
    #     cur.execute("SELECT note_id FROM main_table WHERE tree=:tree AND is_root=1 ", {
    #                 "tree": tree_type})
    #     result = cur.fetchone()
    #     for row in result:
    #         note_id = int(row)
    #     return note_id

    def _move(self, note_id_list, indent: int, destination_note_id: int):
        # Move list of notes
        a_tree = self.DbTree(self.a_db)
        a_tree.set_version(self.i_ver)

        # Move notes
        a_list = [int(n, 10) for n in note_id_list]
        # 'Move to after Note Id [0=move to top] '
        i_dest = destination_note_id
        if a_tree.move_list(a_list, int(i_dest)):
            print('Move ok')
            self.b_renum = True
        else:
            print(a_tree)

    def _create_new_tree_item(self, id_to_insert_after: int, indent: int, item_values: dict):
        '''
        function:: _create_new_tree_item(id_to_insert_after, indent)
        Append to parent as the las of children
        :param : int, note_id of parent
        :rtype note_id: int, note_id of the new note
        '''

        # Add note
        a_tree = self.DbTree(self.a_db)

        #print('Add note')
        #'Id to insert after [0=top, -1=bottom]
        i_dest = id_to_insert_after
        s_title = "title"
        i_indent = indent

        # Add note
        a_sh = DbNote(self.a_db, 0, False)
        i_item_id = a_sh.add()
        # Set title, version + indent
        # You can use individual setters:
        a_sh.set_title(s_title)
        # Or a dict to do multiple columns at once.
        a_sh.set_item({'l_indent': i_indent})
        a_sh.set_item(item_values)

        #  Move note to desired position
        a_tree.move_list([i_item_id], int(i_dest))
        self.b_renum = True

        self.renum_all()

        self.subscriber.acreate_new_item_afternnounce_update("data." + self._tree_name)
        self.subscriber.announce_update("data.project.notsaved")

        return i_item_id

    def create_new_child_item(self, parent_item_id, item_values: dict = {}) -> int:
        a_parent_item = DbNote(self.a_db, parent_item_id, False)

        list_of_child_id = a_parent_item.get_list_of_child_id()
        if not list_of_child_id:
            dest_item_id = parent_item_id
        else:
            dest_item_id = list_of_child_id[-1]

        return self._create_new_tree_item(dest_item_id, a_parent_item.get_indent() + 1, item_values)

    def create_new_item_after(self, destination_item_id, item_values: dict = {}) -> int:
        a_dest_item = DbNote(self.a_db, destination_item_id, False)
        a_parent_item = a_dest_item
        list_of_child_id = a_parent_item.get_list_of_child_id()
        if not list_of_child_id:
            dest_item_id = destination_item_id
        else:
            dest_item_id = list_of_child_id[-1]
        return self._create_new_tree_item(dest_item_id, a_dest_item.get_indent(), item_values)

    def get_title(self, item_id: int) -> str:

        a_sh = DbNote(self.a_db, item_id, False)
        return a_sh.get_title()

    def set_title(self, item_id, new_title):

        a_sh = DbNote(self.a_db, item_id, True)
        a_sh.set_title(new_title)
        self.subscriber.announce_update("data." + self._tree_name + ".title", item_id)
        self.subscriber.announce_update("data.project.notsaved")

    def get_content(self, item_id):
        a_item = DbNote(self.a_db, item_id, False)
        return a_item.get_content()

    def set_content(self, item_id, content):
        a_item = DbNote(self.a_db, item_id, True)
        a_item.set_content(content)
        self.subscriber.announce_update("data." + self.tree_name + ".content", item_id)
        self.subscriber.announce_update("data.project.notsaved")

    def get_content_type(self, note_id):
        # db = self.a_db
        # cur = db.cursor()
        # cur.execute(
        #     "SELECT content_type FROM main_table WHERE note_id=:id", {"id": note_id})
        # result = cur.fetchone()
        # for row in result:
        #     content_type = row
        # return content_type
        pass

    def set_content_type(self, note_id, content_type):
        # self.a_db.cursor().execute("UPDATE main_table SET content_type=:content_type WHERE note_id=:id",
        #                            {"content": content_type, "id": note_id})
        # self.a_db.commit()
        # self.subscriber.announce_update("data.note_tree.content_type", note_id)
        # self.subscriber.announce_update("data.project.notsaved")
        pass

    def get_properties(self, item_id):

        a_item = self.DBTreeItem(self.a_db, item_id, False)
        return a_item.get_properties()

    def set_property(self, item_id, property_name, property_value):
        # properties_str = transform_dict_into_text(properties)
        # self.a_db.cursor().execute("UPDATE main_table SET properties=:properties WHERE sheet_id=:id",
        #                            {"properties": properties_str, "id": sheet_id})
        # self.a_db.commit()
        a_item = self.DBTreeItem(self.a_db, item_id, True)
        a_item.set_property(property_name, property_value)
        self.subscriber.announce_update("data." + self._tree_name + ".properties", item_id)
        self.subscriber.announce_update("data.project.notsaved")

    def remove_property(self, item_id, property_name):
        a_item = self.DBTreeItem(self.a_db, item_id, True)
        a_item.remove_property(property_name)
        self.subscriber.announce_update("data." + self._tree_name + ".properties", item_id)
        self.subscriber.announce_update("data.project.notsaved")

    def change_property_name(self, item_id, old_name, new_name):
        a_item = self.DBTreeItem(self.a_db, item_id, True)
        a_item.change_property_name(old_name, new_name)
        self.subscriber.announce_update("data." + self._tree_name + ".properties", item_id)
        self.subscriber.announce_update("data.project.notsaved")

    def get_modification_date(self, note_id):
        # db = self.a_db
        # cur = db.cursor()
        # cur.execute(
        #     "SELECT modification_date FROM main_table WHERE note_id=:id", {"id": note_id})
        # result = cur.fetchone()
        # for row in result:
        #     modification_date = row
        # return modification_date
        pass

    def set_modification_date(self, note_id, modification_date):
        # self.a_db.cursor().execute("UPDATE main_table SET modification_date=:modification_date WHERE note_id=:id",
        #                            {"modification_date": modification_date, "id": note_id})
        # self.a_db.commit()
        # self.subscriber.announce_update("data.note_tree.modification_date", note_id)
        # self.subscriber.announce_update("data.project.notsaved")
        pass

    def get_creation_date(self, note_id):
        # db = self.a_db
        # cur = db.cursor()
        # cur.execute(
        #     "SELECT creation_date FROM main_table WHERE note_id=:id", {"id": note_id})
        # result = cur.fetchone()
        # for row in result:
        #     creation_date = row
        # return creation_date
        pass

    def set_creation_date(self, note_id, creation_date):
        # self.a_db.cursor().execute("UPDATE main_table SET creation_date=:creation_date WHERE note_id=:id",
        #                            {"creation_date": creation_date, "id": note_id})
        # self.a_db.commit()
        # self.subscriber.announce_update("data.note_tree.creation_date", note_id)
        # self.subscriber.announce_update("data.project.notsaved")
        pass

    def get_version(self, note_id):
        # db = self.a_db
        # cur = db.cursor()
        # cur.execute(
        #     "SELECT version FROM main_table WHERE note_id=:id", {"id": note_id})
        # result = cur.fetchone()
        # for row in result:
        #     version = row
        # return version
        pass

    def set_version(self, note_id, version):
        # self.a_db.cursor().execute("UPDATE main_table SET version=:version WHERE note_id=:id",
        #                            {"version": version, "id": note_id})
        # self.a_db.commit()
        # self.subscriber.announce_update("data.note_tree.version", note_id)
        # self.subscriber.announce_update("data.project.notsaved")
        pass
#useless
def transform_children_id_text_into_int_tuple(children_id_text):
    int_tuple = ()
    int_list = []
    if children_id_text is not None:
        for txt in children_id_text.split(","):
            if txt is not None:
                int_list.append(int(txt))
        int_tuple = tuple(int_list)
    return int_tuple

#useless
def transform_properties_text_into_dict(properties):
    properties_dict = {}
    if properties is not None:
        properties_dict = ast.literal_eval(properties)

    return properties_dict

#useless
def transform_dict_into_text(properties):
    properties_str = "{}"
    if properties is not {}:
        properties_str = str(properties)

    return properties_str

