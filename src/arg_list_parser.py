#literal functions
import re
import unittest

def integer(s, index):
    m = re.match('-?([0-9]+)', s[index:])
    if m:
        token = m.group(1)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting digit, got {s[index:index+1]}', index)

def floating_point(s, index):
    m = re.match('-?([0-9]+)\.([0-9]*)', s[index:])
    if m:
        token = m.group(1)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting float, got {s[index:index+1]}', index)

def character(s, index):
    m = re.match('\'[^\']\'', s[index:])
    if m:
        token = m.group(1)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting char, got {s[index:index+1]}', index)


def string(s, index):
    m = re.match('"[^"]*"', s[index:])
    if m:
        token = m.group(1)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting String, got {s[index:index+1]}', index)

def boolean(s, index):
    m = re.search('(true|false)', s[index:])
    if m:
        token = m.group(1)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting bool, got {s[index:index+1]}', index)


def array(s, index):
    pass

class TestLiteralFunctions(unittest.TestCase):
    def test_interger(self):
        self.assertEqual(('ERROR: Expecting digit, got ', 0), integer('', 0))
        self.assertEqual(('12345', 5), integer('12345', 0))
        self.assertEqual(('234', 4), integer('1234', 1))
        self.assertEqual(('1234', 4), integer('1234.5678', 0))
        self.assertEqual(('123', 3), integer('123a345.789', 0))
        self.assertEqual(('ERROR: Expecting digit, got .', 0), integer('.1234', 0))
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