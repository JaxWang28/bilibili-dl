libcjson.a: cjson.o
	ar rcs libcjson.a cjson.o

cjson.o:cJSON.c cJSON.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f libcjson.a cjson.o

.PHONY:
	clean
