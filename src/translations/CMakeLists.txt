

find_package(QT NAMES Qt6 Qt5 COMPONENTS LinguistTools REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS LinguistTools REQUIRED)


set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


# Update en_US only because it's the source lang for Transifex


set(LUPDATE_EXECUTABLE ${Qt${QT_VERSION_MAJOR}_LUPDATE_EXECUTABLE})
message(${LUPDATE_EXECUTABLE})
set(EN_US_FILENAME "${CMAKE_SOURCE_DIR}/src/translations/skribisto_en_US.ts")
message(${EN_US_FILENAME})
message("${LUPDATE_EXECUTABLE} ${CMAKE_SOURCE_DIR} -ts ${EN_US_FILENAME}")

add_custom_target("update_ts_en_US" ALL
  COMMAND ${LUPDATE_EXECUTABLE} ${CMAKE_SOURCE_DIR} -ts ${EN_US_FILENAME}
  DEPENDS ${EN_US_FILENAME}
)


# Release :

set(LRELEASE_EXECUTABLE ${Qt${QT_VERSION_MAJOR}_LRELEASE_EXECUTABLE})
message(${LRELEASE_EXECUTABLE})

file(GLOB_RECURSE TS_FILES CONFIGURE_DEPENDS "${CMAKE_SOURCE_DIR}/src/translations/skribisto_*.ts")

foreach(TS_FILEPATH ${TS_FILES})
      get_filename_component(TS_FILENAME ${TS_FILEPATH} NAME)
      string(REGEX MATCH "\\_([0-9a-zA-Z_]*)\\." LANGSTR ${TS_FILENAME})
      string(REPLACE "." ""  LANGSTR ${LANGSTR})
      set(QM_FILENAME "skribisto${LANGSTR}.qm")
      add_custom_target("release_ts_${LANGSTR}" ALL
        COMMAND ${LRELEASE_EXECUTABLE} ${TS_FILEPATH} -silent -qm ${QM_FILENAME}
        DEPENDS ${TS_FILEPATH}
      )

      if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
          install(FILES "${CMAKE_CURRENT_BINARY_DIR}/${QM_FILENAME}" DESTINATION "${KDE_INSTALL_DATADIR}/skribisto/translations")
      endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

endforeach(TS_FILEPATH)


# make the ts files visible in the IDE
add_custom_target(translations SOURCES ${TS_FILES})
