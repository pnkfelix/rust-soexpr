DEMO_SRC=demo.rs
DEMO=$(shell rustc --crate-file-name $(DEMO_SRC))

SDL_1_2_INSTALL=$(HOME)/opt/sdl-release-1.2.15-dbg-nopt
SDL_1_2_ROOT=$(HOME)/Dev/Rust/rust-sdl/SDL-mirror
SDL_LIBDIR=$(SDL_1_2_INSTALL)/lib
SDL_INCLUDEDIR=$(SDL_1_2_INSTALL)/include/SDL

RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl

SDL_MAIN_M=$(SDL_1_2_ROOT)/src/main/macosx/SDLMain.m
default: $(DEMO)

$(DEMO): $(DEMO_SRC)
	rustc -o $@ -g $< -L$(RUST_SDL_LIBDIR) -L$(SDL_LIBDIR) -C link-args=" -I$(SDL_INCLUDEDIR) -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "
