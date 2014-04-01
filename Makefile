default: hello # soe # do-demo

DEMO_SRC=demo.rs
DEMO_DEPS=$(wildcard *.rs tests/*.rs)
DEMO=$(shell rustc --crate-file-name $(DEMO_SRC))

# RUST_SDL_LIBDIR=$(HOME)/Dev/Rust/rust-sdl/objdir-opt
RUST_SDL_LIBDIR=$(HOME)/opt/rustlibs

#do-demo: $(DEMO)-dbg $(DEMO)
do-demo: $(DEMO)-dbg
	./$<

#soe: $(DEMO)-dbg $(DEMO)
soe: $(DEMO)-dbg
	./$< $@

hello: $(DEMO)-dbg
	./$< $@

testsprite: $(DEMO)-dbg $(DEMO)
	./$< $@

$(DEMO)-dbg: $(DEMO_SRC) $(DEMO_DEPS)
	rustc -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args=" -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "

$(DEMO): $(DEMO_SRC) $(DEMO_DEPS)
	rustc -O -o $@ -g $< -L$(RUST_SDL_LIBDIR) -C link-args=" -framework CoreFoundation -framework CoreGraphics -framework AppKit $(SDL_MAIN_M)  "
