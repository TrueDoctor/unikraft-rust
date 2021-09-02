UK_ROOT ?= $(PWD)/../../unikraft
UK_LIBS ?= $(PWD)/../../libs
LIBS := $(UK_LIBS)/lib-pthread-embedded:$(UK_LIBS)/lwip:$(UK_LIBS)/lib-compiler-rt:$(UK_LIBS)/libcxx:$(UK_LIBS)/libcxxabi:$(UK_LIBS)/lib-newlib:$(UK_LIBS)/lib-libunwind:$(UK_LIBS)/lib-librust

all:
	@$(MAKE) -C $(UK_ROOT) A=$(PWD) L=$(LIBS)

$(MAKECMDGOALS):
	@$(MAKE) -C $(UK_ROOT) A=$(PWD) L=$(LIBS) $(MAKECMDGOALS)
