# Bardi SQL Test

def select(string):
    pass


def create_new_database():

    database = sqlite3.connect(":memory:")
    cursor = database.cursor()
    cursor.execute(
        "CREATE TABLE project_table(key TEXT primary key, value TEXT)")
    cursor.execute("INSERT INTO project_table VALUES('name', 'test')")
    database.commit()
    database.close()
    return database


'''
        node.sheet_id = int(sheet_id)
        node.name = str(name)
        if parent_id != None :
            node.parent_id = int(parent_id)
        if children_id != None :
            node.children_id = []
            for txt in children_id.split(","):
                node.children_id.append(int(txt))
        if properties != None :
            node.properties = dict(properties)
'''
