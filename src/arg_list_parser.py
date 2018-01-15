#literal functions
import re
import unittest

def integer(s, index):
    int_literal = re.search('-?([0-9]*)', s[index:]).group(1)
    index += len(int_literal)
    return (int_literal, index)


def floating_point(s, index):
    float_literal = re.search('-?([0-9]*)\.([0-9]*)', s[index:]).group(1)
    index += len(float_literal)
    return(float_literal, index)

def character(s, index):
    char_literal = re.search('\'[^\']\'', s[index:]).group(1)
    index += len(char_literal)
    return (char_literal, index)


def string(s, index):
    string_literal = re.search('"[^"]*"', s[index:]).group(1)
    index += len(string_literal)
    return (string_literal, index)

def boolean(s, index):
    bool_literal = re.search('(true|false)', s[index:]).group(1)
    index += len(bool_literal)
    return (bool_literal, index)


def array(s, index):
    pass

class TestLiteralFunctions(unittest.TestCase):
    def test_interger(self):
        self.assertEqual(('', 0), integer('', 0))
        self.assertEqual(('12345', 5), integer('12345', 0))
        self.assertEqual(('234', 4), integer('1234', 1))
        self.assertEqual(('1234', 4), integer('1234.5678', 0))
        self.assertEqual(('123', 3), integer('123a345.789', 0))
        self.assertEqual(('', 0), integer('.1234', 0))
    def test_floating_point(self):
        pass
    def test_character(self):
        pass
    def test_string(self):
        pass
    def test_boolean(self):
        pass
    def test_array(self):
        pass
if __name__ == '__main__':
    unittest.main()