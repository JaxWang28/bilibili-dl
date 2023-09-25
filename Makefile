CFLAGS = -Wall -g
CFLAGS+=-DDEBUG

# 指定链接选项
LDFLAGS = -lcjson -lcurl
# 指定库文件目录
LIBS = -Llib/cjson

# 指定头文件目录
INCLUDES = -Iinclude

SOURCES = $(wildcard src/*.c)
OBJECTS = $(SOURCES:.c=.o)
TARGET = bbdl
OBJECTS = $(SOURCES:.c=.o)

$(TARGET) : $(OBJECTS) lib/cjson/libcjson.a
	$(CC) -o $@ $^ $(LDFLAGS)  $(LIBS)

# 编译源代码文件
%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

lib/cjson/libcjson.a:
	make -C ./lib

clean:
	rm -f *.o bbdl
	make -C ./lib clean


