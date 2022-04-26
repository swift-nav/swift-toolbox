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
