default: do-demo

DEMO_SRC=demo.rs
DEMO_DEPS=$(wildcard *.rs tests/*.rs)
DEMO=$(shell rustc --crate-file-name $(DEMO_SRC))

SDL_1_2_INSTALL=$(HOME)/opt/sdl-release-1.2.15-dbg-nopt
SDL_1_2_ROOT=$(HOME)/Dev/Rust/rust-sdl/SDL-mirror
SDL_LIBDIR=$(SDL_1_2_INSTALL)/lib
SDL_INCLUDEDIR=$(SDL_1_2_INSTALL)/include/SDL

RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl

SDL_MAIN_M=$(SDL_1_2_ROOT)/src/main/macosx/SDLMain.m

do-demo: $(DEMO)
	./$< testsprite

$(DEMO): $(DEMO_SRC) $(DEMO_DEPS)
	rustc -o $@ -g $< -L$(RUST_SDL_LIBDIR) -L$(SDL_LIBDIR) -C link-args=" -I$(SDL_INCLUDEDIR) -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "
