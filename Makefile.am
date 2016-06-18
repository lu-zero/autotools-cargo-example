
bin_PROGRAMS = hello_cargo

hello_cargo_SOURCES = src/c/main.c
hello_cargo_LDADD = libcargo_example.a

libcargo_example.a:
	cd $(srcdir)/src/rust; cargo rustc -- --emit=link -o $(abs_builddir)/$@
