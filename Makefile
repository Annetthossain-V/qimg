
CXX = clang++
CXFLAGS := -Wall -Wextra

LDFLAGS := 

SRC := $(wildcard src/*.cxx)
OBJ := $(patsubst src/%.cxx,%.o,$(SRC))

all: build link clean

build: $(OBJ)

%.o: src/%.cxx 
		$(CXX) $(CXFLAGS) -c $< -o $@

link:
	$(CXX) $(LDFLAGS) $(OBJ) -o libimg


clean:
		rm -rf *.o
