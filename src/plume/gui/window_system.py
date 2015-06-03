# -*- coding: utf-8 -*-
'''
Created on 25 avr. 2015

@author: Cyril Jacquet

This window system is used to attach or detach QWidget using controls like buttons 
(think like the detachable tabs in Firefox).


'''

from PyQt5.QtCore import Qt
    
class WindowSystemController():
    '''
    The WSC manage what are the WindowSystemActionHandler and the WindowSystemActionHandler
    ''' 
    def __init__(self):
        '''
        
        '''
        super().__init__()   
        self.window_dict = {}
        self.window_system_parent_widget = None
        self.sub_window_action_list = []
        self._window_system_update_funcs = []
                
    def detach_window_from_action(self):
        try:
            widget = self.side_bar.get_widget_linked_to_action(self.sender())
            self.detach_sub_window(self, widget)
        except:
            pass

    def detach_sub_window(self, sub_window):
        sub_window.setParent(None, Qt.Tool)
        sub_window.show()
        self.window_dict[sub_window.objectName()] = [sub_window, "detached"]
        
        
    def attach_sub_window(self, sub_window):
        sub_window.setParent(self.window_system_parent_widget)
        self.window_system_parent_widget.attach_sub_window(sub_window)

        self.window_dict[sub_window.objectName()] = [sub_window, "attached"]

    
    
    def add_action_to_window_system(self, action):
        if action not in self.sub_window_action_list:
            self.sub_window_action_list.append(action)
        self.announce_window_system_update()
        
        
    def subscribe_to_window_system_updates(self, func):
        if func not in self._window_system_update_funcs:
            self._window_system_update_funcs.append(func)
    
    
    def unsubscribe_to_window_system_updates(self, func):
        if func in self._window_system_update_funcs:
            self._window_system_update_funcs.remove(func)
            
            
    def announce_window_system_update(self):
        for func in self._window_system_update_funcs:
            func() 



class WindowSystemParentWidget(object):
    '''


    ''' 
    def attach_sub_window(self, sub_window):
        pass
    
    def detach_sub_window(self, sub_window):
        pass
    
    def set_sub_window_visible(self, sub_window):
        pass

class WindowSystemActionHandler(object):
    '''
    The QActions added to the action handler must have the QObject.property("sub_window_object_name") 
    filled with the object name of the corresponding sub_window
    ''' 
    def __init__(self):
        '''
        Constructor
        '''
        super().__init__()   
        
        self._window_system_controller = None
        self._sub_window_action_list = []
    

    @property
    def window_system_controller(self):
        return self._window_system_controller
    
    @window_system_controller.setter
    def window_system_controller(self, value):
        if not issubclass(value.__class__, WindowSystemController) :
            print("WindowSystemActionHandler.window_system_controller value is not a WindowSystemController")
        else:
            self._window_system_controller = value
            self._window_system_controller.subscribe_to_window_system_updates(self.update_from_window_system_ctrl)
    
    def update_from_window_system_ctrl(self):
        self._sub_window_action_list = self.window_system_controller.sub_window_action_list
    
    def detach_sub_window(self):
        sub_window = self.get_sub_window_linked_to_action(self.sender())
        self._window_system_controller.detach_sub_window(sub_window)
        
        
    def attach_sub_window(self):
        sub_window = self.get_sub_window_linked_to_action(self.sender())
        self._window_system_controller.attach_sub_window(sub_window)
    
    def get_sub_window_linked_to_action(self, sender_action):              
        sub_window_object_name = sender_action.property("sub_window_object_name")
        
        if sub_window_object_name == None :
            print(self.objectName() + " WindowSystemActionHandler action '" + sender_action \
                  +"' lacks 'sub_window_object_name' property")
            pass
        
        _list = self._window_system_controller.window_dict[sub_window_object_name]
        sub_window = _list[0]
        return sub_window
    
        
        
        
