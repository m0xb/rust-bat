import re
import requests
from bs4 import BeautifulSoup
import arg_list_parser

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
    expected = re.search('→ (.*)', row).group(1)
    return arg_list_parser.parse_literals(expected, 0)[0]

def get_fn_name(row):
    return re.search('([^(]*)', row).group(0)

def get_invocation(row):
    inv = re.search('(\(.*\))', row).group(1)
    return arg_list_parser.parse_literals(inv, 0)[0]





