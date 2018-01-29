import bat_scraper_2
def build_input(pid):
    #bat, type and signature are used to build the code variable, which in turn will be submitted to CodingBats to get the results table.
    bat = bat_scraper_2.get_bat(pid)
    type = bat_scraper_2.get_type(bat)
    return_statement = bat_scraper_2.generate_return(type)
    code = bat_scraper_2.generate_code(bat, return_statement)
    #Response is a list of strings, containing the rows from the results table on the CodingBats website.
    responses = bat_scraper_2.submit_code(code, pid)
    fn_name = bat_scraper_2.get_fn_name(responses[0])
    #List of tuples containing the various ivocations for the pid.
    invocation_list = []
    #List of tuples containing the expected results for the pid.
    expectation_list = []
    for row in responses:
        invocation_list.append(bat_scraper_2.get_invocation(row).to_rust_code())
        expectation_list.append(bat_scraper_2.get_expected(row).to_rust_code())
    return (fn_name, invocation_list, expectation_list)

def build_string(input_tuple):
    master_string = f'printbat!({input_tuple[0]},'
    for index in range(len(input_tuple[1])):
        if index < len(input_tuple[1]) - 1:
            master_string += f'\n    {input_tuple[1][index][1:-1]} => {input_tuple[2][index]},'
        else:
            master_string += f'\n    {input_tuple[1][index][1:-1]} => {input_tuple[2][index]});'
    return master_string

test = build_input('p105136')
print(build_string(test))

