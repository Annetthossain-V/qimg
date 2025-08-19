
LIB_DIR =
LIB_INC =

BUILD_DIR = build

CXX = clang++
CXFLAGS := -Wall -Wextra -std=c++23 -I$(LIB_INC) -D_BUILD64XX -Wno-unused-parameter

LDFLAGS := 

SRC := $(wildcard src/*.cxx)
OBJ := $(patsubst src/%.cxx,%.o,$(SRC))

OBJ_BUILD = $(addprefix $(BUILD_DIR)/, $(OBJ))
OUTPUT_NAME = libqimg

ifdef Lib 
OUTPUT_NAME = libqimg.so
LDFLAGS += -shared
CXFLAGS += -D__LIB__
endif 

ifdef Release
CXFLAGS += -O3 -mtune=native -march=x86-64-v2 -fPIE -ffunction-sections -fdata-sections -fstack-protector-strong -D_FORTIFY_SOURCE=2 -flto
LDFLAGS += -flto -Wl,--gc-sections -s -pie -Wl,-z,relro -Wl,-z,now -Wl,-O2
endif

ifndef Release
CXFLAGS += -O0 -g
endif

all: build

build: $(OBJ) link

%.o: src/%.cxx 
		$(CXX) $(CXFLAGS) -c $< -o $@
		mv $@ $(BUILD_DIR)/

link:
	$(CXX) $(LDFLAGS) $(OBJ_BUILD) -o $(OUTPUT_NAME)


clean:
		rm -rf build/*.o
