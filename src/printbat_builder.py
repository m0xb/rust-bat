import bat_scraper_2
def build_input(pid):
    #bat, type and signature are used to build the code variable, which in turn will be submitted to CodingBats to get the results table.
    bat = bat_scraper_2.get_bat(pid)
    type = bat_scraper_2.get_type(bat)
    signature = bat_scraper_2.generate_return(type)
    code = bat_scraper_2.generate_code(bat, signature)
    #Response is a list of strings, containing the rows from the results table on the CodingBats website.
    responses = bat_scraper_2.submit_code(code, pid)
    fn_name = bat_scraper_2.get_fn_name(responses[0])
    #List of tuples containing the various ivocations for the pid.
    invocation_list = []
    #List of tuples containing the expected results for the pid.
    expectation_list = []
    for row in responses:
        invocation_list.append(bat_scraper_2.get_invocation(row))
        expectation_list.append(bat_scraper_2.get_expected(row))
    return (fn_name, invocation_list, expectation_list)

def build_string(tup):
    macro_invocation = f'''\nprintbat!({tup[0]},
    '''
    #Argument positions in invocation_list which are vecs.
    vec_list = []
    for index in range(len(tup[1][0])):
        if isinstance(tup[1][0][index], list):
            vec_list.append(index)
    #Number of tests for a particular pid.
    item_count = len(tup[1])
    # for i in range(item_count):
    #     for j in tup[1][i]:
    #         if tup[1][i][j] in vec_list and i < item_count - 1:
    #             macro_invocation += ''
    #         elif tup[1][i][j] in vec_list and i == item_count - 1:
    #             macro_invocation += ''




# How do I deal with functions that take a String as an argument? (if Rust function takes a String, I have to append .to_string() on each literal)
# How do I del with functions which take a &str as an argument?  (I do not have to append .to_string() on each literal if the function takes a &str'


