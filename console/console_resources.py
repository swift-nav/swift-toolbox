# Resource object code (Python 3)
# Created by: object code
# Created by: The Resource Compiler for Qt version 5.15.2
# WARNING! All changes made in this file will be lost!

from PySide2 import QtCore

qt_resource_data = b"\
\x00\x00\x07\xe5\
i\
mport QtQuick 2.\
12\x0d\x0aimport QtQui\
ck.Controls 2.12\
\x0d\x0aimport QtChart\
s 2.2\x0d\x0aimport Qt\
Quick.Layouts 1.\
15\x0d\x0a\x0d\x0aimport Swi\
ftConsole 1.0\x0d\x0a\x0d\
\x0aApplicationWind\
ow {\x0d\x0a\x0d\x0a    widt\
h: 640\x0d\x0a    heig\
ht: 480\x0d\x0a\x0d\x0a    f\
ont.pointSize: 8\
\x0d\x0a\x0d\x0a    ConsoleP\
oints {\x0d\x0a       \
 id: console_poi\
nts\x0d\x0a    }\x0d\x0a\x0d\x0a  \
  ColumnLayout {\
\x0d\x0a\x0d\x0a        anch\
ors.fill: parent\
\x0d\x0a        anchor\
s.margins: 4\x0d\x0a  \
      spacing: 2\
\x0d\x0a\x0d\x0a        Char\
tView {\x0d\x0a\x0d\x0a     \
       Layout.fi\
llHeight: true\x0d\x0a\
            Layo\
ut.fillWidth: tr\
ue\x0d\x0a\x0d\x0a          \
  legend.font.po\
intSize: 7\x0d\x0a    \
        titleFon\
t.pointSize: 8\x0d\x0a\
\x0d\x0a            ti\
tle: \x22Velocity\x22\x0d\
\x0a            ant\
ialiasing: true\x0d\
\x0a\x0d\x0a            L\
ineSeries {\x0d\x0a   \
             id:\
 velocity_graph\x0d\
\x0a               \
 name: \x22m/s\x22\x0d\x0a  \
              ax\
isX: ValueAxis {\
\x0d\x0a              \
      id: x_axis\
\x0d\x0a              \
      labelsFont\
.pointSize: 7\x0d\x0a \
               }\
\x0d\x0a              \
  axisY: ValueAx\
is {\x0d\x0a          \
          id: y_\
axis\x0d\x0a          \
          min: -\
1.0\x0d\x0a           \
         max: 1.\
0\x0d\x0a             \
       labelsFon\
t.pointSize: 7\x0d\x0a\
                \
}\x0d\x0a             \
   //useOpenGL: \
true\x0d\x0a          \
  }\x0d\x0a\x0d\x0a         \
   Timer {\x0d\x0a    \
            inte\
rval: 1000/5 // \
5 Hz refresh\x0d\x0a  \
              ru\
nning: true\x0d\x0a   \
             rep\
eat: true\x0d\x0a     \
           onTri\
ggered: {\x0d\x0a     \
               d\
ata_model.fill_c\
onsole_points(co\
nsole_points);\x0d\x0a\
                \
    if (!console\
_points.valid) {\
\x0d\x0a              \
          return\
;\x0d\x0a             \
       }\x0d\x0a      \
              va\
r points = conso\
le_points.points\
;\x0d\x0a             \
       var last \
= points[points.\
length - 1];\x0d\x0a  \
                \
  x_axis.min = l\
ast.x - 10;\x0d\x0a   \
                \
 x_axis.max = la\
st.x;\x0d\x0a         \
           y_axi\
s.min = console_\
points.min;\x0d\x0a   \
                \
 y_axis.max = co\
nsole_points.max\
;\x0d\x0a             \
       console_p\
oints.fill_serie\
s(velocity_graph\
);\x0d\x0a            \
    }\x0d\x0a         \
   }\x0d\x0a        }\x0d\
\x0a\x0d\x0a        Butto\
n {\x0d\x0a           \
 text: \x22Connect\x22\
\x0d\x0a            on\
Clicked: data_mo\
del.connect()\x0d\x0a \
       }\x0d\x0a    }\x0d\
\x0a\x0d\x0a    Component\
.onCompleted: {\x0d\
\x0a        visible\
 = true;\x0d\x0a    }\x0d\
\x0a}\x0d\x0a\
\x00\x00\x00\x1c\
[\
Controls]\x0d\x0aStyle\
=Material\x0d\x0a\
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
\x00\x00\x00\x16\x00\x00\x00\x00\x00\x01\x00\x00\x07\xe9\
\x00\x00\x01w_z\xd6\xc5\
\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\
\x00\x00\x01wq?\x9f\xc9\
"

def qInitResources():
    QtCore.qRegisterResourceData(0x03, qt_resource_struct, qt_resource_name, qt_resource_data)

def qCleanupResources():
    QtCore.qUnregisterResourceData(0x03, qt_resource_struct, qt_resource_name, qt_resource_data)

qInitResources()
