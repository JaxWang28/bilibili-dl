CFLAGS+=-lcurl
CFLAGS+=-lcjson
CFLAGS+=-Lcjson
CFLAGS+=-DDEBUG

BIN=bbdl

SOURCES = main.c login.c api.c

OBJECTS = $(SOURCES:.c=.o)

$(BIN):$(OBJECTS) cjson/libcjson.a
	$(CC) $(OBJECTS) $(CFLAGS) -o $(BIN)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

cjson/libcjson.a:
	make -C ./cjson
clean:
	rm -f *.o bbdl
	make -C ./cjson clean
