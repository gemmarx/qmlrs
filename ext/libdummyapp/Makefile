
QMLS := empty.qml

TARGET := libdummyapp.a

RCC := rcc
CXX := c++
AR  := ar

all: $(TARGET)

dummy.qrc: $(QMLS)
	ls $^ |sed 's|\(.*\)|<file>\1</file>|;1i<RCC><qresource>' |sed '$$a</qresource></RCC>' >$@

dummy.cpp: dummy.qrc
	$(RCC) -name qml $< -o $@

dummy.o: dummy.cpp
	$(CXX) -c $^ -fPIC -nostdlib

libdummyapp.a: dummy.o
	$(AR) csr $@ $^

clean:
	-rm $(TARGET)
	-rm dummy.*

