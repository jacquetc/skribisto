from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format
from pathlib import Path


def get_generation_dict(
    common_cmake_folder_path: str,
    application_name: str,
    feature_by_name: dict,
    entities_by_name: dict,
) -> dict:
    generation_dict = {}
    for feature_name, feature in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_spinal_name = stringcase.spinalcase(feature_name)
        feature_camel_name = stringcase.camelcase(feature_name)
        generation_dict[feature_pascal_name] = {
            "feature_snake_name": feature_snake_name,
            "feature_pascal_name": feature_pascal_name,
            "feature_spinal_name": feature_spinal_name,
            "feature_camel_name": feature_camel_name,
        }
        # add export_header
        export_header = f"application_{feature_snake_name}_export.h"
        generation_dict[feature_pascal_name]["export_header"] = export_header
        generation_dict[feature_pascal_name]["export_header_file"] = os.path.join(
            common_cmake_folder_path,
            feature_snake_name + "_feature",
            export_header,
        )
        generation_dict[feature_pascal_name][
            "export"
        ] = f"{stringcase.uppercase(application_name)}_APPLICATION_{stringcase.uppercase(feature_snake_name)}_EXPORT"

    # add CRUD handlers
    for feature_name, feature in generation_dict.items():
        feature_snake_name = feature["feature_snake_name"]
        feature_pascal_name = feature["feature_pascal_name"]
        feature["crud_handlers"] = {}
        crud_handlers = feature["crud_handlers"]
        crud_data = feature_by_name[feature_name].get("CRUD", {})
        entity_mappable_with = crud_data.get("entity_mappable_with", None)
        if crud_data.get("enabled", False) and not entity_mappable_with:
            raise Exception(
                f"CRUD.entity_mappable_with not defined for feature {feature_name}"
            )
        entity_mappable_with_snake = stringcase.snakecase(entity_mappable_with)
        entity_mappable_with_pascal = stringcase.pascalcase(entity_mappable_with)
        if crud_data.get("enabled", False):
            create_data = crud_data.get("create", {})
            if create_data.get("enabled", False) and create_data.get("generate", True):
                # find out if the owner field is a list
                owner_fields = entities_by_name.get(
                    entities_by_name[entity_mappable_with_pascal]
                    .get("owner", {})
                    .get("name", ""),
                    {},
                ).get("fields", [])
                for owner_field in owner_fields:
                    if owner_field.get("type", "").count("QList<") > 0:
                        owner_field_is_list = True
                        break
                    else:
                        owner_field_is_list = False

                # fill crud_handlers for create
                crud_handlers["create"] = {
                    "generate": True,
                    "templates": [
                        "create_handler.cpp.jinja2",
                        "create_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"create_{entity_mappable_with_snake}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"create_{entity_mappable_with_snake}_command_handler.h",
                        ),
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                    # unique to create : need to add the new entity to the owner entity
                    "has_owner": True
                    if entities_by_name[entity_mappable_with_pascal]
                    .get("owner", {})
                    .get("name", None)
                    else False,
                    "owner_name_snake": stringcase.snakecase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("name", None)
                    ),
                    "owner_name_pascal": stringcase.pascalcase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("name", None)
                    ),
                    "owner_name_camel": stringcase.camelcase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("name", None)
                    ),
                    "owner_field_snake": stringcase.snakecase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("field", None)
                    ),
                    "owner_field_pascal": stringcase.pascalcase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("field", None)
                    ),
                    "owner_field_camel": stringcase.camelcase(
                        entities_by_name[entity_mappable_with_pascal]
                        .get("owner", {})
                        .get("field", None)
                    ),
                    "owner_field_is_list": owner_field_is_list,
                }
            remove_data = crud_data.get("remove", {})
            if remove_data.get("enabled", False) and remove_data.get("generate", True):
                crud_handlers["remove"] = {
                    "generate": True,
                    "templates": [
                        "remove_handler.cpp.jinja2",
                        "remove_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"remove_{entity_mappable_with_snake}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"remove_{entity_mappable_with_snake}_command_handler.h",
                        ),
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                }
            update_data = crud_data.get("update", {})
            if update_data.get("enabled", False) and update_data.get("generate", True):
                crud_handlers["update"] = {
                    "generate": True,
                    "templates": [
                        "update_handler.cpp.jinja2",
                        "update_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"update_{entity_mappable_with_snake}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "commands",
                            f"update_{entity_mappable_with_snake}_command_handler.h",
                        ),
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                }
            get_data = crud_data.get("get", {})
            if get_data.get("enabled", False) and get_data.get("generate", True):
                crud_handlers["get"] = {
                    "generate": True,
                    "templates": [
                        "get_handler.cpp.jinja2",
                        "get_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_{entity_mappable_with_snake}_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_{entity_mappable_with_snake}_query_handler.h",
                        ),
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                }
            get_all_data = crud_data.get("get_all", {})
            if get_all_data.get("enabled", False) and get_all_data.get(
                "generate", True
            ):
                crud_handlers["get_all"] = {
                    "generate": True,
                    "templates": [
                        "get_all_handler.cpp.jinja2",
                        "get_all_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_all_{entity_mappable_with_snake}_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_all_{entity_mappable_with_snake}_query_handler.h",
                        ),
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                }
            get_with_details_data = crud_data.get("get_with_details", {})
            if get_with_details_data.get(
                "enabled", False
            ) and get_with_details_data.get("generate", True):
                crud_handlers["get_with_details"] = {
                    "generate": True,
                    "templates": [
                        "get_with_details_handler.cpp.jinja2",
                        "get_with_details_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_{entity_mappable_with_snake}_with_details_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name + "_feature",
                            feature_snake_name,
                            "queries",
                            f"get_{entity_mappable_with_snake}_with_details_query_handler.h",
                        ),
                    ],
                    "lazy_load_pascal_fields": [
                        stringcase.pascalcase(field)
                        for field in get_lazy_loading_fields(
                            entity_mappable_with_pascal, entities_by_name
                        )
                    ],
                    "entity_mappable_with_snake": entity_mappable_with_snake,
                    "entity_mappable_with_pascal": entity_mappable_with_pascal,
                    "entity_mappable_with_camel": stringcase.camelcase(
                        entity_mappable_with
                    ),
                }
        for handler_name, handler_data in crud_handlers.items():
            handler_data["export"] = feature["export"]
            handler_data["export_header"] = feature["export_header"]
            handler_data["export_header_file"] = feature["export_header_file"]

    # add commands and queries to the feature:
    for feature_name, feature in generation_dict.items():
        feature_snake_name = feature["feature_snake_name"]
        feature_pascal_name = feature["feature_pascal_name"]
        feature["custom_handlers"] = {}
        if feature_by_name[feature_name].get("commands", None):
            feature["custom_handlers"]["commands"] = feature_by_name[feature_name][
                "commands"
            ]

            for command in feature["custom_handlers"]["commands"]:
                files = [
                    os.path.join(
                        common_cmake_folder_path,
                        feature_snake_name + "_feature",
                        feature_snake_name,
                        "commands",
                        f"{stringcase.snakecase(command['name'])}_command_handler.h",
                    ),
                    os.path.join(
                        common_cmake_folder_path,
                        feature_snake_name + "_feature",
                        feature_snake_name,
                        "commands",
                        f"{stringcase.snakecase(command['name'])}_command_handler.cpp",
                    ),
                ]
                command["files"] = files
                command["templates"] = [
                    "custom_command_handler.h.jinja2",
                    "custom_command_handler.cpp.jinja2",
                ]
                command["repositories"] = []
                for entity_name in command.get("entities", []):
                    command["repositories"].append(
                        {
                            "pascal_name": stringcase.pascalcase(entity_name),
                            "snake_name": stringcase.snakecase(entity_name),
                            "camel_name": stringcase.camelcase(entity_name),
                        }
                    )

                command["export"] = feature["export"]
                command["export_header"] = feature["export_header"]
                command["export_header_file"] = feature["export_header_file"]
                command["camel_name"] = stringcase.camelcase(command["name"])
                command["snake_name"] = stringcase.snakecase(command["name"])
                command["pascal_name"] = stringcase.pascalcase(command["name"])
                # DTO in
                if command.get("dto", {}).get("in", {}):
                    if command.get("dto", {}).get("in", {}).get("is_void", False):
                        command["dto_in_pascal_name"] = "void"
                        command["dto_in_camel_name"] = "void"
                        command["dto_in_snake_name"] = "void"
                    else:
                        command["dto_in_pascal_name"] = stringcase.pascalcase(
                            command["dto"]["in"]["type_prefix"] + "DTO"
                        )
                        command["dto_in_camel_name"] = stringcase.camelcase(
                            command["dto"]["in"]["type_prefix"] + "DTO"
                        )
                        command["dto_in_snake_name"] = (
                            stringcase.snakecase(command["dto"]["in"]["type_prefix"])
                            + "_dto"
                        )
                # DTO out
                if command.get("dto", {}).get("out", {}):
                    if command.get("dto", {}).get("out", {}).get("is_void", False):
                        command["dto_out_pascal_name"] = "void"
                        command["dto_out_camel_name"] = "void"
                        command["dto_out_snake_name"] = "void"
                    else:
                        command["dto_out_pascal_name"] = stringcase.pascalcase(
                            command["dto"]["out"]["type_prefix"] + "DTO"
                        )
                        command["dto_out_camel_name"] = stringcase.camelcase(
                            command["dto"]["out"]["type_prefix"] + "DTO"
                        )
                        command["dto_out_snake_name"] = (
                            stringcase.snakecase(command["dto"]["out"]["type_prefix"])
                            + "_dto"
                        )

                # for each command.dto.out.type_prefix, add command.dto.out.type_prefix_camel
                for _, dto_data in command.get("dto", {}).items():
                    if dto_data.get("type_prefix", None):
                        dto_data["type_prefix_camel"] = stringcase.camelcase(
                            dto_data["type_prefix"]
                        )
                        dto_data["type_prefix_snake"] = stringcase.snakecase(
                            dto_data["type_prefix"]
                        )
                        dto_data["type_prefix_pascal"] = stringcase.pascalcase(
                            dto_data["type_prefix"]
                        )

        if feature_by_name[feature_name].get("queries", None):
            feature["custom_handlers"]["queries"] = feature_by_name[feature_name][
                "queries"
            ]

            for query in feature["custom_handlers"]["queries"]:
                files = [
                    os.path.join(
                        common_cmake_folder_path,
                        feature_snake_name + "_feature",
                        feature_snake_name,
                        "queries",
                        f"{stringcase.snakecase(query['name'])}_query_handler.h",
                    ),
                    os.path.join(
                        common_cmake_folder_path,
                        feature_snake_name + "_feature",
                        feature_snake_name,
                        "queries",
                        f"{stringcase.snakecase(query['name'])}_query_handler.cpp",
                    ),
                ]
                query["files"] = files
                query["templates"] = [
                    "custom_query_handler.h.jinja2",
                    "custom_query_handler.cpp.jinja2",
                ]

                query["repositories"] = []
                for entity_name in query.get("entities", []):
                    query["repositories"].append(
                        {
                            "pascal_name": stringcase.pascalcase(entity_name),
                            "snake_name": stringcase.snakecase(entity_name),
                            "camel_name": stringcase.camelcase(entity_name),
                        }
                    )
                query["export"] = feature["export"]
                query["export_header"] = feature["export_header"]
                query["export_header_file"] = feature["export_header_file"]
                query["camel_name"] = stringcase.camelcase(query["name"])
                query["snake_name"] = stringcase.snakecase(query["name"])
                query["pascal_name"] = stringcase.pascalcase(query["name"])
                # DTO in
                if query.get("dto", {}).get("in", {}):
                    if query.get("dto", {}).get("in", {}).get("is_void", False):
                        query["dto_in_pascal_name"] = "void"
                        query["dto_in_camel_name"] = "void"
                        query["dto_in_snake_name"] = "void"

                    else:
                        query["dto_in_pascal_name"] = stringcase.pascalcase(
                            query["dto"]["in"]["type_prefix"] + "DTO"
                        )
                        query["dto_in_camel_name"] = stringcase.camelcase(
                            query["dto"]["in"]["type_prefix"] + "DTO"
                        )
                        query["dto_in_snake_name"] = (
                            stringcase.snakecase(query["dto"]["in"]["type_prefix"])
                            + "_dto"
                        )
                # DTO out
                if query.get("dto", {}).get("out", {}):
                    if query.get("dto", {}).get("out", {}).get("is_void", False):
                        query["dto_out_pascal_name"] = "void"
                        query["dto_out_camel_name"] = "void"
                        query["dto_out_snake_name"] = "void"
                    else:
                        query["dto_out_pascal_name"] = stringcase.pascalcase(
                            query["dto"]["out"]["type_prefix"] + "DTO"
                        )
                        query["dto_out_camel_name"] = stringcase.camelcase(
                            query["dto"]["out"]["type_prefix"] + "DTO"
                        )
                        query["dto_out_snake_name"] = (
                            stringcase.snakecase(query["dto"]["out"]["type_prefix"])
                            + "_dto"
                        )

                # for each query.dto.out.type_prefix, add query.dto.out.type_prefix_camel
                for _, dto_data in query.get("dto", {}).items():
                    if dto_data.get("type_prefix", None):
                        dto_data["type_prefix_camel"] = stringcase.camelcase(
                            dto_data["type_prefix"]
                        )
                        dto_data["type_prefix_snake"] = stringcase.snakecase(
                            dto_data["type_prefix"]
                        )
                        dto_data["type_prefix_pascal"] = stringcase.pascalcase(
                            dto_data["type_prefix"]
                        )

        # add commands and queries files to feature custom handlers:
        feature["custom_handlers"]["files"] = []
        if feature["custom_handlers"].get("commands", None):
            for command in feature["custom_handlers"]["commands"]:
                feature["custom_handlers"]["files"] += command["files"]
        if feature["custom_handlers"].get("queries", None):
            for query in feature["custom_handlers"]["queries"]:
                feature["custom_handlers"]["files"] += query["files"]

    # add files to the feature:
    for feature_name, feature in generation_dict.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature["handler_files"] = []
        for crud_handler_name, crud_handler in feature["crud_handlers"].items():
            feature["handler_files"] += crud_handler["files"]

        # add custom handlers files to the feature:
        if feature.get("custom_handlers", None):
            custom_handler = feature["custom_handlers"]
            feature["handler_files"] += custom_handler["files"]

        # add CMakelists.txt file name to the feature:
        feature["cmakelists_file"] = os.path.join(
            common_cmake_folder_path,
            feature_snake_name + "_feature",
            "CMakeLists.txt",
        )

    return generation_dict


