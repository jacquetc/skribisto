if (CMAKE_BUILD_TYPE STREQUAL "Debug")
#add_subdirectory(testpage)
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

add_subdirectory(textPage)
add_subdirectory(folderPage)
add_subdirectory(sectionPage)
add_subdirectory(projectPage)
add_subdirectory(overviewPage)
add_subdirectory(cardViewPage)
add_subdirectory(themePage)
add_subdirectory(writingGamesPageToolbox)
add_subdirectory(favoritesProjectToolbox)
add_subdirectory(navigationProjectToolbox)
add_subdirectory(plumeCreatorImporter)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    add_subdirectory(antidotePageToolbox)
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")


#add_subdirectory(welcomewindow)
#add_subdirectory(writewindow)

