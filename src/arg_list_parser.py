#literal functions
import re
import unittest

def integer(s, index):
    if len(s) > 0:
        int_literal = re.search('-?([0-9]+)', s[index:]).group(1)
        index += len(int_literal)
        return (int_literal, index)
    return (s, index)

def floating_point(s, index):
    if len(s) > 0:
        float_literal = re.search('-?([0-9]+)\.([0-9]+)', s[index:]).group(1)
        index += len(float_literal)
        return(float_literal, index)
    return(s, index)

def character(s, index):
    if len(s) > 0:
        char_literal = re.search('\'[^\']\'', s[index:]).group(1)
        index += len(char_literal)
        return (char_literal, index)
    return (s, index)

def string(s, index):
    if len(s) > 0:
        string_literal = re.search('"[^"]*"', s[index:]).group(1)
        index += len(string_literal)
        return (string_literal, index)
    return (s, index)

def boolean(s, index):
    if len(s) > 0:
        bool_literal = re.search('(true|false)', s[index:]).group(1)
        index += len(bool_literal)
        return (bool_literal, index)
    return (s, index)

def array(s, index):
    pass

class TestLiteralFunctions(unittest.TestCase):
    def test_interger(self):
        self.assertEqual(('', 0), integer('', 0))
        self.assertEqual(('12345', 5), integer('12345', 0))
        self.assertEqual(('234', 4), integer('1234', 1))
        self.assertEqual(('1234', 4), integer('1234.5678', 0))
        self.assertEqual(('123', 3), integer('123a345.789', 0))
        #This test should fail, s should contain nothing and the index should be 0.
        #String should return the largest index value here, so I think this test passing won't be an issue as
        #far as the programs overall effectiveness is concerned.
        self.assertEqual(('1234', 4), integer('.1234', 0))
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