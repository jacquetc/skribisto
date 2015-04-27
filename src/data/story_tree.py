'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''
from .tree import Tree

class StoryTree(Tree):
    '''
    classdocs
    '''


    def __init__(self):
        super(StoryTree, self).__init__()
        '''
        Constructor
        '''
        self.table_name = "story_table"
        self.db = None


    def get_tree_model_necessities(self):
        '''
        
        Quick way to get the necessary to build a Qt treeModel
        
        return [(sheet_id, name, parent_id, children_id, properties), (...)]
        '''
        
        db = self.db
               
        cur = db.cursor()
        cur.execute("SELECT sheet_id, name, parent_id, children_id, properties FROM story_table")


        
        return cur.fetchall()


        
    def rename(self, id, new_name):
        pass
    
    def move(self, id, old_position_in_children, old_parent_id, new_position_in_children, new_parent_id):
        pass







    
    def get_content(self, id):
        pass
    
    def set_content(self, id, content):
        pass
   
    def get_content_type(self, id):
        pass
    
    def set_content_type(self, id, content_type):
        pass
   
    def get_synopsys(self, id):
        pass
    
    def set_synopsys(self, id, content):
        pass
    
    def get_notes(self, id):
        pass
    
    def set_notes(self, id, content):
        pass
    
    def get_properties(self, id):
        pass
    
    def set_properties(self, id, properties):
        pass
    
    def get_modification_date(self, id):
        pass
    
    def set_modification_date(self, id, date):
        pass
    
    def get_creation_date(self, id):
        pass

    def set_creation_date(self, id, date):
        pass
    
    def get_version(self, id):
        pass
    