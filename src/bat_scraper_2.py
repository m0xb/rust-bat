import unittest
import re
import requests
from bs4 import BeautifulSoup
def get_bat(pid):
    response = requests.post(f'http://codingbat.com/prob/{pid}')
    return BeautifulSoup(response.text, 'html.parser').select('#ace_div')[0].text

def get_type(text):
    my_reg = re.search('(public|private|protected) ((static|final) )?([0-9a-zA-Z<>\[\]]*) ', text)
    return my_reg.group(4)

def generate_return(text, type):
    generic_type_dict = {'int': 'a = 0; return a;', 'String': 'String a = "hello; return a;', 'String[]': 'String[] a = \{\"Hello\"\}; return a;',
                         'int[]': 'int[] a = {0}; return a', 'boolean': 'a = true; return a', 'List<Integer>': 'List<Integer> a = new ArrayList<Integer>(); return a;',
                         'List<String>': 'List<String> a = new ArrayList<String>(); return a;', 'char': '', 'char[]': '', 'boolean[]': '', 'float': '', 'float[]': ''}
    pass

# bat = get_bat('p159531')
# print(bat)
# print(get_type(bat))

class Testget_type(unittest.TestCase):
    def test_bool(self):
        self.assertEqual('boolean', get_type('public boolean lucky13(int[] nums) {}'))
    def test_list_int(self):
        self.assertEqual('List<Integer>', get_type('public List<Integer> no9(List<Integer> nums) {}'))
    def test_list_string(self):
        self.assertEqual('List<String>', get_type('public List<String> noYY(List<String> strings) {}'))
    def test_string_array(self):
        self.assertEqual('String[]', get_type('public String[] fizzBuzz(int start, int end) {}'))
    def test_int_array(self):
        self.assertEqual('int[]', get_type('public int[] fizzArray(int n) {}'))
    def test_bool_array(self):
        #Not from a bat
        self.assertEqual('boolean[]', get_type('private static boolean[] boolMaster(boolean[] bools_for_fools) {}'))
    def test_char_array(self):
        #Not from a bat
        self.assertEqual('char[]', get_type('protected final char[] charCarp(int a, int b) {}'))
    def test_char(self):
        #Not from a bat
        self.assertEqual('char', get_type('public final char charCar(int a) {]'))
    def test_int(self):
        self.assertEqual('int', get_type('public int diff21(int n) {}'))
    def test_string(self):
        self.assertEqual('String', get_type('public String nonStart(String a, String b) {}'))
    def test_float(self):
        #Not from a bat
        self.assertEqual('float', get_type('public float floaty(float a, float b) {}'))

if __name__ == '__main__':
    unittest.main()

