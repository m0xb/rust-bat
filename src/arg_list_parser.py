#literal functions
import re
import unittest

def integer(s, index):
    m = re.match('-?([0-9]+)', s[index:])
    if m:
        token = m.group(0)
        return (int(token), index + len(token))
    else:
        return (f'ERROR: Expecting digit, got {s[index:index+1]}', index)

def floating_point(s, index):
    m = re.match('-?([0-9]+\.[0-9]*)', s[index:])
    if m:
        token = m.group(0)
        return (float(token), index + len(token))
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
    m = re.match('(true|false)', s[index:])
    if m:
        token = m.group(0)
        return (token == 'true', index + len(token))
    else:
        return (f'ERROR: Expecting bool, got {s[index:index+1]}', index)

def array(s, index):
    if index < len(s) and s[index] == '[':
        parsed_array = []
        s_index = index + 1
        while s_index < len(s):
            (value, new_index) = parse_scalar_literal(s, s_index)
            if s_index == new_index:
                return (parsed_array, s_index + 1)
            else:
                parsed_array.append(value)
                s_index = new_index
            if s[s_index] == ',':
                s_index += 1
                if s[s_index] == ' ':
                    s_index += 1
            if s[s_index] == ']':
                return (parsed_array, s_index + 1)
    else:
        return ('ERROR: Not an array.', index)

def parse_scalar_literal(s, index):
    literal_list = [parse_fn(s, index) for parse_fn in [integer, floating_point, character, string, boolean]]
    largest_index = 0
    for tuple in literal_list:
        if tuple[1] > largest_index:
            largest_index = tuple[1]
    for final in literal_list:
        if final[1] == largest_index:
            return final

class TestLiteralFunctions(unittest.TestCase):
    def test_interger(self):
        self.assertEqual(('ERROR: Expecting digit, got ', 0), integer('', 0))
        self.assertEqual((12345, 5), integer('12345', 0))
        self.assertEqual((234, 4), integer('1234', 1))
        self.assertEqual((1234, 4), integer('1234.5678', 0))
        self.assertEqual((123, 3), integer('123a345.789', 0))
        self.assertEqual(('ERROR: Expecting digit, got .', 0), integer('.1234', 0))

    def test_floating_point(self):
        self.assertEqual(('ERROR: Expecting float, got ', 0), floating_point('', 0))
        self.assertEqual(('ERROR: Expecting float, got 1', 0), floating_point('1', 0))
        self.assertEqual(('ERROR: Expecting float, got ', 4), floating_point('', 4))
        self.assertEqual(('ERROR: Expecting float, got ', 4), floating_point('1.0', 4))
        self.assertEqual(('ERROR: Expecting float, got .', 0), floating_point('.234', 0))
        self.assertEqual((1., 2), floating_point('1.', 0))
        self.assertEqual((1., 2), floating_point('1..0', 0))
        self.assertEqual((1.1, 3), floating_point('1.1', 0))
        self.assertEqual((1234.0, 6), floating_point('1234.0', 0))
        self.assertEqual((1234.56789, 10), floating_point('1234.56789', 0))
        self.assertEqual((234.56789, 10), floating_point('1234.56789', 1))

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
        self.assertEqual((True, 4), boolean('true', 0))
        self.assertEqual((False, 5), boolean('false', 0))
        self.assertEqual((True, 4), boolean('truetruetruetrue', 0))
        self.assertEqual((True, 8), boolean('truetruetruetrue', 4))
        self.assertEqual((False, 5), boolean('falsefalsefalsefalse', 0))
        self.assertEqual((False, 10), boolean('falsefalsefalsefalse', 5))

    def test_array(self):
        # self.assertEqual(('ERROR: Expecting [, got ', 0), array('', 0))
        # self.assertEqual(('ERROR: Expecting [, got ]', 0), array(']', 0))
         self.assertEqual(([], 2), array('[]', 0))
         self.assertEqual(([1], 3), array('[1]', 0))
         self.assertEqual(([1, 2, 2], 9), array('[1, 2, 2]', 0))
         self.assertEqual((['\'a\'', '\'b\'', '\'c\'', '\'d\''], 20), array('[\'a\', \'b\', \'c\', \'d\']', 0))
         self.assertEqual((['"Hello,"', '" there "', '"Dave!"'], 30), array('["Hello,", " there ", "Dave!"]', 0))
         self.assertEqual(([1.2, 1., 2.3456], 17), array('[1.2, 1., 2.3456]', 0))
         self.assertEqual(([True, False, True, False], 26), array('[true, false, true, false]', 0))


    def test_parse_scalar_literal(self):
        self.assertEqual((12345, 5), parse_scalar_literal('12345', 0))
        self.assertEqual((123.456, 7), parse_scalar_literal('123.456', 0))
        self.assertEqual(('\'c\'', 3), parse_scalar_literal('\'c\'', 0))
        self.assertEqual(('"Hi Alice!"', 11), parse_scalar_literal('"Hi Alice!"', 0))
        self.assertEqual((False, 5), parse_scalar_literal('false', 0))

if __name__ == '__main__':
    unittest.main()