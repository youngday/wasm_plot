#!/usr/bin/env python3
import http.server
import socketserver
import os
import logging
from pathlib import Path

PORT = 8080
HOST = "0.0.0.0"
DIRECTORY = "dist"

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)
    
    def log_message(self, format, *args):
        logging.info("%s - %s" % (self.address_string(), format%args))

def main():
    # Configure logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(message)s',
        datefmt='%Y-%m-%d %H:%M:%S'
    )
    
    if not Path(DIRECTORY).exists():
        logging.error(f"Directory '{DIRECTORY}' not found!")
        return

    with socketserver.TCPServer((HOST, PORT), Handler) as httpd:
        logging.info(f"Serving WASM files from '{DIRECTORY}' at http://{HOST}:{PORT}")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            logging.info("Server stopped by user")
        except Exception as e:
            logging.error(f"Server error: {str(e)}")

if __name__ == "__main__":
    main()