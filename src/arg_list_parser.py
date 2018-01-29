#literal functions
import re
import unittest

#AST Enhancement

#This class still does nothing.
class AstElement:
    pass

class Expression(AstElement):
    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.__dict__ == other.__dict__
        else:
            return


class IntegerLiteral(Expression):
    def __init__(self, intx):
        self.intx = intx
        if not isinstance(intx, int):
            raise Exception(f'Expected integer as argument, got {intx}')
    def __str__(self):
        return 'IntegerLiteral(' + str(self.intx) + ')'

    def to_rust_code(self):
        return f'{self.intx}'

class FloatLiteral(Expression):
    def __init__(self, floatx):
        self.floatx = floatx

    def __str__(self):
        return 'FloatLiteral(' + str(self.floatx) + ')'

    def to_rust_code(self):
        return f'{self.floatx}'

class CharLiteral(Expression):
    def __init__(self, charx):
        self.charx = charx

    def __str__(self):
         return 'CharLiteral(\'' + str(self.charx) + '\')'

    def to_rust_code(self):
        return f'{self.charx}'


class StringLiteral(Expression):
    def __init__(self, stringx):
        self.stringx = stringx

    def __str__(self):
        return 'StringLiteral("' + self.stringx + '")'

    def to_rust_code(self):
        return f'{self.stringx}'

class BooleanLiteral(Expression):
    def __init__(self, boolx):
        self.boolx = boolx

    def __str__(self):
        return 'BooleanLiteral(' + str(self.boolx).lower() + ')'

    def to_rust_code(self):
        return f'{self.boolx}'.lower()

class ArrayLiteral(Expression):
    def __init__(self, arrayx):
        self.arrayx = arrayx

    def __str__(self):
        string = 'ArrayLiteral('
        count = 0
        for elem in self.arrayx:
            if count < len(self.arrayx) - 1:
                string += str(elem) + ', '
                count += 1
            else:
                string += str(elem)
        return string + ')'

    def to_rust_code(self):
        return f'vec!{self.arrayx}'

class TupleLiteral(Expression):
    def __init__(self, tuplex):
        self.tuplex = tuplex
        if not isinstance(tuplex, tuple):
            raise Exception(f'Expected tuple as argument, got {tuplex}')

    def __str__(self):
        string = 'TupleLiteral('
        count = 0
        for elem in self.tuplex:
            if count < len(self.tuplex) - 1:
                string += str(elem) + ', '
                count += 1
            else:
                string += str(elem)
        return string + ')'

    def to_rust_code(self):
        return f'{self.tuplex}'


def integer(s, index):
    m = re.match('-?([0-9]+)', s[index:])
    if m:
        token = m.group(0)
        return (IntegerLiteral(int(token)), index + len(token))
    else:
        return (f'ERROR: Expecting digit, got {s[index:index+1]}', index)

def floating_point(s, index):
    m = re.match('-?([0-9]+\.[0-9]*)', s[index:])
    if m:
        token = m.group(0)
        return (FloatLiteral(float(token)), index + len(token))
    else:
        return (f'ERROR: Expecting float, got {s[index:index+1]}', index)

def character(s, index):
    m = re.match('\'[^\']\'', s[index:])
    if m:
        token = m.group(0)
        return (CharLiteral(token), index + len(token))
    else:
        return (f'ERROR: Expecting char, got {s[index:index+1]}', index)

def string(s, index):
    m = re.match('"[^"]*"', s[index:])
    if m:
        token = m.group(0)
        return (StringLiteral(token[1:-1]), index + len(token))
    else:
        return (f'ERROR: Expecting String, got {s[index:index+1]}', index)

def boolean(s, index):
    m = re.match('(true|false)', s[index:])
    if m:
        token = m.group(0)
        return (BooleanLiteral(token == 'true'), index + len(token))
    else:
        return (f'ERROR: Expecting bool, got {s[index:index+1]}', index)

def array(s, index):
    if index < len(s) and s[index] == '[':
        parsed_array = []
        s_index = index + 1
        while s_index < len(s):
            (value, new_index) = parse_literals(s, s_index)
            if s_index == new_index:
                return (ArrayLiteral(parsed_array), s_index + 1)
            else:
                parsed_array.append(value)
                s_index = new_index
            if s[s_index] == ',':
                s_index += 1
                if s[s_index] == ' ':
                    s_index += 1
            if s[s_index] == ']':
                return (ArrayLiteral(parsed_array), s_index + 1)
    else:
        return ('ERROR: Not an array.', index)

def parse_tuple(s, index):
    if index < len(s) and s[index] == '(':
        # Initially a list as tuples are immutable, will be converted to a tuple when returned.
        parsed_tuple = []
        s_index = index + 1
        while s_index < len(s):
            (value, new_index) = parse_literals(s, s_index)
            if s_index == new_index:
                return (TupleLiteral(tuple(parsed_tuple)), s_index + 1)
            else:
                parsed_tuple.append(value)
                s_index = new_index
            if s[s_index] == ',':
                s_index += 1
                if s[s_index] == ' ':
                    s_index += 1
            if s[s_index] == ')':
                return (TupleLiteral(tuple(parsed_tuple)), s_index + 1)
    else:
        return('ERROR: Not a tuple.', index)


def parse_scalar_literal(s, index):
    literal_list = [parse_fn(s, index) for parse_fn in [integer, floating_point, character, string, boolean]]
    largest_index = 0
    for tuple in literal_list:
        if tuple[1] > largest_index:
            largest_index = tuple[1]
    for final in literal_list:
        if final[1] == largest_index:
            return final

def parse_literals(s, index):
    (value, new_index) = parse_scalar_literal(s, index)
    (array_value, new_array_index) = array(s, index)
    (tuple_value, new_tuple_index) = parse_tuple(s, index)
    if new_array_index > max(new_index, new_tuple_index):
        return (array_value, new_array_index)
    elif new_tuple_index > max(new_index, new_array_index):
        return (tuple_value, new_tuple_index)
    elif new_index > max(new_array_index, new_tuple_index):
        return (value, new_index)
    else:
        return (s, index)



my_tuple = parse_tuple('(\'a\', "Hello", 1.1, 2, [1, 2, 3.3], true)', 0)
print(my_tuple)
#I haven't implemented the __str__() method correctly.  I understand from the spec on Google Docs what the __str__() method is supposed to do, but I'm not sure where/how to implement it.  What is a 'concrete' subclass?
#Outside of practicing with the syntax of inheritance in Python, what does the AstElement class do?  Does its lack of code have anything to do with why I can't get __str__ to work right?
#to_rust_code() is also not working, but I think this is due to problems outside the scope of that method.  Are the reasons to_rust_code() and __str__() are not working, the same?
#This is a expansion of my last question; am I instantiating my parser functions incorrectly?
print(my_tuple)