def get_lazy_loading_fields(entity_mappable_with: str, entities_by_name: dict) -> list:
    def is_unique_foreign_entity(field_type: str) -> bool:
        for entity_name, entity in entities_by_name.items():
            name = entity["name"]
            if name == field_type:
                return True

        return False

    def is_list_foreign_entity(field_type: str) -> bool:
        if "<" not in field_type:
            return False

        type = field_type.split("<")[1].split(">")[0].strip()

        for entity_name, entity in entities_by_name.items():
            name = entity["name"]
            if name == type:
                return True

        return False

    entity_mappable_with_pascal = stringcase.pascalcase(entity_mappable_with)

    # create a list of foreign entities
    lazy_loading_fields = []
    for field in entities_by_name[entity_mappable_with_pascal]["fields"]:
        field_type = field["type"]
        field_name = field["name"]
        if is_unique_foreign_entity(field_type):
            lazy_loading_fields += [stringcase.pascalcase(field_name)]
        if is_list_foreign_entity(field_type):
            lazy_loading_fields += [stringcase.pascalcase(field_name)]

    return lazy_loading_fields


def generate_common_cmakelists(
    feature_by_name: dict,
    common_cmake_folder_path: str,
    files_to_be_generated: dict[str, bool],
):
    # generate common cmakelists.txt

    common_cmakelists_file = os.path.join(common_cmake_folder_path, "CMakeLists.txt")
    if files_to_be_generated.get(common_cmakelists_file, False):
        ## After the loop, write the list of features folders to the common cmakelists.txt
        with open(common_cmakelists_file, "w") as fh:
            for feature_name, _ in feature_by_name.items():
                fh.write(
                    f"add_subdirectory({stringcase.snakecase(feature_name)}_feature)"
                    + "\n"
                )
            print(f"Successfully wrote file {common_cmakelists_file}")


