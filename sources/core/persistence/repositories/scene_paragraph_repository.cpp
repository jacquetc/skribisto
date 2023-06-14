#include "scene_paragraph_repository.h"

using namespace Repository;

SceneParagraphRepository::SceneParagraphRepository(
    InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::SceneParagraph>(sceneParagraphDatabase)
{}
