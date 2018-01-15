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

def generate_return(type):
    generic_type_dict = {'int': 'return 0;',
                         'String': 'String a = "hello"; return a;',
                         'String[]': 'String[] a = {"Hello"}; return a;',
                         'int[]': 'int[] a = {0}; return a;',
                         'boolean': 'return true;',
                         'List<Integer>': 'List<Integer> a = new ArrayList<Integer>(); return a;',
                         'List<String>': 'List<String> a = new ArrayList<String>(); return a;',
                         'char': 'char a = \'a\'; return a;',
                         'char[]': 'char[] a = {\'a\'}; return a;',
                         'boolean[]': 'boolean[] a = {true}; return a;',
                         'float': 'a = 2.0; return a;',
                         'float[]': 'float[] a = {2.0}; return a;'}
    return generic_type_dict[type]

def generate_code(text, return_statement):
    return text[0:text.index('}')] + return_statement + '}'

def submit_code(code, pid):
    response = requests.post('http://codingbat.com/run', data={"id": pid, 'code':code})
    soup = BeautifulSoup(response.text, 'html.parser')
    rows = soup.select('div table tr')
    row_list = []
    for row in list(rows)[1:]:
        invocation_td = next(row.children)
        row_list.append(invocation_td.text.strip())
    return row_list[0:-1]

def get_expected(row):
    return re.search('→ ([A-Za-z0-9"[\]()!.?:;/\\\\, ]*)', row).group(1)



def get_invocation(row):
    pass



pid = 'p128796'
bat = get_bat(pid)
print(bat)
print(get_type(bat))
a = generate_code(bat, generate_return(get_type(bat)))
print(a)
b = submit_code(a, pid)
print(b)
for i in range(0, len(b)):
    print(get_expected(b[i]))



class Test_get_type(unittest.TestCase):
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

class Test_generate_return(unittest.TestCase):
    def test_int(self):
        self.assertEqual('return 0;', generate_return('int'))
    def test_String(self):
        self.assertEqual('String a = "hello"; return a;', generate_return('String'))
    def test_char(self):
        self.assertEqual('char a = \'a\'; return a;', generate_return('char'))
    def test_bool(self):
        self.assertEqual('return true;', generate_return('boolean'))
    def test_float(self):
        self.assertEqual('a = 2.0; return a;', generate_return('float'))
    def test_List_int(self):
        pass
    def test_List_String(self):
        pass
    def test_array_int(self):
        pass
    def test_array_String(self):
        pass
    def test_array_char(self):
        pass
    def test_array_bool(self):
        self.assertEqual('boolean[] a = {true}; return a;', generate_return('boolean[]'))
    def test_array_float(self):
        pass

class Test_generate_code(unittest.TestCase):
    def test_cigarParty(self):
        self.assertEqual('public boolean cigarParty(int cigars, boolean isWeekend) {a = true; return a;}', generate_code('public boolean cigarParty(int cigars, boolean isWeekend) {}', 'a = true; return a;'))

class Test_expected(unittest.TestCase):
    def test_repeatFront(self):
        self.assertEqual('"JavaJavJaJ"', get_expected('repeatFront("Java", 4) → "JavaJavJaJ"'))


if __name__ == '__main__':
    unittest.main()

