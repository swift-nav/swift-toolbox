import sys
from PySide2.QtNetwork import QSslSocket

socket = QSslSocket()
socket.connectToHostEncrypted("google.com", 443)
if not socket.waitForEncrypted():
    print(socket.errorString())
    sys.exit(1)

socket.write(b"GET / HTTP/1.0\r\n\r\n")
while socket.waitForReadyRead():
    print(socket.readAll().data())
