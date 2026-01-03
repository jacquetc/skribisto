### Skribisto Architecture Overview

This document explains the pragmatic architectural style used in Skribisto, how the codebase is organized, how modules
depend on each other, and the main patterns guiding composition and data flow. It reflects the current, intentionally
simplified design used in the repository.

### Principles (Pragmatic, Qt-friendly)

- "Common means common": the `src/common` module contains shared pieces used across features and layers, including
  domain entities, repositories, DB utilities, events, a service locator, and undo/redo. This is a deliberate choice (
  not strict Clean Architecture separation by module).
- Layers are mostly inside modules, not enforced between modules. We use SOLID when it improves testability or clarity,
  and avoid unnecessary indirection.
- Qt/QML friendliness: we keep code and patterns that integrate well with Qt types (QObject, QPointer, QML type
  exposure) and avoid over-abstracting around them.
- Factories return non-shared objects: repositories are created via a factory and returned as `std::unique_ptr` to avoid
  unintended sharing across threads.

### Module Map

- `src/common` (library: `skribisto-common`)
    - Entities/value objects under `entities/` (work, binder, content, root, recent_work, binder_item, binder_tag,
      dict_word).
    - Database utilities under `database/` (DbContext, DbSubContext, table cache, junction-table helpers).
    - Direct access feature code under `direct_access/`:
        - Repository headers/implementations per feature (root, work, binder, binder_item, binder_tag, content,
          recent_work).
        - Event types per feature and an `EventRegistry` (QObject-based, accessed via QPointer specializations).
        - `RepositoryFactory` with free functions that build fresh repository instances bound to a `DbSubContext` and
          event registry; returns `std::unique_ptr`.
    - Feature-level events under `features/` (e.g., `work_management_events`).
    - Undo/redo infrastructure under `undo_redo/`.
    - Service location: `ServiceLocator` holds shared singletons or app-composed services for use at runtime (DbContext,
      EventRegistry, UndoRedo system, FeatureEventRegistry).

- `src/direct_access` (library: `skribisto-direct-access`)
    - Aggregates per-feature submodules through `file_list.cmake` includes and links to `skribisto-common`.
    - QML bridge(s) and feature code that complements or wraps direct-access functionality.

- `src/work_management` (library: `skribisto-work-management`)
    - Cross-feature orchestration: use cases and units of work that coordinate multiple feature repositories and events.
    - Key parts:
        - `WorkManagementController`: resolves dependencies from `ServiceLocator`, constructs Units of Work (UoWs) and
          use cases, and executes them via the Undo/Redo system with QCoro.
        - UoWs (e.g., `load_work_uow`, `save_work_uow`): own a `DbSubContext` (scoped connection/transaction) and create
          fresh repositories through `RepositoryFactory` for each operation.
        - Use cases (e.g., `LoadWorkUseCase`, `SaveWorkUseCase`) encapsulate business steps built on the UoWs.

- `src/qml_app` (executable: `Skribisto`)
    - Composition root and QML UI. Loads QML modules, wires the `ServiceLocator` at startup, and optionally exposes
      selected C++ types to QML.
    - QML modules under `src/qml_app/content` and `src/qml_app/real_imports/...` provide UI-side packaging. Some
      modules (e.g., `Skr.Singles`) use `QML_FOREIGN` to expose types defined elsewhere for prototyping.

### Dependency Direction (current, by intent)

- App/QML (`Skribisto`) depends on `skribisto-common`, `skribisto-direct-access`, and UI/QML libs.
- `skribisto-work-management` depends on `skribisto-common` for entities, DB utilities, repositories, events, and
  undo/redo.
- `skribisto-direct-access` depends on `skribisto-common` and aggregates feature code; it also provides QML-facing
  adapters.
- There is no strict “domain-only” module; the project favors simplicity by co-locating domain entities and data access
  in `common`.

This is a practical, non-purist arrangement. It keeps builds and code navigation straightforward while allowing UoWs to
operate with clear ownership and transaction scope.

### Composition and Service Location

- The QML app acts as the composition root. It constructs and injects into the `ServiceLocator`:
    - A process-wide `DbContext` that creates per-operation `DbSubContext`s.
    - The `EventRegistry` instance (QObject-based), exposed via `QPointer` from the locator and passed to repos/UoWs.
    - The Undo/Redo system (`UndoRedoSystem`) used for asynchronous command execution.
    - Optional feature event registries.
- `WorkManagementController` has a parameterless constructor, calls `resolveDependencies()`, and retrieves these
  components from the `ServiceLocator` at runtime. This suits QML construction and avoids heavy DI boilerplate.

### Units of Work and Transactions

- Each UoW creates and owns a `DbSubContext` from the `DbContext`. This:
    - Scopes the database connection and transaction to the UoW lifetime.
    - Provides a clear surface for `beginTransaction/commit/rollback` and savepoints.
