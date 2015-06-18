'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''

from . import subscriber
import ast


class Tree(object):

    '''
    classdocs
    '''

    def __init__(self):
        super(Tree, self).__init__()
        '''
        Constructor
        '''
        self.db = None

    def get_tree_model_necessities(self, tree_type=None):
        '''
        :param tree_type: restrict to a given tree. Ex : write
        Quick way to get the necessary to build a Qt treeModel

        return [(sheet_id, title, parent_id, children_id, properties), (...)]
        '''

        db = self.db

        if db is None:  # closed
            return []

        cur = db.cursor()
        if tree_type is None:  # select only designated tree type
            cur.execute("SELECT sheet_id, title, parent_id, children_id, properties FROM main_table WHERE tree=:tree ", {
                        "tree": tree_type})
        else:  # take all
            cur.execute(
                "SELECT sheet_id, title, parent_id, children_id, properties FROM main_table")

        result = cur.fetchall()
        final_result = []
        for row in result:
            sheet_id, title, parent_id, children_id, properties = row
            tuple_ = (sheet_id, title, parent_id, transform_children_id_text_into_int_tuple(
                children_id), transform_properties_text_into_dict(properties))
            final_result.append(tuple_)

        return final_result

    def get_root_id(self, tree_type):
        db = self.db
        if db is None:  # closed
            return -1
        cur = db.cursor()
        cur.execute("SELECT sheet_id FROM main_table WHERE tree=:tree AND is_root=1 ", {
                    "tree": tree_type})
        result = cur.fetchone()
        for row in result:
            sheet_id = int(row)
        return sheet_id

    def move(self, sheet_id, old_position_in_children, old_parent_id, new_position_in_children, new_parent_id):
        pass

    def create_new_sheet(self, parent_id, tree_type):
        '''
        function:: create_new_tree_item(parent, tree_type)
        Append to parent
        :param parent: int, sheet_id of parent
        :param tree_type: string, Ex : "write"
        :rtype sheet_id: int, sheet_id of the new sheet
        '''
        c = self.db.cursor()
        c.execute("INSERT INTO main_table (parent_id, title, tree) \
        VALUES (:parent_id, '', :tree)", {"parent_id": parent_id, "tree": tree_type})
        # get new sheet_id:
        c.execute("SELECT last_insert_rowid()")
        result = c.fetchone()
        for row in result:
            sheet_id = row
        # get parent sheet :
        c.execute(
            "SELECT children_id FROM main_table WHERE sheet_id=:id", {"id": parent_id})
        result = c.fetchone()
        for row in result:
            children_id = row
        # modify parent's children_i :
        t = transform_children_id_text_into_int_tuple(children_id)
        children_list = list(t)
        children_list.append(sheet_id)
        if len(children_list) == 0:
            children_list_str = ""
        else:
            children_list_str = [str(child_id) for child_id in children_list]
        children_id = ",".join(children_list_str)
        self.db.cursor().execute("UPDATE main_table SET children_id=:children_id WHERE sheet_id=:id",
                                 {"children_id": children_id, "id": parent_id})
        self.db.commit()
        subscriber.announce_update("data.tree")
        subscriber.announce_update("data.project.notsaved")

        return sheet_id

    def get_title(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT title FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            title = row
        return title

    def set_title(self, sheet_id, new_title):
        self.db.cursor().execute("UPDATE main_table SET title=:title WHERE sheet_id=:id",
                                 {"title": new_title, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.title", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_other_contents(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute("SELECT other_sheet_contents_id FROM main_table WHERE sheet_id=:id", {
                    "id": sheet_id})
        result = cur.fetchone()
        for row in result:
            other_id = row
        # create a new one if no other_sheet_contents_id :
        if other_id is None:
            c = self.db.cursor()
            c.execute("INSERT INTO other_sheet_contents DEFAULT VALUES")
            # get new sheet_id:
            c.execute("SELECT last_insert_rowid()")
            result = c.fetchone()
            for row in result:
                other_id = row
            c.execute("UPDATE main_table SET other_sheet_contents_id=:other_id WHERE sheet_id=:id", {
                      "other_id": other_id, "id": sheet_id})
            db.commit()

        # insert in dict each column:
        dict_ = {}
        cur = db.cursor()
        cursor = cur.execute('SELECT * FROM other_sheet_contents')
        names = [description[0] for description in cursor.description]
        for name in names:
            cur = db.cursor()
            query = "".join(
                ["SELECT ",  name, " FROM other_sheet_contents WHERE other_sheet_contents_id=",  str(other_id)])
            cur.execute(query)
            result = cur.fetchone()
            for row in result:
                dat = row
            dict_[name] = dat

        return dict_

    def set_other_contents(self, sheet_id, dict_):
        db = self.db
        cur = db.cursor()
        cur.execute("SELECT other_sheet_contents_id FROM main_table WHERE sheet_id=:id", {
                    "id": sheet_id})
        result = cur.fetchone()
        for row in result:
            other_id = row

        cursor = cur.execute('select * from other_sheet_contents')
        names = [description[0] for description in cursor.description]

        for key in dict_.keys():
            if key == "other_sheet_contents_id":
                continue
            if key not in names:  # create column
                query = "".join(
                    ["ALTER TABLE other_sheet_contents ADD COLUMN ",  key, " NONE"])
                self.db.cursor().execute(query)
                self.db.commit()
            # insert date in column :
            dat = dict_[key]
            query = "".join(
                ["UPDATE other_sheet_contents SET ",  key, "=:dat WHERE other_sheet_contents_id=:id"])
            self.db.cursor().execute(query, {"dat": dat,  "id": other_id})
            self.db.commit()

        subscriber.announce_update("data.tree.other_contents", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_content(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT content FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            content = row
        return content

    def set_content(self, sheet_id, content):
        self.db.cursor().execute("UPDATE main_table SET content=:content WHERE sheet_id=:id",
                                 {"content": content, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.content", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_content_type(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT content_type FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            content_type = row
        return content_type

    def set_content_type(self, sheet_id, content_type):
        self.db.cursor().execute("UPDATE main_table SET content_type=:content_type WHERE sheet_id=:id",
                                 {"content": content_type, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.content_type", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_properties(self, sheet_id):
        prop_dict = {}
        db = self.db

        cur = db.cursor()
        cur.execute(
            "SELECT properties FROM main_table WHERE sheet_id=:id", {"id": sheet_id})

        result = cur.fetchone()
        for row in result:
            properties = row
            prop_dict = transform_properties_text_into_dict(properties)

        return prop_dict

    def set_properties(self, sheet_id, properties):
        properties_str = transform_dict_into_text(properties)
        self.db.cursor().execute("UPDATE main_table SET properties=:properties WHERE sheet_id=:id",
                                 {"properties": properties_str, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.properties", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_modification_date(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT modification_date FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            modification_date = row
        return modification_date

    def set_modification_date(self, sheet_id, modification_date):
        self.db.cursor().execute("UPDATE main_table SET modification_date=:modification_date WHERE sheet_id=:id",
                                 {"modification_date": modification_date, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.modification_date", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_creation_date(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT creation_date FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            creation_date = row
        return creation_date

    def set_creation_date(self, sheet_id, creation_date):
        self.db.cursor().execute("UPDATE main_table SET creation_date=:creation_date WHERE sheet_id=:id",
                                 {"creation_date": creation_date, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.creation_date", sheet_id)
        subscriber.announce_update("data.project.notsaved")

    def get_version(self, sheet_id):
        db = self.db
        cur = db.cursor()
        cur.execute(
            "SELECT version FROM main_table WHERE sheet_id=:id", {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            version = row
        return version

    def set_version(self, sheet_id, version):
        self.db.cursor().execute("UPDATE main_table SET version=:version WHERE sheet_id=:id",
                                 {"version": version, "id": sheet_id})
        self.db.commit()
        subscriber.announce_update("data.tree.version", sheet_id)
        subscriber.announce_update("data.project.notsaved")


def transform_children_id_text_into_int_tuple(children_id_text):
    int_tuple = ()
    int_list = []
    if children_id_text is not None:
        for txt in children_id_text.split(","):
            if txt != None:
                int_list.append(int(txt))
        int_tuple = tuple(int_list)
    return int_tuple


def transform_properties_text_into_dict(properties):
    properties_dict = {}
    if properties is not None:
        properties_dict = ast.literal_eval(properties)

    return properties_dict


def transform_dict_into_text(properties):
    properties_str = "{}"
    if properties is not {}:
        properties_str = str(properties)

    return properties_str
