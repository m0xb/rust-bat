import requests
from bs4 import BeautifulSoup
# curl -s -X POST http://codingbat.com/run --data 'id=p187868&code=public boolean sleepIn(boolean weekday, boolean vacation) {return true;}' | grep sleepIn | sed -E 's/.+&rarr;(.+?)<\/td><td>.+/\1/'
id = 'p187868'
code = 'public boolean sleepIn(boolean weekday, boolean vacation) {return true;}'
response = requests.post('http://codingbat.com/run', data={"id": id, 'code':code})
# print(response)
# print(response.text)
soup = BeautifulSoup(response.text, 'html.parser')
rows = soup.select('div table tr')
for row in list(rows)[1:]:
    # for child in row.children:
    #     print(child)
    invocation_td = next(row.children)
    print(invocation_td.text.strip())
# print(soup.prettify())
# print(rows)


