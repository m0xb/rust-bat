import bat_scraper_2
import unittest


class Test_get_type(unittest.TestCase):
    def test_bool(self):
        self.assertEqual('boolean', bat_scraper_2.get_type('public boolean lucky13(int[] nums) {}'))
    def test_list_int(self):
        self.assertEqual('List<Integer>', bat_scraper_2.get_type('public List<Integer> no9(List<Integer> nums) {}'))
    def test_list_string(self):
        self.assertEqual('List<String>', bat_scraper_2.get_type('public List<String> noYY(List<String> strings) {}'))
    def test_string_array(self):
        self.assertEqual('String[]', bat_scraper_2.get_type('public String[] fizzBuzz(int start, int end) {}'))
    def test_int_array(self):
        self.assertEqual('int[]', bat_scraper_2.get_type('public int[] fizzArray(int n) {}'))
    def test_bool_array(self):
        #Not from a bat
        self.assertEqual('boolean[]', bat_scraper_2.get_type('private static boolean[] boolMaster(boolean[] bools_for_fools) {}'))
    def test_char_array(self):
        #Not from a bat
        self.assertEqual('char[]', bat_scraper_2.get_type('protected final char[] charCarp(int a, int b) {}'))
    def test_char(self):
        #Not from a bat
        self.assertEqual('char', bat_scraper_2.get_type('public final char charCar(int a) {]'))
    def test_int(self):
        self.assertEqual('int', bat_scraper_2.get_type('public int diff21(int n) {}'))
    def test_string(self):
        self.assertEqual('String', bat_scraper_2.get_type('public String nonStart(String a, String b) {}'))
    def test_float(self):
        #Not from a bat
        self.assertEqual('float', bat_scraper_2.get_type('public float floaty(float a, float b) {}'))

class Test_generate_return(unittest.TestCase):
    def test_int(self):
        self.assertEqual('return 0;', bat_scraper_2.generate_return('int'))
    def test_String(self):
        self.assertEqual('String a = "hello"; return a;', bat_scraper_2.generate_return('String'))
    def test_char(self):
        self.assertEqual('char a = \'a\'; return a;', bat_scraper_2.generate_return('char'))
    def test_bool(self):
        self.assertEqual('return true;', bat_scraper_2.generate_return('boolean'))
    def test_float(self):
        self.assertEqual('a = 2.0; return a;', bat_scraper_2.generate_return('float'))
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
        self.assertEqual('boolean[] a = {true}; return a;', bat_scraper_2.generate_return('boolean[]'))
    def test_array_float(self):
        pass

class Test_generate_code(unittest.TestCase):
    def test_cigarParty(self):
        self.assertEqual('public boolean cigarParty(int cigars, boolean isWeekend) {a = true; return a;}', bat_scraper_2.generate_code('public boolean cigarParty(int cigars, boolean isWeekend) {}', 'a = true; return a;'))

class Test_expected(unittest.TestCase):
    def test_repeatFront(self):
        self.assertEqual('"JavaJavJaJ"', bat_scraper_2.get_expected('repeatFront("Java", 4) → "JavaJavJaJ"'))


if __name__ == '__main__':
    unittest.main()
