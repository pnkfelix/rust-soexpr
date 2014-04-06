default: open_gl # hello: $(DEMO)-dbg

RUSTC=rustc
DEMO_SRC=demo.rs
DEMO_DEPS=$(wildcard *.rs tests/*.rs)
DEMO=$(shell $(RUSTC) --crate-file-name $(DEMO_SRC))

# RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl/objdir-opt
RUST_SDL_LIBDIR=$(HOME)/opt/rustlibs

#do-demo: $(DEMO)-dbg $(DEMO)
do-demo: $(DEMO)-dbg
	./$<

#soe: $(DEMO)-dbg $(DEMO)
soe: $(DEMO)-dbg
	./$< $@

open_gl: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@

hello: $(DEMO)-dbg
	RUST_BACKTRACE=1 ./$< $@

testsprite: $(DEMO)-dbg $(DEMO)
	./$< $@

$(DEMO)-dbg: $(DEMO_SRC) $(DEMO_DEPS)
	$(RUSTC) -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args=" -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "

$(DEMO): $(DEMO_SRC) $(DEMO_DEPS)
	$(RUSTC) -O -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args=" -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "
