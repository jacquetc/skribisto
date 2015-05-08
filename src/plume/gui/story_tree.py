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
        self.old_index = None


    def itemClicked(self, index):



        if index != self.oldIndex: # reset if change
            oneClickCheckpoint = False
            twoClicksCheckpoint = False
    
            self.oldIndex = index


        if oneClickCheckpoint & twoClicksCheckpoint: # third click

            #if(!index.data(37).toBool()) #if no separator
            #   self.edit(index);
            
            #reset :
            oneClickCheckpoint = False; 
            twoClicksCheckpoint = False
    
        elif oneClickCheckpoint == True: # second click

            #if !index.data(37).toBool()) #if no separator


            
            sheet_id = index.data(37)
            
            cfg.core.story_tree
            #Sheet *sheet = Core::instance()->project()->openSheet(path)



            #emit openSheetInView(sheet, "mainTextWritingZone")
            #emit openSheetInView(sheet, "noteDockWritingZone")
            #emit openSheetInView(sheet, "synopsisDockWritingZone")
        #        emit currentOpenedSheetSignal(index.data(Qt::UserRole).toInt())
        

            twoClicksCheckpoint = True
    
        else: # first click
            oneClickCheckpoint = True

