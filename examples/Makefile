
QMLS := factorial_ui.qml multiply_ui.qml

TARGET := libqmlapp.a

RCC := rcc
CXX := c++
AR  := ar
RSC := rustc

BUILD_TYPE := debug
PREFIX     := ../target/$(BUILD_TYPE)

all: $(TARGET)

qml.qrc: $(QMLS)
	ls $^ |sed 's|\(.*\)|<file>\1</file>|;1i<RCC><qresource>' |sed '$$a</qresource></RCC>' >$@

qml.cpp: qml.qrc
	$(RCC) -name qml $< -o $@

qml.o: qml.cpp
	$(CXX) -c $^ -fPIC -nostdlib

libqmlapp.a: qml.o
	$(AR) csr $@ $^

factorial_qrc: factorial_qrc.rs factorial_ui.qml libqmlapp.a
	$(RSC) factorial_qrc.rs -o $(PREFIX)/examples/$@ -lstatic=qmlapp -Ldependency=$(PREFIX)/deps --extern qmlrs=$(PREFIX)/libqmlrs.rlib

clean:
	-rm *.a *.o
	-rm qml.cpp qml.qrc 

