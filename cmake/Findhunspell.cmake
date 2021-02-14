find_library(HUNSPELL_LIBRARY NAMES ${PKG_HUNSPELL_LIBRARIES} "libhunspell" "hunspell" "hunspell-1.7" "hunspell-1.8" "hunspell/lib/x86_64-linux-gnu"
  PATHS "/usr/local/lib" ${SYSTEM_LIB_DIRS} "/usr/lib64" "/lib/x86_64-linux-gnu"
  HINTS ${PKG_HUNSPELL_LIBRARY_DIRS}
  )

find_path(HUNSPELL_INCLUDE_DIRS
    NAMES "hunspell.hxx"
    PATH_SUFFIXES hunspell
    HINTS ${PKG_HUNSPELL_INCLUDE_DIRS}
    )
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")
message("${PKG_HUNSPELL_LIBRARY_DIRS}")
message("${PKG_HUNSPELL_INCLUDE_DIRS}")
message("${HUNSPELL_INCLUDE_DIRS}")
message("${HUNSPELL_LIBRARY}")
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")
message("--------------------------------------------------------------")

# handle the QUIETLY and REQUIRED arguments and
# set HUNSPELL_FOUND to TRUE if all listed variables are TRUE
include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(hunspell DEFAULT_MSG HUNSPELL_LIBRARY HUNSPELL_INCLUDE_DIRS)





mark_as_advanced(HUNSPELL_LIBRARY HUNSPELL_INCLUDE_DIRS)
