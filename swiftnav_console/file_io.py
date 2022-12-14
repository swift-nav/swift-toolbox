# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

from PySide6.QtCore import Property, Slot, Signal, QObject, QFile, QIODevice, QTextStream
import swiftnav_console.console_resources  # type: ignore # pylint: disable=unused-import


class FileIO(QObject):
    source_changed = Signal(str, arguments="source")
    text_changed = Signal()
    error = Signal(str, arguments="msg")

    def __init__(self, parent=None):
        super().__init__(parent)
        self._source = ""
        self._text = ""

    def get_source(self):
        return self._source

    @Slot(str)  # type: ignore
    def set_source(self, source):
        self._source = source
        self.source_changed.emit(source)  # type: ignore
        try:
            text = self.read()
            self.set_text(text)
        except IOError as e:
            print(e)

    @Slot()  # type: ignore
    def get_text(self):
        return self._text

    def set_text(self, text):
        self._text = text
        self.text_changed.emit()  # type: ignore

    @Slot()  # type: ignore
    def read(self):
        if not self._source:
            self.error.emit("source is empty")
            return ""

        file = QFile(self._source)
        fileContent = ""
        if file.open(QIODevice.ReadOnly):
            line = ""
            t = QTextStream(file)
            line = t.readLine()
            fileContent = line + "\n"
            lineno = 1
            while not t.atEnd():
                lineno += 1
                line = t.readLine()
                fileContent += line + "\n"
            file.close()
        else:
            self.error.emit("Unable to open the file")
            return ""
        return fileContent

    source = Property(str, get_source, set_source, notify=source_changed)  # type: ignore
    text = Property(str, get_text, notify=text_changed)  # type: ignore
