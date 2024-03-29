

find_package(QT NAMES Qt6 COMPONENTS LinguistTools REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS LinguistTools REQUIRED)


set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


# Update en_US only because it's the source lang for Transifex


set(LUPDATE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lupdate")
if(NOT EXISTS "${LUPDATE_EXECUTABLE}")
    set(LUPDATE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lupdate-qt6")
endif()
if(NOT EXISTS "${LUPDATE_EXECUTABLE}")
    set(LUPDATE_EXECUTABLE "/usr/lib/qt6/bin/lupdate")
endif()
if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    if(EXISTS "${CMAKE_PREFIX_PATH}/../../bin/lupdate.exe")
        set(LUPDATE_EXECUTABLE "${CMAKE_PREFIX_PATH}/../../bin/lupdate.exe")
    else()
        set(LUPDATE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lupdate.exe")
    endif()
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")

message(${LUPDATE_EXECUTABLE})
set(EN_US_FILENAME "${CMAKE_SOURCE_DIR}/src/translations/skribisto_en_US.ts")
message(${EN_US_FILENAME})
message("${LUPDATE_EXECUTABLE} ${CMAKE_SOURCE_DIR} -ts ${EN_US_FILENAME}")

if(ANDROID OR IOS)
else()

    add_custom_target("update_ts_en_US" ALL
      COMMAND ${LUPDATE_EXECUTABLE} ${CMAKE_SOURCE_DIR} -ts ${EN_US_FILENAME}
      DEPENDS ${EN_US_FILENAME}
    )
endif()



# Release :

if(ANDROID OR IOS)
else()
    set(LRELEASE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lrelease")
    if(NOT EXISTS "${LRELEASE_EXECUTABLE}")
        set(LRELEASE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lrelease-qt6")
    endif()
    if(NOT EXISTS "${LRELEASE_EXECUTABLE}")
        set(LRELEASE_EXECUTABLE "/usr/lib/qt6/bin/lrelease")
    endif()
    if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
        if(EXISTS "${CMAKE_PREFIX_PATH}/../../bin/lrelease.exe")
            set(LRELEASE_EXECUTABLE "${CMAKE_PREFIX_PATH}/../../bin/lrelease.exe")
        else()
            set(LRELEASE_EXECUTABLE "${CMAKE_PREFIX_PATH}/bin/lrelease.exe")
        endif()
    endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    message(${LRELEASE_EXECUTABLE})

    file(GLOB_RECURSE TS_FILES CONFIGURE_DEPENDS "${CMAKE_SOURCE_DIR}/src/translations/skribisto_*.ts")

    foreach(TS_FILEPATH ${TS_FILES})
          get_filename_component(TS_FILENAME ${TS_FILEPATH} NAME)
          string(REGEX MATCH "\\_([0-9a-zA-Z_]*)\\." LANGSTR ${TS_FILENAME})
          string(REPLACE "." ""  LANGSTR ${LANGSTR})
          set(QM_FILENAME "skribisto${LANGSTR}.qm")
          add_custom_target("release_ts${LANGSTR}" ALL
            COMMAND ${LRELEASE_EXECUTABLE} ${TS_FILEPATH} -silent -qm ${QM_FILENAME}
            DEPENDS ${TS_FILEPATH}
          )

          if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
              install(FILES "${CMAKE_CURRENT_BINARY_DIR}/${QM_FILENAME}" DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/skribisto/translations")
          endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

          if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
              install(FILES "${CMAKE_CURRENT_BINARY_DIR}/${QM_FILENAME}" DESTINATION "bin/translations")
          endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")

    endforeach(TS_FILEPATH)


    # make the ts files visible in the IDE
    add_custom_target(translations SOURCES ${TS_FILES})
endif()
