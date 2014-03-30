default: soe # do-demo

DEMO_SRC=demo.rs
DEMO_DEPS=$(wildcard *.rs tests/*.rs)
DEMO=$(shell rustc --crate-file-name $(DEMO_SRC))

SDL_1_2_INSTALL_DBGNOPT=$(HOME)/opt/sdl-release-1.2.15-dbg-nopt
SDL_1_2_INSTALL_OPT=$(HOME)/opt/sdl-release-1.2.15
SDL_1_2_ROOT=$(HOME)/Dev/Rust/rust-sdl/SDL-mirror
SDL_LIBDIR_DBGNOPT=$(SDL_1_2_INSTALL_DBGNOPT)/lib
SDL_LIBDIR_OPT=$(SDL_1_2_INSTALL_OPT)/lib
SDL_INCLUDEDIR_DBGNOPT=$(SDL_1_2_INSTALL_DBGNOPT)/include/SDL
SDL_INCLUDEDIR_OPT=$(SDL_1_2_INSTALL_OPT)/include/SDL

# RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl/objdir-opt
RUST_SDL_LIBDIR=$(HOME)/opt/rustlibs

SDL_MAIN_M=$(SDL_1_2_ROOT)/src/main/macosx/SDLMain.m

#do-demo: $(DEMO)-dbg $(DEMO)
do-demo: $(DEMO)-dbg
	./$<

#soe: $(DEMO)-dbg $(DEMO)
soe: $(DEMO)-dbg
	./$< $@

testsprite: $(DEMO)-dbg $(DEMO)
	./$< $@

$(DEMO)-dbg: $(DEMO_SRC) $(DEMO_DEPS)
	rustc -o $@ -g $< -L$(RUST_SDL_LIBDIR) -L$(SDL_LIBDIR_DBGNOPT) -C link-args=" -I$(SDL_INCLUDEDIR_DBGNOPT) -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "

$(DEMO): $(DEMO_SRC) $(DEMO_DEPS)
	rustc -O -o $@ -g $< -L$(RUST_SDL_LIBDIR) -L$(SDL_LIBDIR_OPT) -C link-args=" -I$(SDL_INCLUDEDIR_OPT) -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "
