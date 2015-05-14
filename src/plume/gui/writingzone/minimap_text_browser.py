'''
Created on 13 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QTextEdit, QMenu, QGraphicsView, QGraphicsScene,\
    QGraphicsItem
from PyQt5.QtCore import pyqtSlot, Qt, pyqtSignal
from PyQt5.QtOpenGL import QGLWidget
from PyQt5.QtGui import QPainter, QPen
from PyQt5.Qt import QRectF

class Minimap2(QGraphicsView):
    '''
    Minimap
    '''

    def __init__(self, parent = 0):
        '''
        Constructor
        '''


        super(Minimap2, self).__init__(parent)
       
        self._scene = QGraphicsScene()
        self.setRenderHint(QPainter.Antialiasing, True)
        self._text_browser = TextBrowser()
        self._scrollbar = None
        self.setAutoFillBackground(True)
        self.setViewport(QGLWidget())
        self.setScene(self._scene)
        self.setAlignment(Qt.AlignTop)
        self.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        
        #cursor:
        self._nav_cursor = Cursor()
        self._scene.addItem(self._nav_cursor)
        self._nav_cursor.setZValue(1)
        
        self._text_edit = None
        
        self._graphics_proxy_text_browser = self._scene.addWidget(self._text_browser)
        self._scale = 0.3
        self._graphics_proxy_text_browser.setScale(self._scale)
        
        
        
    @property
    def text_edit(self):
        return self._text_edit
    
    @text_edit.setter
    def text_edit(self, text_edit):
        if not isinstance(text_edit, QTextEdit):
            return
        self._text_edit = text_edit
        #self._text_edit.textChanged.connect(self.update)
        #self._scrollbar.valueChanged.connect(self._change_cursor_pos)


        self.setFixedWidth(self._text_edit.width() * self._scale)
        
        self._text_browser.setDocument(self._text_edit.document())       
        
        #set cursor:
        
        
        #connect scrollbar
        baseScrollBar = self._text_edit.verticalScrollBar()
        baseScrollBar.rangeChanged.connect(self._set_scrollBar_range)
        baseScrollBar.valueChanged.connect(self._text_browser.verticalScrollBar().setValue)         
        self._text_browser.verticalScrollBar().valueChanged.connect(baseScrollBar.setValue)        
        self._text_browser.verticalScrollBar().hide()
        
    @pyqtSlot()
    def update(self):
        #if self._is_cursor_moved:
            #return   
        pass

        
    def _change_scrollbar_value(self, value):
        #self._scrollbar.setValue(value)
        pass 
     
    @pyqtSlot('QSize')
    def update_size(self, size):
        self.setFixedWidth(self._text_edit.width() * self._scale)
        
    def resizeEvent(self, event):
        


        
        #self._graphics_proxy_text_browser.resize(self._scene.sceneRect().size())
        self._graphics_proxy_text_browser.setGeometry(QRectF(0, 0
                                                             , event.size().width() / self._scale
                                                              , event.size().height() / self._scale))
        self.setSceneRect(0, 0, event.size().width(), event.size().height())
        self._nav_cursor.set_width(event.size().width())
        self._nav_cursor.set_height(self._text_edit.height() * self._scale)
        #return QGraphicsView.resizeEvent(self, event)    

    def _set_scrollBar_range(self, min_, max_):
        
        self._text_browser.verticalScrollBar().setMinimum(min_)
        self._text_browser.verticalScrollBar().setMaximum(max_)

        if min_ == 0 and max_ == 0:
            self._text_browser.verticalScrollBar().hide()
        else:
            self._text_browser.verticalScrollBar().show()
            
                  
from PyQt5.QtWidgets import QTextBrowser

class TextBrowser(QTextBrowser):
    '''
    MinimapTextBrowser
    '''

    def __init__(self, parent=None):
        '''
        Constructor
        '''

        super(TextBrowser, self).__init__(parent=None)
        self.setTextInteractionFlags(Qt.NoTextInteraction)
        self.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        
        
    
class Cursor(QGraphicsItem):
    '''
    Cursor
    '''
    cursor_pos_changed = pyqtSignal(int, name='cursor_pos_changed')
    
    def __init__(self):
        super(Cursor, self).__init__()
        self._width = 10
        self._height = 10
        self._text_edit_height = 1
        self._minimap_height = 1
        self._doc_height = 1
        self.pos_ratio = 1
        self.graphics_view = None
        self.setCursor(Qt.OpenHandCursor)  
        self.setFlag(QGraphicsItem.ItemIsMovable, True)

    
    def set_width(self, width):
        self.prepareGeometryChange()
        self._width = width
        
    def set_height(self, height):
        self.prepareGeometryChange()
        self._height = height
        
    def height(self):
        return self._height
        
    def set_text_edit_height(self, height):
        self._text_edit_height = height
        
    def set_doc_height(self, height):
        self._doc_height = height
                
    def set_minimap_height(self, height):
        self._minimap_height = height

    def paint(self, painter, option, widget):
        pen = QPen(Qt.red)
        pen.setWidth(1)
        painter.setPen(pen)
        painter.drawRect(1, 1, self._width - 1, self._height - 1)
        
    def boundingRect(self):
        return QRectF(0, 0, self._width, self._height)
    
    def mousePressEvent(self, event):
        if event.buttons() == Qt.LeftButton:
            self.setCursor(Qt.ClosedHandCursor)
            self.graphics_view._is_cursor_moved = True
        
        
    def mouseReleaseEvent(self, event):
        self.setCursor(Qt.OpenHandCursor)
        self.graphics_view._is_cursor_moved = False
        
    def hoverLeaveEvent(self, event):
        self.setCursor(Qt.OpenHandCursor)
        self.graphics_view._is_cursor_moved = False 
                          
    def mouseMoveEvent(self, event): 
        #pos = self.mapFromGlobal(event.globalPos())
        self.graphics_view._is_cursor_moved = True
        QGraphicsItem.mouseMoveEvent(self, event)
        if event.buttons() == Qt.LeftButton:
            max_y_pos = self._minimap_height -  self.height()
            # fix vertical
            self.setX(0)
            if self.y() < 0:
                self.setY(0)
            elif self.y() > max_y_pos and max_y_pos < 0:
                self.setY(0)
            elif self.y() > max_y_pos and max_y_pos > 0:
                self.setY(max_y_pos)
            
            
            #move textedit's scrollbar 
            elif self.pos_ratio > 0:
                print("self.y()",self.y())
                print("self.pos_ratio",self.pos_ratio)
                print("/",int(self.y() / self.pos_ratio))
                cursor_ratio = self.y() / self._minimap_height
                print("cursor_ratio",cursor_ratio)
                r = self.y() + self.height() * cursor_ratio
                print("r",r)
                self.graphics_view._change_scrollbar_value(int(r / self.pos_ratio))
               
