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
            (value, new_index) = parse_literals(s, s_index)
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

def parse_tuple(s, index):
    if index < len(s) and s[index] == '(':
        # Initially a list as tuples are immutable, will be converted to a tuple when returned.
        parsed_tuple = []
        s_index = index + 1
        while s_index < len(s):
            (value, new_index) = parse_literals(s, s_index)
            if s_index == new_index:
                return (tuple(parsed_tuple), s_index + 1)
            else:
                parsed_tuple.append(value)
                s_index = new_index
            if s[s_index] == ',':
                s_index += 1
                if s[s_index] == ' ':
                    s_index += 1
            if s[s_index] == ')':
                return (tuple(parsed_tuple), s_index + 1)
    else:
        return('ERROR: Not a tuple.', index)


def parse_scalar_literal(s, index):
    literal_list = [parse_fn(s, index) for parse_fn in [integer, floating_point, character, string, boolean]]
    largest_index = 0
    for tuple in literal_list:
        if tuple[1] > largest_index:
            largest_index = tuple[1]
    for final in literal_list:
        if final[1] == largest_index:
            return final

def parse_literals(s, index):
    (value, new_index) = parse_scalar_literal(s, index)
    (array_value, new_array_index) = array(s, index)
    (tuple_value, new_tuple_index) = parse_tuple(s, index)
    if new_array_index > max(new_index, new_tuple_index):
        return (array_value, new_array_index)
    elif new_tuple_index > max(new_index, new_array_index):
        return (tuple_value, new_tuple_index)
    elif new_index > max(new_array_index, new_tuple_index):
        return (value, new_index)
    else:
        return (s, index)

