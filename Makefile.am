ACLOCAL_AMFLAGS = -I m4

bin_PROGRAMS = hello_cargo

hello_cargo_SOURCES = src/c/main.c
hello_cargo_LDADD = libhello.la
# Need to work out how to get this from rustc
hello_cargo_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

lib_LTLIBRARIES = libhello.la

am_libhello_la_OBJECTS = hello.o
libhello_la_SOURCES = ""
libhello_la_LIBADD = libcargo_example.a

hello.o:
	cd $(srcdir)/src/rust; cargo rustc -- --emit obj=$(abs_builddir)/hello.o

libcargo_example.a:
	cd $(srcdir)/src/rust; cargo rustc -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

clean-local:
	cd $(srcdir)/src/rust; cargo clean

