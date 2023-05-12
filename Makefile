#
# Make system for step-hermit
#

APP = helloworld
LIB = libhermit-rs

all: build

build: $(APP)

$(APP): $(LIB) FORCE
	make -C $@

$(LIB): FORCE
	make -C $@

justrun:
	make -C $(APP) justrun

clean:
	make -C $(APP) clean
FORCE:

.PHONY: all build clean run justrun FORCE