def generate_feature_export_file(
    feature: dict, application_name: str, files_to_be_generated: dict[str, bool]
):
    template_env = Environment(loader=FileSystemLoader("templates/application"))
    dto_cmakelists_template = template_env.get_template("export_template.jinja2")

    export_header_file = feature["export_header_file"]
    export = feature["export"]

    if not files_to_be_generated.get(export_header_file, False):
        return

    application_uppercase_name = stringcase.uppercase(application_name)

    feature_snake_name = feature["feature_snake_name"]
    feature_uppercase_name = stringcase.uppercase(feature_snake_name)

    rendered_template = dto_cmakelists_template.render(
        application_uppercase_name=application_uppercase_name,
        feature_uppercase_name=feature_uppercase_name,
        export=export,
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(export_header_file), exist_ok=True)

    with open(export_header_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {export_header_file}")


def generate_handler_cmakelists(
    feature: dict, application_name: str, files_to_be_generated: dict[str, bool]
):
    # generate these DTO's cmakelists.txt

    template_env = Environment(loader=FileSystemLoader("templates/application"))
    dto_cmakelists_template = template_env.get_template("cmakelists_template.jinja2")

    dto_cmakelists_file = feature["cmakelists_file"]

    if not files_to_be_generated.get(dto_cmakelists_file, False):
        return

    files = feature["handler_files"] + [feature["export_header_file"]]

    ## Convert the file path to be relative to the directory of the cmakelists
    relative_generated_files = []
    for file_path in files:
        relative_generated_file = os.path.relpath(
            file_path, os.path.dirname(dto_cmakelists_file)
        )
        relative_generated_files.append(relative_generated_file)

    feature_snake_name = feature["feature_snake_name"]
    feature_spinal_name = stringcase.spinalcase(feature_snake_name)

    rendered_template = dto_cmakelists_template.render(
        feature_snake_name=feature_snake_name,
        feature_spinal_name=feature_spinal_name,
        feature_uppercase_name=stringcase.uppercase(feature_snake_name),
        files=relative_generated_files,
        application_name=application_name,
        application_uppercase_name=stringcase.uppercase(application_name),
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(dto_cmakelists_file), exist_ok=True)

    with open(dto_cmakelists_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {dto_cmakelists_file}")


def generate_crud_handler(
    handler: dict,
    feature: dict,
    files_to_be_generated: dict[str, bool],
    uncrustify_config_file: str,
):
    for file_path in handler["files"]:
        if not files_to_be_generated.get(file_path, False):
            continue

        if not handler.get("generate", True):
            continue

        template_env = Environment(
            loader=FileSystemLoader("templates/application/CRUD")
        )
        template = template_env.get_template(
            handler["templates"][handler["files"].index(file_path)]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(file_path), exist_ok=True)

        with open(file_path, "w") as f:
            f.write(
                template.render(
                    feature_pascal_name=feature["feature_pascal_name"],
                    feature_snake_name=feature["feature_snake_name"],
                    export=handler["export"],
                    export_header=handler["export_header"],
                    entity_mappable_with_pascal=handler["entity_mappable_with_pascal"],
                    entity_mappable_with_snake=handler["entity_mappable_with_snake"],
                    entity_mappable_with_camel=handler["entity_mappable_with_camel"],
                    lazy_load_pascal_fields=handler.get("lazy_load_pascal_fields", []),
                    has_owner=handler.get("has_owner", False),
                    owner_name_snake=handler.get("owner_name_snake", ""),
                    owner_name_pascal=handler.get("owner_name_pascal", ""),
                    owner_name_camel=handler.get("owner_name_camel", ""),
                    owner_field_snake=handler.get("owner_field_snake", ""),
                    owner_field_pascal=handler.get("owner_field_pascal", ""),
                    owner_field_camel=handler.get("owner_field_camel", ""),
                )
            )
            print(f"Successfully wrote file {file_path}")

        # if uncrustify_config_file:
        #     uncrustify.run_uncrustify(file_path, uncrustify_config_file)
        clang_format.run_clang_format(file_path)


def generate_custom_command_handler(
    handler: dict,
    feature: dict,
    files_to_be_generated: dict[str, bool],
    uncrustify_config_file: str,
):
    for file_path in handler["files"]:
        if not files_to_be_generated.get(file_path, False):
            continue

        template_env = Environment(
            loader=FileSystemLoader("templates/application/custom")
        )
        template = template_env.get_template(
            handler["templates"][handler["files"].index(file_path)]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(file_path), exist_ok=True)

        with open(file_path, "w") as f:
            f.write(
                template.render(
                    feature_pascal_name=feature["feature_pascal_name"],
                    feature_snake_name=feature["feature_snake_name"],
                    command=handler,
                    export=handler["export"],
                    export_header=handler["export_header"],
                    validator_enabled=handler.get("validator", {}).get(
                        "enabled", False
                    ),
                )
            )
            print(f"Successfully wrote file {file_path}")

        # if uncrustify_config_file:
        #     uncrustify.run_uncrustify(file_path, uncrustify_config_file)
        clang_format.run_clang_format(file_path)


def generate_custom_query_handler(
    handler: dict,
    feature: dict,
    files_to_be_generated: dict[str, bool],
    uncrustify_config_file: str,
):
    for file_path in handler["files"]:
        if not files_to_be_generated.get(file_path, False):
            continue

        template_env = Environment(
            loader=FileSystemLoader("templates/application/custom")
        )
        template = template_env.get_template(
            handler["templates"][handler["files"].index(file_path)]
        )

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(file_path), exist_ok=True)

        with open(file_path, "w") as f:
            f.write(
                template.render(
                    feature_pascal_name=feature["feature_pascal_name"],
                    feature_snake_name=feature["feature_snake_name"],
                    feature_camel_name=feature["feature_camel_name"],
                    query=handler,
                    export=handler["export"],
                    export_header=handler["export_header"],
                    validator_enabled=handler.get("validator", {}).get(
                        "enabled", False
                    ),
                )
            )
            print(f"Successfully wrote file {file_path}")

        # if uncrustify_config_file:
        #     uncrustify.run_uncrustify(file_path, uncrustify_config_file)
        clang_format.run_clang_format(file_path)


def generate_application_files(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = {},
    uncrustify_config_file: str = "",
):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])
    common_cmake_folder_path = application_data.get("common_cmake_folder_path", "")

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list if entity.get("generate", True)]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    for _, feature in get_generation_dict(
        common_cmake_folder_path, application_name, feature_by_name, entities_by_name
    ).items():
        # generate crud handlers
        for handler in feature["crud_handlers"].values():
            generate_crud_handler(
                handler, feature, files_to_be_generated, uncrustify_config_file
            )

        # generate custom handlers
        if feature.get("custom_handlers", None):
            if feature["custom_handlers"].get("commands", None):
                for command_handler in feature["custom_handlers"]["commands"]:
                    generate_custom_command_handler(
                        command_handler,
                        feature,
                        files_to_be_generated,
                        uncrustify_config_file,
                    )
            if feature["custom_handlers"].get("queries", None):
                for query_handler in feature["custom_handlers"]["queries"]:
                    generate_custom_query_handler(
                        query_handler,
                        feature,
                        files_to_be_generated,
                        uncrustify_config_file,
                    )

        # generate handler cmakelists.txt
        generate_handler_cmakelists(feature, application_name, files_to_be_generated)

        generate_feature_export_file(feature, application_name, files_to_be_generated)

    # generate common cmakelists.txt
    generate_common_cmakelists(
        feature_by_name, common_cmake_folder_path, files_to_be_generated
    )


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])
    common_cmake_folder_path = application_data.get("common_cmake_folder_path", "")

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list if entity.get("generate", True)]

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    files = []
    for _, feature in get_generation_dict(
        common_cmake_folder_path, application_name, feature_by_name, entities_by_name
    ).items():
        files += feature["handler_files"]

        common_cmake_file = feature["cmakelists_file"]
        files.append(common_cmake_file)
        export_header_file = feature["export_header_file"]
        files.append(export_header_file)

    # # add CMakelists.txt:
    common_cmake_file = os.path.join(common_cmake_folder_path, "CMakeLists.txt")
    files.append(common_cmake_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_application_files(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = {},
    uncrustify_config_file: str = "",
):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["application"]["common_cmake_folder_path"] = "preview/" + manifest[
        "application"
    ]["common_cmake_folder_path"].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    # preprend preview/ to the file names in the dict files_to_be_generated and remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated["preview/" + path.replace("..", "")] = value

        generate_application_files(
            manifest_preview_file, preview_files_to_be_generated, uncrustify_config_file
        )

    else:
        generate_application_files(manifest_preview_file, {}, uncrustify_config_file)


# Main execution
if __name__ == "__main__":
    full_path = Path(__file__).resolve().parent

    # add the current directory to the path so that we can import the generated files
    sys.path.append(full_path)

    # set the current directory to the generator directory
    os.chdir(full_path)

    file = "manifest.yaml"
    # if argument is --preview, generate the files into the preview folder
    if len(sys.argv) > 1 and sys.argv[1] == "--preview":
        preview_application_files(file)
    else:
        # generate the files
        generate_application_files(file)
