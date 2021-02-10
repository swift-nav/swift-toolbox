# Resource object code (Python 3)
# Created by: object code
# Created by: The Resource Compiler for Qt version 5.15.2
# WARNING! All changes made in this file will be lost!

from PySide2 import QtCore

qt_resource_data = b"\
\x00\x00\x07\x96\
i\
mport QtQuick 2.\
12\x0aimport QtQuic\
k.Controls 2.12\x0a\
import QtCharts \
2.2\x0aimport QtQui\
ck.Layouts 1.15\x0a\
\x0aimport SwiftCon\
sole 1.0\x0a\x0aApplic\
ationWindow {\x0a\x0a \
   width: 640\x0a  \
  height: 480\x0a\x0a \
   font.pointSiz\
e: 8\x0a\x0a    Consol\
ePoints {\x0a      \
  id: console_po\
ints\x0a    }\x0a\x0a    \
ColumnLayout {\x0a\x0a\
        anchors.\
fill: parent\x0a   \
     anchors.mar\
gins: 4\x0a        \
spacing: 2\x0a\x0a    \
    ChartView {\x0a\
\x0a            Lay\
out.fillHeight: \
true\x0a           \
 Layout.fillWidt\
h: true\x0a\x0a       \
     legend.font\
.pointSize: 7\x0a  \
          titleF\
ont.pointSize: 8\
\x0a\x0a            ti\
tle: \x22Velocity\x22\x0a\
            anti\
aliasing: true\x0a\x0a\
            Line\
Series {\x0a       \
         id: vel\
ocity_graph\x0a    \
            name\
: \x22m/s\x22\x0a        \
        axisX: V\
alueAxis {\x0a     \
               i\
d: x_axis\x0a      \
              la\
belsFont.pointSi\
ze: 7\x0a          \
      }\x0a        \
        axisY: V\
alueAxis {\x0a     \
               i\
d: y_axis\x0a      \
              mi\
n: -1.0\x0a        \
            max:\
 1.0\x0a           \
         labelsF\
ont.pointSize: 7\
\x0a               \
 }\x0a             \
   //useOpenGL: \
true\x0a           \
 }\x0a\x0a            \
Timer {\x0a        \
        interval\
: 1000/5 // 5 Hz\
 refresh\x0a       \
         running\
: true\x0a         \
       repeat: t\
rue\x0a            \
    onTriggered:\
 {\x0a             \
       data_mode\
l.fill_console_p\
oints(console_po\
ints);\x0a         \
           if (!\
console_points.v\
alid) {\x0a        \
                \
return;\x0a        \
            }\x0a  \
                \
  var points = c\
onsole_points.po\
ints;\x0a          \
          var la\
st = points[poin\
ts.length - 1];\x0a\
                \
    x_axis.min =\
 last.x - 10;\x0a  \
                \
  x_axis.max = l\
ast.x;\x0a         \
           y_axi\
s.min = console_\
points.min_;\x0a   \
                \
 y_axis.max = co\
nsole_points.max\
_;\x0a             \
       console_p\
oints.fill_serie\
s(velocity_graph\
);\x0a             \
   }\x0a           \
 }\x0a        }\x0a\x0a  \
      Button {\x0a \
           text:\
 \x22Connect\x22\x0a     \
       onClicked\
: data_model.con\
nect()\x0a        }\
\x0a    }\x0a\x0a    Comp\
onent.onComplete\
d: {\x0a        vis\
ible = true;\x0a   \
 }\x0a}\x0a\
\x00\x00\x00\x1a\
[\
Controls]\x0aStyle=\
Material\x0a\
"

qt_resource_name = b"\
\x00\x08\
\x0f\xca[\xbc\
\x00v\
\x00i\x00e\x00w\x00.\x00q\x00m\x00l\
\x00\x15\
\x08\x1e\x16f\
\x00q\
\x00t\x00q\x00u\x00i\x00c\x00k\x00c\x00o\x00n\x00t\x00r\x00o\x00l\x00s\x002\x00.\
\x00c\x00o\x00n\x00f\
"

qt_resource_struct = b"\
\x00\x00\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x01\
\x00\x00\x00\x00\x00\x00\x00\x00\
\x00\x00\x00\x16\x00\x00\x00\x00\x00\x01\x00\x00\x07\x9a\
\x00\x00\x01wjtK\xe8\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\
\x00\x00\x01w\x8d\xc9f\xfb\
"


def qInitResources():
    QtCore.qRegisterResourceData(0x03, qt_resource_struct, qt_resource_name, qt_resource_data)


def qCleanupResources():
    QtCore.qUnregisterResourceData(0x03, qt_resource_struct, qt_resource_name, qt_resource_data)


qInitResources()
