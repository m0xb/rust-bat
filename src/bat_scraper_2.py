import unittest
import requests
from bs4 import BeautifulSoup
def get_type(pid):
    search_list = ['public ', 'private ', 'protected ']
    ignore_list = ['static ', 'final ']
    response = requests.post('http://codingbat.com/prob/', data=('id': pid))

    return ''

class Testget_type(unittest.TestCase):
    def test_bool(self):
        self.assertEqual('', get_type())
    def test_int_array(self):
        pass
    def test_string_array(self):
        pass
    def test_bool_array(self):
        pass
    def test_int(self):
        pass
    def test_string(self):
        pass
    def test_tup(self):
        pass
    def test_float(self):
        pass

if __name__ == '__main__':
    unittest.main()

