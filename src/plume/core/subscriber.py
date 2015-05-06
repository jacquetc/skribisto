'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

_update_funcs = []

def subscribe_update_func(func):
    if func not in _update_funcs:
        _update_funcs.append(func)
            

def unsubscribe_update_func(func):
    if func in _update_funcs:
        _update_funcs.remove(func)
            

def announce_update():
    for func in _update_funcs:
        func()
    
