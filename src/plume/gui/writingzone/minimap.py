'''
Created on 11 mai 2015

@author: cyril
'''

from PyQt5.QtWidgets import QTextEdit, QMenu, QGraphicsView, QGraphicsScene, QLabel
from PyQt5.QtCore import pyqtSlot, Qt, QThread, pyqtSignal, QRectF
from PyQt5.QtGui import QPixmap, QPalette, QTextBlockFormat
from PyQt5.Qt import QPainterPath, QImage, QPainter, QPen, QPoint, QRect
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
        self._graphic_item_list = []
        
        self._text_edit = None
        
        
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
        

    
    @pyqtSlot()
    def update(self):
        self._generate_image(self.text_edit)
        
       

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
        self._graphic_item_list.append(item)
        
        self.setFixedWidth(pixmap.width())

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
        h = 3
        w = 2        
    
        def draw_char(pos, pen_color):
            pix_paint.setPen(QPen(pen_color))

            rect = QRect(i*w + i, 1, w, h)
            pix_paint.drawRect(rect)
            pix_paint.fillRect(rect, pen_color)

        

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
        for pix in pix_list:

            last_image = image
            image = QImage(max_width, image.height() + pix.height(), QImage.Format_ARGB32_Premultiplied )
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
    