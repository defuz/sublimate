# -*- coding: utf-8 -*-
from .attributed import AttrFlow, AttrString


class VerticalSplitter(object):
    def __init__(self, canvas):
        self.canvas = canvas

    def __getitem__(self, s):
        free_height, ellipsis_count = self.canvas.height, 0
        for i in s:
            if i == Ellipsis:
                ellipsis_count += 1
            else:
                free_height -= i
        assert free_height >= 0, ellipsis_count <= 1
        pos = 0
        for i in s:
            height = free_height if i == Ellipsis else i
            yield SubCanvas(self.canvas, 0, pos, self.canvas.width, height)
            pos += height


class HorizontalSplitter(object):
    def __init__(self, canvas):
        self.canvas = canvas

    def __getitem__(self, s):
        free_width, ellipsis_count = self.canvas.width, 0
        for i in s:
            if i == Ellipsis:
                ellipsis_count += 1
            else:
                free_width -= i
        assert free_width >= 0, ellipsis_count <= 1
        pos = 0
        for i in s:
            width = free_width if i == Ellipsis else i
            yield SubCanvas(self.canvas, pos, 0, width, self.canvas.height)
            pos += width


class UrwidCanvasAdapter(object):

    def __init__(self, canvas):
        self.canvas = canvas
        self.cursor = None

    def content(self):
        def charset(s):
            if isinstance(s, unicode):
                return "U"
        def encode(s):
            if isinstance(s, unicode):
                return s.encode('utf8')
            return s
        for line in self.canvas:
            yield [(attr, charset(s), encode(s)) for attr, s in line]

    def cols(self):
        return self.canvas.width

    def rows(self):
        return self.canvas.height



class BaseCanvas(object):
    def __init__(self, width, height, style=None):
        self.width = width
        self.height = height
        self.style = style

    vert = property(VerticalSplitter)
    horz = property(HorizontalSplitter)

    def __iter__(self):
        raise NotImplementedError()

    def __getitem__(self):
        raise NotImplementedError()

    def set_style(self, style):
        if style:
            self.style = style
        return self

    def draw(self, attrstr, x=0, y=0):
        raise NotImplementedError()

    def get_mouse_target(self, x, y):
        raise NotImplementedError()

    def set_mouse_target(self, target, x=0, y=0, width=None, height=None):
        raise NotImplementedError()

    def overlay(self, x, y, width, height):
        return SubCanvas(self, x, y, width, height)

    def padding(self, left=0, right=0, top=0, bottom=0):
        assert left + right <= self.width
        assert top + bottom <= self.height
        width = self.width - left - right
        height = self.height - top - bottom
        return SubCanvas(self, left, top, width, height)

    def alignment(self, left_width, right_width):
        assert left_width + right_width <= self.width
        return (SubCanvas(self, 0, 0, left_width, self.height),
                SubCanvas(self, self.width - right_width, 0, right_width, self.height))

    def create_attrstr(self, text):
        return AttrString(text, attr=self.style)

    def create_solid(self, size, char=u' '):
        return self.create_attrstr(char * size)

    def draw_text(self, text, x=0, y=0):
        self.draw(self.create_attrstr(text), x, y)
        return self

    def draw_solid(self, size, char=' ', x=0, y=0):
        self.draw(self.create_solid(size, char), x, y)
        return self

    def draw_fill(self, char=' '):
        for i in range(self.height):
            self.draw_solid(self.width, char, y=i)
        return self

    def __str__(self):
        return 'Canvas(%s, %s, %s, %s)' % (self.x, self.y, self.width, self.height)


class Canvas(BaseCanvas):
    def __init__(self, width, height):
        BaseCanvas.__init__(self, width, height)
        self.y, self.x = 0, 0
        self.base_canvas = self
        self.data = [self.create_solid(self.width) for i in range(self.height)]
        self.mouse_target = [AttrFlow.fill(self.width) for i in range(self.height)]

    def __iter__(self):
        return iter(self.data)

    def __getitem__(self, index):
        return self.data[index]

    def draw(self, attrstr, x=0, y=0):
        assert y < self.height, x + len(attrstr) < self.width
        self.data[y][x:x+len(attrstr)] = attrstr

    def get_mouse_target(self, x, y):
        return self.mouse_target[y][x]

    def set_mouse_target(self, target, x=0, y=0, width=None, height=None):
        width = self.width if width is None else width
        height = self.height if height is None else height
        assert x + width <= self.width, y + height <= self.height
        for i in range(height):
            self.mouse_target[y+i][x:x+width] = AttrFlow.fill(width, target)
        return self

class SubCanvas(BaseCanvas):
    def __init__(self, base_canvas, x, y, width, height):
        assert x + width <= base_canvas.width, y + height <= base_canvas.height
        BaseCanvas.__init__(self, width, height, base_canvas.style)
        if isinstance(base_canvas, SubCanvas):
            x = base_canvas.x + x
            y = base_canvas.y + y
            base_canvas = base_canvas.base_canvas
        self.base_canvas = base_canvas
        self.x = x
        self.y = y

    def __iter__(self):
        for i in range(height):
            yield self[i]

    def __getitem__(self, index):
        return self.base_canvas[self.y+index][self.x:self.x+height]

    def draw(self, attrstr, x=0, y=0):
        assert y < self.height, x + len(attrstr) < self.width
        self.base_canvas.draw(attrstr, x=self.x+x, y=self.y+y)

    def get_mouse_target(self, x, y):
        return self.base_canvas.get_mouse_target(self.x + x, self.y + y)

    def set_mouse_target(self, target, x=0, y=0, width=None, height=None):
        width = self.width if width is None else width
        height = self.height if height is None else height
        assert x + width <= self.width, y + height <= self.height
        self.base_canvas.set_mouse_target(target, self.x+x, self.y+y, width, height)
        return self
