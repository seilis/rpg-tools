#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([RPG Map Generator], [0.01], [aaron.seilis@gmail.com],[rpgmap],[])
AM_INIT_AUTOMAKE([-Wall silent-rules foreign])
AC_CONFIG_SRCDIR([src/main.cpp])
AC_CONFIG_HEADERS([config.h])

# Checks for programs.
AC_PROG_CXX
AX_CXX_COMPILE_STDCXX_11
#AC_PROG_LIBTOOL
AC_PROG_CC

# Checks for libraries.
PKG_CHECK_MODULES([DEPS],[cairo])
#AX_BOOST_BASE([0.1], [], [This project requires the Boost C++ Libraries!])

# Checks for header files.
#AC_CHECK_HEADERS([boost/program_options.hpp], [],
 #   [AC_MSG_ERROR(This program requires Boost.)])

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
