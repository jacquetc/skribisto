import unittest
import sqlite3
from PyQt5.QtSql import QSqlQuery, QSqlDatabase

from sql import select

class Test_SQL(unittest.TestCase):
 
    def setUp(self):
        self.database = sqlite3.connect(":memory:")
        cursor = self.database.cursor()

 
        cursor.execute("CREATE TABLE project_table(key TEXT primary key, value TEXT)")
        cursor.execute("INSERT INTO project_table VALUES('name', 'test')")
        self.database.commit()


    def test_select(self):
 
        cursor = self.database.cursor()
        cursor.execute("SELECT value FROM project_table WHERE key = 'name'")
        first = cursor.fetchone()
        self.assertEqual(first[0], 'test')


if __name__ == '__main__':
    unittest.main()
