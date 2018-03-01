# Configure spec for the C++ version of RPGMAP
AC_PREREQ([2.69])
AC_INIT([RPG Map Generator], [0.01], [aaron.seilis@gmail.com],[rpgmap],[])
AM_INIT_AUTOMAKE([-Wall silent-rules foreign])
AC_CONFIG_SRCDIR([src/main.cpp])
AC_CONFIG_HEADERS([config.h])
AC_CONFIG_MACRO_DIRS([m4])

# Checks for programs.
AC_PROG_CXX
AX_CXX_COMPILE_STDCXX_17
AC_PROG_CC

# Libtool setup; explicitly disable static builds.
LT_INIT([disable-static])

# Checks for libraries.
PKG_CHECK_MODULES([DEPS], [cairo])

# Output files
AC_CONFIG_FILES([Makefile])
AC_OUTPUT
