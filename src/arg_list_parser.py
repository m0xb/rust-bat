#literal functions
import re
def interger(s, index):
    int_literal = re.search('-?([0-9]+)', s[index:]).group(1)
    index = index + len(int_literal)
    return (index, int_literal)

def float(s, index):
    float_literal = re.search('-?([0-9]+)\.([0-9]+)', s[index:]).group(1)
    index = index + len(float_literal)
    return(index, float_literal)

def char(s, index):
    char_literal = re.search('\'[^\']\'', s[index:])
    return (index + 1, char_literal)

def string(s, index):
    string_literal = re.search('"[^"]*"', s[index:]).group(1)
    index = index + len(string_literal)
    return (index, string)

def boolean(s, index):
    bool_literal = re.search('(true|false)', s[index:]).group(1)
    index = index + len(bool_literal)
    return (index, bool_literal)

def array(s, index):
    pass