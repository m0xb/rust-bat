#literal functions
import re
import unittest

def integer(s, index):
    m = re.match('-?([0-9]+)', s[index:])
    if m:
        token = m.group(0)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting digit, got {s[index:index+1]}', index)

def floating_point(s, index):
    m = re.match('-?([0-9]+\.[0-9]*)', s[index:])
    if m:
        token = m.group(0)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting float, got {s[index:index+1]}', index)

def character(s, index):
    m = re.match('\'[^\']\'', s[index:])
    if m:
        token = m.group(0)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting char, got {s[index:index+1]}', index)

def string(s, index):
    m = re.match('"[^"]*"', s[index:])
    if m:
        token = m.group(0)
        return (token, index + len(token))
    else:
        return (f'ERROR: Expecting String, got {s[index:index+1]}', index)

def boolean(s, index):
    m = re.search('(true|false)', s[index:])
    if m:
        token = m.group(0)
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
        self.assertEqual(('ERROR: Expecting float, got ', 0), floating_point('', 0))
        self.assertEqual(('ERROR: Expecting float, got 1', 0), floating_point('1', 0))
        self.assertEqual(('ERROR: Expecting float, got ', 4), floating_point('', 4))
        self.assertEqual(('ERROR: Expecting float, got ', 4), floating_point('1.0', 4))
        self.assertEqual(('ERROR: Expecting float, got .', 0), floating_point('.234', 0))
        self.assertEqual(('1.', 2), floating_point('1.', 0))
        self.assertEqual(('1.', 2), floating_point('1..0', 0))
        self.assertEqual(('1.1', 3), floating_point('1.1', 0))
        self.assertEqual(('1234.0', 6), floating_point('1234.0', 0))
        self.assertEqual(('1234.56789', 10), floating_point('1234.56789', 0))
        self.assertEqual(('234.56789', 10), floating_point('1234.56789', 1))

    def test_character(self):
        self.assertEqual(('ERROR: Expecting char, got ', 0), character('', 0))
        self.assertEqual(('ERROR: Expecting char, got a', 0), character('ab', 0))
        self.assertEqual(('ERROR: Expecting char, got \'', 0), character('\'ab\'', 0))
        self.assertEqual(('ERROR: Expecting char, got \'', 0), character('\'\'', 0))
        self.assertEqual(('ERROR: Expecting char, got a', 1), character('\'a\'', 1))
        self.assertEqual(('ERROR: Expecting char, got b', 0), character('bcd\'a\'', 0))
        self.assertEqual(('\'a\'', 3), character('\'a\'', 0))
        self.assertEqual(('\'"\'', 3), character('\'"\'', 0))

    def test_string(self):
        self.assertEqual(('ERROR: Expecting String, got ', 0), string('', 0))
        self.assertEqual(('ERROR: Expecting String, got a', 0), string('a""', 0))
        self.assertEqual(('ERROR: Expecting String, got ', 4), string('a""', 4))
        self.assertEqual(('ERROR: Expecting String, got "', 0), string('"', 0))
        self.assertEqual(('""', 2), string('""', 0))
        self.assertEqual(('""', 2), string('"""', 0))
        self.assertEqual(('""', 3), string('"""', 1))
        self.assertEqual(('"Hello"', 8), string('""Hello"', 1))

    def test_boolean(self):
        self.assertEqual(('ERROR: Expecting bool, got ', 0), boolean('', 0))
        self.assertEqual(('ERROR: Expecting bool, got r', 1), boolean('true', 1))
        self.assertEqual(('ERROR: Expecting bool, got a', 1), boolean('false', 1))
        self.assertEqual(('true', 4), boolean('true', 0))
        self.assertEqual(('false', 5), boolean('false', 0))
        self.assertEqual(('true', 4), boolean('truetruetruetrue', 0))
        self.assertEqual(('true', 8), boolean('truetruetruetrue', 4))
        self.assertEqual(('false', 5), boolean('falsefalsefalsefalse', 0))
        self.assertEqual(('false', 10), boolean('falsefalsefalsefalse', 5))

    def test_array(self):
        pass

if __name__ == '__main__':
    unittest.main()