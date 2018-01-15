#literal functions
import re
def interger(s, index):
    ints = re.search('-?([0-9]+)', s[index:]).group(1)
    index = index + len(ints)
    return (index, ints)

def float(s, index):
    floats = re.search('-?([0-9]+)\.([0-9]+)', s[index:]).group(1)
    index = index + len(floats)
    return(index, floats)

def char(s, index):
    return (index + 1, s[index])

def string(s, index):
    string = re.search('("[A-Za-z0-9;.:\\\\[\](){}]*")', s[index:]).group(1)
    index = index + len(string)
    return (index, string)

def boolean(s, index):
    bool = re.search('true|false', s[index:index + 5]).group(1)
    index = index + len(bool)
    return (index, bool)

def array(s, index):
    pass