- Repositories are created on demand through `RepositoryFactory` with the UoW’s `DbSubContext` and `EventRegistry`.
    - Return type is `std::unique_ptr<...Repository>` (non-shared ownership), preventing accidental sharing across
      threads.
- Example flow (Load Work):
    1) Controller clears Undo/Redo scopes.
    2) Controller constructs `LoadWorkUnitOfWork(DbContext&, EventRegistry, FeatureEventRegistry)`.
    3) UoW opens a `DbSubContext`, begins transactions as needed.
    4) UoW creates repositories via `RepositoryFactory::create*Repository(m_dbSubContext, m_eventRegistry)` and performs
       operations.
    5) UoW publishes feature-level events using `FeatureEventRegistry`.

### Undo/Redo and QCoro

- Use cases are executed within `UndoRedoCommand`s and scheduled through the `UndoRedoSystem` which supports async
  execution with QCoro (`QCoro::Task`).
- The controller builds use cases, wraps them in commands, and awaits `executeCommandAsync` with a timeout.

### QML Integration

- The app provides QML modules and resources; some QML plugins expose foreign C++ types using `QML_FOREIGN` (e.g.,
  `Skr.Singles::SingleBinderItem`). This is acceptable for the current PoC. As the app grows, prefer exposing
  DTOs/view-models and confining DB access to UoWs/controllers.
- `SKR_BUILD_WITH_MOCKS` allows building the app with mock QML imports while skipping core backends. This is useful for
  UI prototyping and faster iteration.

### Threading and Ownership Guidelines

- Repositories: created per UoW, non-shared (`unique_ptr`), non-copyable.
- Db contexts: `DbContext` is app-owned; each UoW owns its `DbSubContext`. No cross-thread sharing of repository
  instances.
- EventRegistry: QObject-based, held via `QPointer` to avoid dangling pointers; lifetime owned by the app/composition
  root.

### Testing Guidance (within this pragmatic setup)

- Unit-test UoWs and repositories using a temporary SQLite database (file or `:memory:`):
    - Create a `DbContext` pointed at a temp DB path.
    - Build a UoW with a `DbSubContext` and a real `EventRegistry`.
    - Use `RepositoryFactory` to exercise repo behavior (create/update/relations) and UoW logic.
- Because the project intentionally avoids additional indirection layers, tests typically use real SQLite and events
  rather than mocks.

### Build System Notes

- CMake minimum 3.21. Qt 6.4+. QCoro enabled through `qcoro_enable_coroutines()` in relevant targets.
- Targets of interest:
    - Libraries: `skribisto-common`, `skribisto-direct-access`, `skribisto-work-management`, QML plugin targets.
    - Executable: `Skribisto` (the QML app).
    - Optional test targets can be enabled via `SKR_BUILD_TESTS`.
- Linux install rules and QML lint/type-registration helper targets are present.

### When to Introduce More Abstraction (only if it pays off)

- If a need arises to test `work_management` without SQLite, or to support a second storage backend, small
  interfaces/factories can be introduced at the seam where UoWs acquire repositories or where DB export features live.
  Until then, keep the current straightforward approach.

### Data Flow Example (Save Work)

1) `WorkManagementController::saveWork(dto)` clears undo/redo scopes.
2) Constructs `SaveWorkUnitOfWork(DbContext&, EventRegistry)`.
3) UoW opens a `DbSubContext`, optionally starts a transaction.
4) Creates repositories (`WorkRepository`, `BinderRepository`, etc.) via `RepositoryFactory`, performs changes.
5) For exporting a database file, UoW performs a WAL checkpoint and copies the database to the chosen path.
6) Controller wraps the use case in an `UndoRedoCommand` and awaits execution via QCoro.

### Database and Persistence Model

The application uses an ephemeral SQLite database in `/tmp/` as working memory, completely decoupled from user file I/O.
`DbBuilder` creates the schema at startup. Load and Save are use cases that transform between file formats and the
internal database. Crash recovery detects orphaned databases and offers to restore work, tracking the original file
path in metadata.

Relationships are managed through junction tables with four semantic types: `OneToOne`, `OrderedOneToMany`,
`UnorderedOneToMany`, and `UnorderedManyToMany`. Each provides batch operations, caching, and proper invalidation.
Table and junction caches are thread-safe, time-expiring (30 minutes), and invalidated at write time within the Table
layer.

### Summary

Skribisto uses a pragmatic, Qt-friendly architectural variant that prioritizes clarity and productivity:

- Shared/common code is centralized in `src/common` (entities, repos, DB, events, undo/redo, service locator).
- Cross-feature orchestration lives in `src/work_management` and composes UoWs and use cases around a scoped
  `DbSubContext` and non-shared repositories.
- The QML app is the composition root and wires long-lived services into a `ServiceLocator`.
- Repositories are created via a factory and returned as `std::unique_ptr` for thread-safety and ownership clarity.

This approach intentionally avoids over-engineering while preserving clear ownership, good Qt integration, and a path
for future extraction if requirements evolve.