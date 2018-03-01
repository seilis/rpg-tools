# Make sub-directory compiling nice
AUTOMAKE_OPTIONS = subdir-objects
ACLOCAL_AMFLAGS = -Im4

# Enable C++17 support on all platforms
AM_CXXFLAGS = --std=c++17 -O2 -Wall -Wextra

# Global flags
AM_CPPFLAGS = $(DEPS_CFLAGS) $(DEPS_CXXFLAGS)

# Program listing
bin_PROGRAMS = rpgmap

# Source files and library linking options for rpgmap
rpgmap_SOURCES = \
	src/main.cpp \
	src/gridcell.cpp \
	src/gridmap.cpp

# Note temporary addition of a hard-coded Boost library until I figure 
# out the best way to include it properly.
rpgmap_LDADD = $(DEPS_LIBS) -lboost_program_options $(AM_LDFLAGS)
