'''
Created on 26 avr. 2015

@author:  Cyril Jacquet
'''

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
        
        Quick way to get the necessary to build a Qt treeModel
        
        return [(sheet_id, title, parent_id, children_id, properties), (...)]
        '''
        
        db = self.db
               
        cur = db.cursor()
        if tree_type is None: #select only designated tree type
            cur.execute("SELECT sheet_id, title, parent_id, children_id, properties FROM main_table WHERE tree=:tree ", {"tree" : tree_type})
        else: #take all
            cur.execute("SELECT sheet_id, title, parent_id, children_id, properties FROM main_table")

        
        result = cur.fetchall()
        final_result = []
        for row in result:
            sheet_id, title, parent_id, children_id, properties = row
            tuple_ =  (sheet_id, title, parent_id\
                       , transform_children_id_text_into_int_tuple(children_id) \
                       , transform_properties_text_into_dict(properties) )
            final_result.append(tuple_)
            

        return final_result

    def get_root_id(self, tree_type):
        db = self.db               
        cur = db.cursor()
        cur.execute("SELECT sheet_id FROM main_table WHERE tree=:tree AND is_root=1 ", {"tree" : tree_type})
        result = cur.fetchone()
        for row in result:
            sheet_id = int(row)        
        return sheet_id       
    
    def move(self, sheet_id, old_position_in_children, old_parent_id, new_position_in_children, new_parent_id):
        pass

    def get_title(self, sheet_id):
        db = self.db               
        cur = db.cursor()
        cur.execute("SELECT title FROM main_table WHERE sheet_id=:id" \
                                 , {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            title = row     
        return title            
    
    def set_title(self, sheet_id, new_title):
        self.db.cursor().execute("UPDATE main_table SET title=:title WHERE sheet_id=:id" \
                                 , {"title": new_title, "id": sheet_id})
        self.db.commit()
        
        
    def get_other_contents(self, sheet_id):
        pass
    
    def set_other_contents(self, sheet_id, dict_):
        pass
    
    

    def get_content(self, sheet_id):
        db = self.db               
        cur = db.cursor()
        cur.execute("SELECT content FROM main_table WHERE sheet_id=:id" \
                                 , {"id": sheet_id})
        result = cur.fetchone()
        for row in result:
            content = row     
        return content              
    
    def set_content(self, sheet_id, content):
        pass
   
    def get_content_type(self,sheet_id):
        pass
    
    def set_content_type(self,sheet_id, content_type):
        pass
   

    def get_properties(self,sheet_id):
        prop_dict = {}
        db = self.db
               
        cur = db.cursor()
        cur.execute("SELECT properties FROM main_table WHERE sheet_id=:id" \
                                 , { "id": sheet_id})
        
        result = cur.fetchone()
        for row in result:
            properties = row
            prop_dict = transform_properties_text_into_dict(properties)
        
        return prop_dict
    
    def set_properties(self,sheet_id, properties):
        pass
    
    def get_modification_date(self,sheet_id):
        pass
    
    def set_modification_date(self,sheet_id, date):
        pass
    
    def get_creation_date(self,sheet_id):
        pass

    def set_creation_date(self,sheet_id, date):
        pass
    
    def get_version(self,sheet_id):
        pass

def transform_children_id_text_into_int_tuple(children_id_text):
    int_tuple = ()
    int_list = []
    if children_id_text is not None:
        for txt in children_id_text.split(","):
            int_list.append(int(txt))            
        int_tuple = tuple(int_list)
    return int_tuple

def transform_properties_text_into_dict(properties):
    import ast
    
    properties_dict = {}
    if properties is not None:
        properties_dict = ast.literal_eval(properties)
    
    return properties_dict
