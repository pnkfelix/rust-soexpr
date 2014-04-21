default: gl # hello # do-demo

RUSTC=rustc
DEMO_SRC=demo.rs
DEMO=$(shell $(RUSTC) --crate-file-name $(DEMO_SRC))

DEMO_DEPS=$(wildcard *.rs tests/*.rs) $(RUST_SDL_LIBDIR)/libsdl2-*-0.0.1.rlib
DEMO_LINK_ARGS= -framework CoreFoundation -framework CoreGraphics -framework AppKit

## These are some alternate definitions that, when used with a rust-sdl2 that
## is in the mac_dylib configuration rather than mac_framework, will set up
## some vague rebuild dependencies that may sometimes actually rebuild some
## of the things you want.
# DEMO_DEPS=$(wildcard *.rs tests/*.rs) $(CORE_SDL_LIBDIR)/libSDL2.a $(RUST_SDL_LIBDIR)/libsdl2-*-0.0.1.rlib
#DEMO_LINK_ARGS= -framework CoreFoundation -framework CoreGraphics -framework AppKit -L$(CORE_SDL_LIBDIR)

# RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl/objdir-opt
CORE_SDL_LIBDIR=$(HOME)/opt/sdl-master-dbg-nopt/lib
CORE_SDL_SRCDIR=$(HOME)/Dev/SDL/SDL-mirror/src
RUST_SDL_SRCDIR=$(HOME)/Dev/Rust/rust-sdl2
RUST_SDL_LIBDIR=$(HOME)/opt/rustlibs

$(CORE_SDL_LIBDIR)/libSDL2.a: $(CORE_SDL_SRCDIR)/video/cocoa/*.m
	make -C $(shell dirname $(CORE_SDL_SRCDIR))/objdir-dbg
	make -C $(shell dirname $(CORE_SDL_SRCDIR))/objdir-dbg install

$(RUST_SDL_LIBDIR)/libsdl2-%-0.0.1.rlib: $(RUST_SDL_SRCDIR)/src/sdl2/lib.rs $(RUST_SDL_SRCDIR)/src/sdl2/*.rs $(RUST_SDL_SRCDIR)/src/sdl2/*/*.rs  $(RUST_SDL_SRCDIR)/Makefile
	make -C $(RUST_SDL_SRCDIR)

#do-demo: $(DEMO)-dbg $(DEMO)
do-demo: $(DEMO)-dbg
	./$<

#soe: $(DEMO)-dbg $(DEMO)
soe: $(DEMO)-dbg
	./$< $@

gl: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@

open_gl: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@

open_gl_textures: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@ mix

hello: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@

testsprite: $(DEMO)-dbg $(DEMO)
	./$< $@

$(DEMO)-dbg: $(DEMO_SRC) $(DEMO_DEPS)
	$(RUSTC) -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args="$(DEMO_LINK_ARGS)"

$(DEMO): $(DEMO_SRC) $(DEMO_DEPS)
	$(RUSTC) -O -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args="$(DEMO_LINK_ARGS)"
