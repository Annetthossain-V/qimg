
CXX = clang++
CXFLAGS := -Wall -Wextra

SRC := $(wildcard src/*.cxx)
OBJ := $(patsubst src/%.cxx,%.o,$(SRC))

all: build link clean

build: $(OBJ)

%.o: src/%.cxx 
		$(CXX) $(CXFLAGS) -c $< -o $@

link:


clean:
		rm -rf *.o
