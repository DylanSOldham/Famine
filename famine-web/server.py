import http.server

HandlerClass = http.server.SimpleHTTPRequestHandler
HandlerClass.extensions_map['.js'] = 'text/javascript'
http.server.test(HandlerClass, port=80, bind="0.0.0.0")