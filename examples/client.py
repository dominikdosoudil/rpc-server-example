from urllib import request

try:
	print(request.urlopen("http://localhost:8000/hello").read())
except Exception as e:
	print(e)
