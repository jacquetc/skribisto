'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

_update_funcs = []
_disabled_funcs = []
_domain_list = []

def subscribe_update_func_to_domain(func, domain, sheet_id=None):
    '''
    function:: subscribe_update_func_to_domain(func, domain)
    :param func:
    :param domain: string like "data.tree.properties"
    :param sheet_id: int. optional. if present, can narrow_down the update.
    '''
    has_already_update_function = False
    
    for update_function in _update_funcs:
        if update_function.function is func and update_function.domain is domain:
            has_already_update_function = True
    if has_already_update_function is False:
            _update_funcs.append(UpdateFunction(func, domain, sheet_id))
        

def unsubscribe_update_func(func):
    '''
    function:: unsubscribe_update_func(func)
    :param func:
    ''' 
    for update_function in _update_funcs:   
        if update_function.function == func:
            _update_funcs.remove(update_function)
            
def disable_func(func):
    for update_function in _update_funcs:   
        if update_function.function == func:
            _update_funcs.remove(update_function)
            _disabled_funcs.append(update_function)
            
def enable_func(func):
    for update_function in _update_funcs:   
        if update_function.function == func:
            _disabled_funcs.remove(update_function)
            _update_funcs.append(update_function)
            
def announce_update(domain, sheet_id=-1):
    '''
    function:: announce_update(domain)
    :param domain:
    :param sheet_id: int. optional. if present, can narrow_down the update.
    '''    
    for update_function in _update_funcs:
        if update_function.domain == domain and update_function.sheet_id == sheet_id:
            update_function.function()
        #for the subscriber interested by all updates from every sheet:
        if update_function.domain == domain and update_function.sheet_id == None:
            f = update_function.function
            f()
        
class UpdateFunction():
    '''
    UpdateFunction
    '''

    def __init__(self, function, domain, sheet_id=None):
        '''
        Constructor
        '''

        super(UpdateFunction, self).__init__()

        self._function = function
        self._domain = domain
        self._sheet_id = sheet_id
        
    @property
    def function(self):
        return self._function
    
    @property
    def domain(self):
        return self._domain
    
    @property
    def sheet_id(self):
        return self._sheet_id
