import unittest
import arg_list_parser
from arg_list_parser import *

class TestLiteralFunctions(unittest.TestCase):
    def test_integer(self):
        self.assertEqual(('ERROR: Expecting digit, got ', 0), arg_list_parser.integer('', 0))
        self.assertEqual((IntegerLiteral(12345), 5), arg_list_parser.integer('12345', 0))
        self.assertEqual((IntegerLiteral(234), 4), arg_list_parser.integer('1234', 1))
        self.assertEqual((IntegerLiteral(1234), 4), arg_list_parser.integer('1234.5678', 0))
        self.assertEqual((IntegerLiteral(123), 3), arg_list_parser.integer('123a345.789', 0))
        self.assertEqual(('ERROR: Expecting digit, got .', 0), arg_list_parser.integer('.1234', 0))

    def test_floating_point(self):
        self.assertEqual(('ERROR: Expecting float, got ', 0), arg_list_parser.floating_point('', 0))
        self.assertEqual(('ERROR: Expecting float, got 1', 0), arg_list_parser.floating_point('1', 0))
        self.assertEqual(('ERROR: Expecting float, got ', 4), arg_list_parser.floating_point('', 4))
        self.assertEqual(('ERROR: Expecting float, got ', 4), arg_list_parser.floating_point('1.0', 4))
        self.assertEqual(('ERROR: Expecting float, got .', 0), arg_list_parser.floating_point('.234', 0))
        # self.assertEqual((1., 2), arg_list_parser.floating_point('1..0', 0))
        self.assertEqual((FloatLiteral(1.), 2), arg_list_parser.floating_point('1..0', 0))
        self.assertEqual((FloatLiteral(1.1), 3), arg_list_parser.floating_point('1.1', 0))
        self.assertEqual((FloatLiteral(1234.0), 6), arg_list_parser.floating_point('1234.0', 0))
        self.assertEqual((FloatLiteral(1234.56789), 10), arg_list_parser.floating_point('1234.56789', 0))
        self.assertEqual((FloatLiteral(234.56789), 10), arg_list_parser.floating_point('1234.56789', 1))

    def test_character(self):
        self.assertEqual(('ERROR: Expecting char, got ', 0), arg_list_parser.character('', 0))
        self.assertEqual(('ERROR: Expecting char, got a', 0), arg_list_parser.character('ab', 0))
        self.assertEqual(('ERROR: Expecting char, got \'', 0), arg_list_parser.character('\'ab\'', 0))
        self.assertEqual(('ERROR: Expecting char, got \'', 0), arg_list_parser.character('\'\'', 0))
        self.assertEqual(('ERROR: Expecting char, got a', 1), arg_list_parser.character('\'a\'', 1))
        self.assertEqual(('ERROR: Expecting char, got b', 0), arg_list_parser.character('bcd\'a\'', 0))
        self.assertEqual(('\'a\'', 3), arg_list_parser.character('\'a\'', 0))
        self.assertEqual((CharLiteral('\'"\''), 3), arg_list_parser.character('\'"\'', 0))
        self.assertEqual((CharLiteral('\'"\''), 3), arg_list_parser.character('\'"\'', 0))
        self.assertEqual(('\'"\'', 3), arg_list_parser.character('\'"\'', 0))


    # def test_string(self):
    #     self.assertEqual(('ERROR: Expecting arg_list_parser.string, got ', 0), arg_list_parser.string('', 0))
    #     self.assertEqual(('ERROR: Expecting arg_list_parser.string, got a', 0), arg_list_parser.string('a""', 0))
    #     self.assertEqual(('ERROR: Expecting arg_list_parser.string, got ', 4), arg_list_parser.string('a""', 4))
    #     self.assertEqual(('ERROR: Expecting arg_list_parser.string, got "', 0), arg_list_parser.string('"', 0))
    #     self.assertEqual(('""', 2), arg_list_parser.string('""', 0))
    #     self.assertEqual(('""', 2), arg_list_parser.string('"""', 0))
    #     self.assertEqual(('""', 3), arg_list_parser.string('"""', 1))
    #     self.assertEqual(('"Hello"', 8), arg_list_parser.string('""Hello"', 1))
    #
    # def test_boolean(self):
    #     self.assertEqual(('ERROR: Expecting bool, got ', 0), arg_list_parser.boolean('', 0))
    #     self.assertEqual(('ERROR: Expecting bool, got r', 1), arg_list_parser.boolean('true', 1))
    #     self.assertEqual(('ERROR: Expecting bool, got a', 1), arg_list_parser.boolean('false', 1))
    #     self.assertEqual((True, 4), arg_list_parser.boolean('true', 0))
    #     self.assertEqual((False, 5), arg_list_parser.boolean('false', 0))
    #     self.assertEqual((True, 4), arg_list_parser.boolean('truetruetruetrue', 0))
    #     self.assertEqual((True, 8), arg_list_parser.boolean('truetruetruetrue', 4))
    #     self.assertEqual((False, 5), arg_list_parser.boolean('falsefalsefalsefalse', 0))
    #     self.assertEqual((False, 10), arg_list_parser.boolean('falsefalsefalsefalse', 5))
    #
    # def test_array(self):
    #     # self.assertEqual(('ERROR: Expecting [, got ', 0), arg_list_parser.array('', 0))
    #     # self.assertEqual(('ERROR: Expecting [, got ]', 0), arg_list_parser.array(']', 0))
    #     self.assertEqual(([], 2), arg_list_parser.array('[]', 0))
    #     self.assertEqual(([1], 3), arg_list_parser.array('[1]', 0))
    #     self.assertEqual(([1, 2, 2], 9), arg_list_parser.array('[1, 2, 2]', 0))
    #     self.assertEqual((['\'a\'', '\'b\'', '\'c\'', '\'d\''], 20), arg_list_parser.array('[\'a\', \'b\', \'c\', \'d\']', 0))
    #     self.assertEqual((['"Hello,"', '" there "', '"Dave!"'], 30), arg_list_parser.array('["Hello,", " there ", "Dave!"]', 0))
    #     self.assertEqual(([1.2, 1., 2.3456], 17), arg_list_parser.array('[1.2, 1., 2.3456]', 0))
    #     self.assertEqual(([True, False, True, False], 26), arg_list_parser.array('[true, false, true, false]', 0))
    #     self.assertEqual(([True, 12345, "\'a\'", '"SAD"', False, 1.234], 39), arg_list_parser.array('[true, 12345, \'a\', "SAD", false, 1.234]', 0))
    #
    #
    # def test_parse_scalar_literal(self):
    #     self.assertEqual((12345, 5), arg_list_parser.parse_scalar_literal('12345', 0))
    #     self.assertEqual((123.456, 7), arg_list_parser.parse_scalar_literal('123.456', 0))
    #     self.assertEqual(('\'c\'', 3), arg_list_parser.parse_scalar_literal('\'c\'', 0))
    #     self.assertEqual(('"Hi Alice!"', 11), arg_list_parser.parse_scalar_literal('"Hi Alice!"', 0))
    #     self.assertEqual((False, 5), arg_list_parser.parse_scalar_literal('false', 0))
    #
    # def test_parse_literal(self):
    #     #Non-Nested tests.
    #     self.assertEqual(('', 0), arg_list_parser.parse_literals('', 0))
    #     self.assertEqual((123, 3), arg_list_parser.parse_literals('123', 0))
    #     self.assertEqual(([1, 2, 3], 9), arg_list_parser.parse_literals('[1, 2, 3]', 0))
    #     #Nested Tests
    #     self.assertEqual(([1, 2, 3, [1, 2.2, [1]]], 24), arg_list_parser.parse_literals('[1, 2, 3, [1, 2.2, [1]]]', 0))
    #     self.assertEqual((']', 0), arg_list_parser.parse_literals(']', 0))
    #     self.assertEqual(([1, (1, 2, 3)], 14), arg_list_parser.parse_literals('[1, (1, 2, 3)]', 0))
    #     self.assertEqual(([1, 2, 2, [2, (3, 4)]], 22), arg_list_parser.parse_literals('[1, 2, 2, [2, (3, 4)]]', 0))
    #     self.assertEqual(([1, 2, 2, [2, (3, 4, [5])]], 26), arg_list_parser.parse_literals('[1, 2, 2, [2, (3, 4,[5])]]', 0))
    #
    #     # This test fails, arg_list_parser.array() does not have any fallback if it encounters a '[' without also encountering a ']'.
    #     # self.assertEqual(('[', 0), arg_list_parser.parse_literals('[', 0))
    #
    # def test_parse_tuple(self):
    #     self.assertEqual(((), 2), arg_list_parser.parse_tuple('()', 0))
    #     #This is an interesting result.
    #     self.assertEqual(((1,), 3), arg_list_parser.parse_tuple('(1)', 0))
    #     self.assertEqual(((1, 2), 6), arg_list_parser.parse_tuple('(1, 2)', 0))

if __name__ == '__main__':
    unittest.main()