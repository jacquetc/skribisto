'''
Created on 11 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QTextEdit, QMenu, QGraphicsView, QGraphicsScene, QLabel,\
    QGraphicsItem
from PyQt5.QtCore import pyqtSlot, Qt, QThread, pyqtSignal, QRectF
from PyQt5.QtGui import QPixmap, QPalette, QTextBlockFormat
from PyQt5.Qt import QPainterPath, QImage, QPainter, QPen, QPoint, QRect,\
    QAbstractTextDocumentLayout, QFontMetrics
from PyQt5.QtOpenGL import QGLWidget

class Minimap(QGraphicsView):
    '''
    Minimap
    '''

    def __init__(self, parent = 0):
        '''
        Constructor
        '''


        super(Minimap, self).__init__(parent)
       
        self._scene = QGraphicsScene()
        self.setAutoFillBackground(True)
        self.setViewport(QGLWidget())
        self.setScene(self._scene)
        self.setAlignment(Qt.AlignTop)
        self.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        
        self.nav_cursor = Cursor()
        self.nav_cursor.graphics_view  = self
        self._scene.addItem(self.nav_cursor)
        self._graphic_item_list = []
        self._pixmap = None
        self._text_edit = None
        self._abstract_doc_layout = None
        self._is_cursor_moved = False
        
        self._img_generator = ImageGenerator()
        self._img_generator.image_generated.connect(self._update_with)
        #self.setBackgroundRole(QPalette.Dark)

    @property
    def text_edit(self):
        return self._text_edit
    
    @text_edit.setter
    def text_edit(self, text_edit):
        if not isinstance(text_edit, QTextEdit):
            pass
        self._text_edit = text_edit
        self._text_edit.textChanged.connect(self.update)
        self._scrollbar = self._text_edit.verticalScrollBar()
        self._scrollbar.valueChanged.connect(self._change_cursor_pos)
    
    @pyqtSlot()
    def update(self):
        if self._is_cursor_moved:
            pass   
        self._generate_image(self.text_edit)
        
    def _change_scrollbar_value(self, value):
            self._scrollbar.setValue(value)

    def resizeEvent(self, event):
        
        #self._image_label.setFixedHeight(self.height())
        #self._image_label.setFixedWidth(self.width())
        return QGraphicsView.resizeEvent(self, event)

#--------------------------------------------------------------------------
    def _generate_image(self, text_edit):
        
        doc = text_edit.document()

        item_list = []
        for i in range(0, doc.blockCount()):
            block = doc.findBlockByNumber(i)
            item = DocItem()
            item.block_text = block.text()
            item.text_format = block.blockFormat()
            item.width = text_edit.viewport().width()
            layout = block.layout()
            item.lines = []
            if layout:
                for i in range(0,layout.lineCount()):
                    item.lines.append(layout.lineAt(i))
            
            
            item_list.append(item)
        self._img_generator.set_items(item_list)
        self._img_generator.start()
        
    @pyqtSlot('QPixmap')
    def _update_with(self, pixmap):
        for g_item in self._graphic_item_list:
            if g_item.name == "map":
                self._scene.removeItem(g_item)
                self._graphic_item_list.remove(g_item)
                del g_item

        item = self._scene.addPixmap(pixmap)
        item.name = "map"
        item.setPos(0,0)
        self._pixmap = pixmap
        self._graphic_item_list.append(item)
        
        self.setFixedWidth(pixmap.width())
        
        # cursor :
        self.nav_cursor.setZValue(1)
        ## width :
        self.nav_cursor.set_width(pixmap.width())
        ## height :
        text_edit_height = self.text_edit.height()
        doc_height = self.text_edit.document().size().height()
        minimap_height = self._pixmap.height()   
        self.nav_cursor.set_minimap_height(minimap_height)
        ratio = doc_height / text_edit_height
        cursor_height = minimap_height / ratio
        self.nav_cursor.set_height(cursor_height)
        ## position :
        self._change_cursor_pos()
    
    def _change_cursor_pos(self):
        if self._is_cursor_moved == True:
            return
        scroll_value = self._scrollbar.value()
        scroll_max = self._scrollbar.maximum()
        available_height_for_cursor = self._pixmap.height() - self.nav_cursor.height()
        if scroll_max != 0:
            pos_ratio = scroll_value / scroll_max
            self.nav_cursor.pos_ratio = pos_ratio
            self.nav_cursor.setPos(0, available_height_for_cursor * pos_ratio)                
        else:
            self.nav_cursor.setPos(0,0)            
  
    
class Cursor(QGraphicsItem):
    '''
    Cursor
    '''
    cursor_pos_changed = pyqtSignal(int, name='cursor_pos_changed')
    
    def __init__(self):
        super(Cursor, self).__init__()
        self._width = 1
        self._height = 1
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
        pen.setWidth(2)
        painter.setPen(pen)
        painter.drawRoundedRect(1, 1, self._width - 3, self._height - 3, 5, 5)
        
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
               
import hashlib

class DocItem():
    '''
    Block
    '''
    def __init__(self):
        super(DocItem, self).__init__()
        self.block_text = ''
        self.text_format = None
        self._hash = ''
        self.index = None
        self._image = None
        self.width = None
        self.lines = []
    
    @property    
    def hash(self):
        if self._hash is "": # generate hash from text and format
            m = hashlib.md5()
            m.update(bytes(self.block_text, "utf-8"))
            m.update(bytes(self.width))
            self._hash = m.digest()
        return self._hash
                
    def __eq__(self, other):
        if self.hash == other.hash:
            return True
        else:
            return False

    @property  
    def image(self):
        if self._image == None:            
            self._image = self._generate_image()
        
        return self._image
    
    def _generate_image(self):
        h = 2
        w = 1        
    
        def draw_char(pos, pen_color):
            pix_paint.setPen(QPen(pen_color))

            rect = QRect(i*w + i, 1, w, h)
            pix_paint.drawRect(rect)
            pix_paint.fillRect(rect, pen_color)

        #range of tab:
        tab_default = 80
        
        text ="m"
        px = QPixmap(100, 100)
        p = QPainter(px)
        fm = p.fontMetrics()
        width = fm.width(text)
        p.end()
        number_chars_in_one_tab = int(tab_default / width)
        
        p_text = self._prepared_text()
        split_p_text = p_text.split("N")
        
        pix_list = []
        for text in split_p_text:
            line_num = 0
            i = 0
            length = len(text)
            if length == 0:
                length = 1
            pixmap = QImage(length*w + length, h*2, QImage.Format_ARGB32_Premultiplied)
            pixmap.fill(Qt.white)
            pix_paint = QPainter(pixmap)
            
            for char in text:
                if char == "c":
                    draw_char(i, Qt.black)
                if char == "t":
                    for _ in range(0,number_chars_in_one_tab):
                        draw_char(i, Qt.transparent)
                        i+= 1
                if char == "_":
                    draw_char(i, Qt.transparent)
                i += 1
            
            pix_list.append(pixmap)                
            line_num += 1
        pix_paint.end()
        #find max width:
        width_list = [pix.width() for pix in pix_list]
        max_width = max(width_list)
        
        image = QImage(max_width, pix_list[0].height()*len(pix_list), QImage.Format_ARGB32_Premultiplied)
        image.fill(Qt.white)       
        painter = QPainter(image)  
        for pix in pix_list:
            y = pix_list[0].height()*pix_list.index(pix)
            painter.drawImage(0, y, pix)
        

        
        painter.end()
        return image
    
    def _prepared_text(self):
        return_pos_list = []
        for line in self.lines:
            return_pos_list.append(line.textStart())
        
        
        
        
        
        indent = self.text_format.textIndent()
        l_margin = self.text_format.leftMargin()
        r_margin = self.text_format.rightMargin()
        
        string = self.block_text
        final = ''
        i = 0
        for char in string:
            if i in return_pos_list and i != 0:
                final = "".join([final,"N"])# line return
            if char == "\t":
                final = "".join([final,"t"]) #tab
            if char.isalnum():
                final = "".join([final,"c"]) #char
            else:
                final = "".join([final,"_"]) #space
            i += 1
        return final
        
                
import time

class ImageGenerator(QThread):
    '''
    ImageGenerator
    '''
    image_generated = pyqtSignal('QPixmap', name="image_generated")
    def __init__(self):
        '''
        Constructor
        '''        
        super(ImageGenerator, self).__init__()
        
        self._old_item_list = {}
        self._old_hash_item_dict = {}
        self._max_width_found = 30
        
    def set_items(self, item_list):
        self._item_list = item_list
        
    def run(self):
        
        time.sleep(0.5)
        # create hash dict of current list
        self._hash_item_dict = {}              
        for item in self._item_list:
            self._hash_item_dict[item.hash] = item
                    
        # compare and swap between lists if new item is not generated
        for new_hash in self._hash_item_dict.keys():
            if new_hash in self._old_hash_item_dict.keys():
                # swap to avoid generating uselessly :
                self._hash_item_dict[new_hash] = self._old_hash_item_dict[new_hash]
            else: # generate
                _ = self._hash_item_dict[new_hash].image
                
        #append generated images
        #images_list.append()
        pix_list = []
        for new_item in self._item_list:
            usable_item = self._hash_item_dict[new_item.hash]
            pix_list.append(usable_item.image)
        
        # draw pixmaps in one :
        image = QImage(20,20, QImage.Format_ARGB32_Premultiplied)
        image.fill(Qt.white)
        point = QPoint(0,0)       
        width_list = [pix.width() for pix in pix_list]
        max_width = max(width_list)
        self._max_width_found = max([max_width, self._max_width_found])
        for pix in pix_list:

            last_image = image
            image = QImage(self._max_width_found, image.height() + pix.height(), QImage.Format_ARGB32_Premultiplied )
            pix_paint = QPainter(image)
            image.fill(Qt.white)
            pix_paint.drawImage(0,0, last_image)
            pix_paint.drawImage(point, pix)
            point = QPoint(0, point.y() + pix.height())
            pix_paint.end()
        '''
        pixmap = QImage(21,21, QImage.Format_ARGB32_Premultiplied) # temp
        pixPaint = QPainter(pixmap)
        myPen = QPen( Qt.black)
        pixPaint.setPen(myPen)
        pixPaint.drawRect(0,0, 20,20);
        pixPaint.end()
        image = pixmap
 
            painter.drawImage(point, pix)            
            point = QPoint(0, point.y() + pix.height())
        painter.end()
          '''  
            
        final_pixmap = QPixmap.fromImage(image, Qt.AutoColor)
        self.image_generated.emit(final_pixmap)
               
        self._old_item_list = self._item_list
        self._old_hash_item_dict = self._hash_item_dict
    