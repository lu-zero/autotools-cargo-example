ACLOCAL_AMFLAGS = -I m4


bin_PROGRAMS = hello_cargo

hello_cargo_SOURCES = src/c/main.c
hello_cargo_LDADD = libhello.la

lib_LTLIBRARIES = libhello.la

am_libhello_la_OBJECTS = hello.o
libhello_la_SOURCES = ""
libhello_la_LIBADD = libcargo_example.a

hello.o:
	cd $(srcdir)/src/rust; cargo rustc -- --emit=obj -o $(abs_builddir)/hello.o

libcargo_example.a:
	cd $(srcdir)/src/rust; cargo rustc -- --emit=link -o $(abs_builddir)/$@
