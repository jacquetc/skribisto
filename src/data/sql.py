import sqlite3

def select(string):
    pass
    
def create_new_database():
    
    database = sqlite3.connect(":memory:")
    cursor = database.cursor()
    cursor.execute("CREATE TABLE project_table(key TEXT primary key, value TEXT)")
    cursor.execute("INSERT INTO project_table VALUES('name', 'test')")
    database.commit()
    database.close()
    return database

    
