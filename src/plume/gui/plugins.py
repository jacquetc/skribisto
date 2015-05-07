'''
Created on 6 mai 2015

@author:  Cyril Jacquet
'''

from . import cfg
import imp
import os
from yapsy.PluginManager import PluginManager
from yapsy.IPlugin import IPlugin
import importlib

class Plugins():
    '''
    Plugins
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(Plugins, self).__init__()
    
        # Build the manager
        self._plugin_manager = PluginManager()
        # Tell it the default place(s) where to find plugins
        self._plugin_manager.setPluginPlaces([os.path.sep.join([os.getcwd(), "plugins"])
                                        , "/home/cyril/Devel/workspace_eclipse/plume-creator/src/plugins/"
                                        , "/home/cyril/Devel/workspace_eclipse/plume-creator/src/plume/plugins/"])
        #print(os.path.sep.join([os.getcwd(), "plugins"]))
        # Define the various categories corresponding to the different
        # kinds of plugins you have defined
        self._plugin_manager.setCategoriesFilter({
                                                 "GuiStoryDockPlugin" : GuiStoryDockPlugin
                                                 })

        self._plugin_manager.collectPlugins()
        
        self.load_plugins(["GuiStoryDockPlugin"])


            
    def load_plugins(self, categories):
        '''
        function:: load_plugins(categories)
        :param categories: list
        '''
        for category in categories:
            for pluginInfo in self._plugin_manager.getPluginsOfCategory(category):
                pluginInfo.plugin_object.print_name()
                print(pluginInfo.plugin_object.gui_class().__name__)
                setattr(self, pluginInfo.plugin_object.gui_class().__name__ \
                        , pluginInfo.plugin_object.gui_class())

        
    
    
class GuiStoryDockPlugin(IPlugin):
    '''
    GuiStoryDockPlugin
    '''

    def __init__(self):
        '''
        Constructor
        '''

        super(GuiStoryDockPlugin, self).__init__()






'''
        self.plugin_folder = "./plugins"
        self.main_module = "__init__"
        
        for i in self.get_plugins():
            print("Loading plugin " + i["name"])
            plugin = self.load_plugin(i)
            plugin.run()
            
    def get_plugins(self):
        plugins = []
        possibleplugins = os.listdir(self.plugin_folder)
        for i in possibleplugins:
            location = os.path.join(self.plugin_folder, i)
            if not os.path.isdir(location) or not self.main_module + ".py" in os.listdir(location):
                continue
            info = imp.find_module(self.main_module, [location])
            plugins.append({"name": i, "info": info})
        return plugins

    def load_plugin(self,plugin):
        return imp.load_module(self.main_module, *plugin["info"])
        
    '''