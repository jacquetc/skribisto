'''
Created on 8 mai 2015

@author:  Cyril Jacquet
'''
from PyQt5.QtWidgets import QTreeView
from . import cfg

class StoryTreeView(QTreeView):
    '''
    StoryTreeView
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''
        super(StoryTreeView, self).__init__(parent=None)
        
        self._old_index = None
        
        self.setEditTriggers(QTreeView.NoEditTriggers)
        self.setExpandsOnDoubleClick(False)
        
        self.old_index = None
        self.clicked.connect(self.itemClicked)


    def itemClicked(self, index):

        if index != self._old_index: # reset if change
            self._one_click_checkpoint = False
            self._two_clicks_checkpoint = False    
        self._old_index= index
        if self._one_click_checkpoint and self._two_clicks_checkpoint: # third click
            #if(!index.data(37).toBool()) #if no separator
            self.setCurrentIndex(index)
            self.edit(index)
            
            #reset :
            self._one_click_checkpoint = False
            self._two_clicks_checkpoint = False    
        elif self._one_click_checkpoint == True: # second click

            #if !index.data(37).toBool()) #if no separator
            sheet_id = index.data(37)           
            cfg.core.tree_sheet_manager.open_sheet(sheet_id)
            #Sheet *sheet = Core::instance()->project()->openSheet(path)



            #emit openSheetInView(sheet, "mainTextWritingZone")
            #emit openSheetInView(sheet, "noteDockWritingZone")
            #emit openSheetInView(sheet, "synopsisDockWritingZone")
        #        emit currentOpenedSheetSignal(index.data(Qt::UserRole).toInt())
        

            self._two_clicks_checkpoint = True
    
        else: # first click
            self._one_click_checkpoint = True

