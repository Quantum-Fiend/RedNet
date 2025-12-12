# C Packet Sniffer Makefile

CC = gcc
CFLAGS = -Wall -Wextra -O2 -fPIC -DBUILDING_DLL
INCLUDES = -Iinclude
LDFLAGS = -shared

# Platform detection
ifeq ($(OS),Windows_NT)
    TARGET = lib/packet_sniffer.dll
    LDFLAGS += -lws2_32
else
    TARGET = lib/libpacket_sniffer.so
endif

SOURCES = src/raw_socket.c
OBJECTS = $(SOURCES:.c=.o)

all: $(TARGET)

$(TARGET): $(OBJECTS)
	@mkdir -p lib
	$(CC) $(LDFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)

test:
	@echo "Running static analysis..."
	cppcheck --enable=all --suppress=missingIncludeSystem $(SOURCES)

.PHONY: all clean test
