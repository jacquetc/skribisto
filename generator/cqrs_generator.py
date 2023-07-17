from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import uncrustify
import clang_format
from pathlib import Path


def get_cqrs_dict(
    cqrs_common_cmake_folder_path, feature_by_name, entities_by_name
) -> dict:
    # example of wanted dict
    # generation_dict = {
    #     "features": {
    #         "MyFeature": {
    #             "queries": {
    #                 "MyQuery": {
    #                     "file": os.path.join(
    #                         cqrs_common_cmake_folder_path,
    #                         "MyFeature",
    #                         "queries",
    #                         "my_query.h",
    #                     ),
    #                     "dto_pascal": "MyQueryDTO",
    #                     "dto_snake": "my_query_dto",
    #                 },
    #             },
    #             "commands": {
    #                 "MyCommand": {
    #                     "file": os.path.join(
    #                         cqrs_common_cmake_folder_path,
    #                         "MyFeature",
    #                         "commands",
    #                         "my_command.h",
    #                     ),
    #                     "dto_pascal": "MyCommandDTO",
    #                     "dto_snake": "my_command_dto",
    #                 },
    #             },
    #             "validators": {
    #                 "MyValidator": {
    #                     "file": os.path.join(
    #                         cqrs_common_cmake_folder_path,
    #                         "MyFeature",
    #                         "validators",
    #                         "my_validator.h",
    #                     ),
    #                     "repository_entity_pascal": "MyEntity",
    #                     "repository_entity_snake": "my_entity",
    #                     "repository_entity_camel": "myEntity",
    #                     "check_if_exists": True,
    #                 }
    #             },
    #             "cmake_folder_path": os.path.join(cqrs_common_cmake_folder_path, "MyFeature"),
    #         },
    #     },
    #     "common_cmake_folder_path": cqrs_common_cmake_folder_path,
    # }

    generation_dict = {
        "features": {},
        "common_cmake_folder_path": cqrs_common_cmake_folder_path,
    }

    for feature_name, feature in feature_by_name.items():
        feature_dict = {
            "queries": {},
            "commands": {},
            "validators": {},
            "cmake_folder_path": os.path.join(
                cqrs_common_cmake_folder_path,
                stringcase.snakecase(feature_name) + "_cqrs",
            ),
        }
        generation_dict["features"][feature_name] = feature_dict

        feature_name_snake = stringcase.snakecase(feature_name)
        generation_dict["features"][feature_name][
            "feature_name_snake"
        ] = feature_name_snake
        feature_name_pascal = stringcase.pascalcase(feature_name)
        generation_dict["features"][feature_name][
            "feature_name_pascal"
        ] = feature_name_pascal
        generation_dict["features"][feature_name][
            "feature_name_camel"
        ] = stringcase.camelcase(feature_name)
        generation_dict["features"][feature_name][
            "feature_name_spinal"
        ] = stringcase.spinalcase(feature_name)

        # add CRUD queries & commands:
        if feature.get("CRUD", False) and feature.get("CRUD", {}).get("enabled", False):
            entity_mappable_with = feature.get("CRUD", {}).get(
                "entity_mappable_with", feature_name_pascal
            )
            entity_mappable_with_snake = stringcase.snakecase(entity_mappable_with)
            entity_mappable_with_pascal = stringcase.pascalcase(entity_mappable_with)
            # get and get_with_details
            if feature.get("CRUD", {}).get("get", {}).get("enabled", False):
                generation_dict["features"][feature_name]["queries"][
                    "Get" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "queries",
                        "get_" + entity_mappable_with_snake + "_query.h",
                    ),
                    "name": "Get" + entity_mappable_with_pascal + "Query",
                    "dto_pascal": f"{entity_mappable_with}DTO",
                    "dto_snake": entity_mappable_with_snake + "_dto",
                    "request_is_id": True,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

            # add CRUD commands:
            # create
            if feature.get("CRUD", {}).get("create", {}).get("enabled", False):
                generation_dict["features"][feature_name]["commands"][
                    "Create" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "commands",
                        "create_" + entity_mappable_with_snake + "_command.h",
                    ),
                    "name": "Create" + entity_mappable_with_pascal + "Command",
                    "dto_pascal": "Create" + entity_mappable_with + "DTO",
                    "dto_snake": "create_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": False,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

                # add validator
                generation_dict["features"][feature_name]["validators"][
                    "Create" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "validators",
                        "create_" + entity_mappable_with_snake + "_command_validator.h",
                    ),
                    "name": "Create" + entity_mappable_with_pascal + "CommandValidator",
                    "dto_pascal": "Create" + entity_mappable_with + "DTO",
                    "dto_snake": "create_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": False,
                    "check_if_exists": False,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                    "repositories": [
                        {
                            "pascal_name": entity_mappable_with,
                            "snake_name": entity_mappable_with_snake,
                            "camel_name": stringcase.camelcase(entity_mappable_with),
                        }
                    ],
                }

            # update
            if feature.get("CRUD", {}).get("update", {}).get("enabled", False):
                generation_dict["features"][feature_name]["commands"][
                    "Update" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "commands",
                        "update_" + entity_mappable_with_snake + "_command.h",
                    ),
                    "name": "Update" + entity_mappable_with_pascal + "Command",
                    "dto_pascal": "Update" + entity_mappable_with + "DTO",
                    "dto_snake": "update_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": False,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

                # add validator
                generation_dict["features"][feature_name]["validators"][
                    "Update" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "validators",
                        "update_" + entity_mappable_with_snake + "_command_validator.h",
                    ),
                    "name": "Update" + entity_mappable_with_pascal + "CommandValidator",
                    "dto_pascal": "Update" + entity_mappable_with + "DTO",
                    "dto_snake": "update_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": False,
                    "check_if_exists": True,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                    "repositories": [
                        {
                            "pascal_name": entity_mappable_with,
                            "snake_name": entity_mappable_with_snake,
                            "camel_name": stringcase.camelcase(entity_mappable_with),
                        }
                    ],
                }

            # remove
            if feature.get("CRUD", {}).get("remove", {}).get("enabled", False):
                generation_dict["features"][feature_name]["commands"][
                    "Remove" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "commands",
                        "remove_" + entity_mappable_with_snake + "_command.h",
                    ),
                    "name": "Remove" + entity_mappable_with_pascal + "Command",
                    "dto_pascal": "Remove" + entity_mappable_with + "DTO",
                    "dto_snake": "remove_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": True,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

                # add validator
                generation_dict["features"][feature_name]["validators"][
                    "Remove" + entity_mappable_with_pascal
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "validators",
                        "remove_" + entity_mappable_with_snake + "_command_validator.h",
                    ),
                    "name": "Remove" + entity_mappable_with + "CommandValidator",
                    "dto_pascal": "Remove" + entity_mappable_with + "DTO",
                    "dto_snake": "remove_" + entity_mappable_with_snake + "_dto",
                    "request_is_id": True,
                    "check_if_exists": True,
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                    "repositories": [
                        {
                            "pascal_name": entity_mappable_with,
                            "snake_name": entity_mappable_with_snake,
                            "camel_name": stringcase.camelcase(entity_mappable_with),
                        }
                    ],
                }

        # add custom queries & commands
        custom_queries = feature.get("queries", [])
        for custom_query in custom_queries:
            custom_query_name = custom_query.get("name", "")
            custom_query_snake_name = stringcase.snakecase(custom_query_name)
            custom_query_pascal_name = stringcase.pascalcase(custom_query_name)
            dto_in_type_prefix = (
                custom_query.get("dto", {}).get("in", {}).get("type_prefix", "")
            )
            if custom_query_name:
                generation_dict["features"][feature_name]["queries"][
                    custom_query_name
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "queries",
                        custom_query_snake_name + "_query.h",
                    ),
                    "name": custom_query_pascal_name + "Query",
                    "dto_pascal": stringcase.pascalcase(dto_in_type_prefix + "DTO"),
                    "dto_snake": stringcase.snakecase(dto_in_type_prefix + "_dto"),
                    "request_is_id": custom_query.get("request_is_id", False),
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

            # if validator is enabled
            if custom_query.get("validator", {}).get("enabled", False):
                custom_queries_entities = custom_query.get("entities", [])
                repositories = [
                    {
                        "camel_name": stringcase.camelcase(x),
                        "pascal_name": stringcase.pascalcase(x),
                        "snake_name": stringcase.snakecase(x),
                    }
                    for x in custom_queries_entities
                ]
                generation_dict["features"][feature_name]["validators"][
                    custom_query_name
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "validators",
                        custom_query_snake_name + "_query_validator.h",
                    ),
                    "name": custom_query_pascal_name + "QueryValidator",
                    "dto_pascal": stringcase.pascalcase(dto_in_type_prefix + "DTO"),
                    "dto_snake": stringcase.snakecase(dto_in_type_prefix + "_dto"),
                    "request_is_id": custom_query.get("request_is_id", False),
                    "check_if_exists": custom_query.get("validator", {}).get(
                        "check_if_exists", False
                    ),
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                    "repositories": repositories,
                }

        custom_commands = feature.get("commands", [])
        for custom_command in custom_commands:
            custom_command_name = custom_command.get("name", "")
            custom_command_snake_name = stringcase.snakecase(custom_command_name)
            custom_command_pascal_name = stringcase.pascalcase(custom_command_name)
            dto_in_type_prefix = (
                custom_command.get("dto", {}).get("in", {}).get("type_prefix", "")
            )
            if custom_command_name:
                generation_dict["features"][feature_name]["commands"][
                    custom_command_name
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "commands",
                        custom_command_snake_name + "_command.h",
                    ),
                    "name": custom_command_pascal_name + "Command",
                    "dto_pascal": stringcase.pascalcase(dto_in_type_prefix + "DTO"),
                    "dto_snake": stringcase.snakecase(dto_in_type_prefix + "_dto"),
                    "request_is_id": custom_command.get("request_is_id", False),
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                }

            # if validator is enabled
            if custom_command.get("validator", {}).get("enabled", False):
                custom_command_entities = custom_command.get("entities", [])
                repositories = [
                    {
                        "camel_name": stringcase.camelcase(x),
                        "pascal_name": stringcase.pascalcase(x),
                        "snake_name": stringcase.snakecase(x),
                    }
                    for x in custom_command_entities
                ]
                generation_dict["features"][feature_name]["validators"][
                    custom_command_name
                ] = {
                    "file": os.path.join(
                        cqrs_common_cmake_folder_path,
                        feature_name_snake + "_cqrs",
                        feature_name_snake,
                        "validators",
                        custom_command_snake_name + "_command_validator.h",
                    ),
                    "name": custom_command_pascal_name + "CommandValidator",
                    "dto_pascal": stringcase.pascalcase(dto_in_type_prefix + "DTO"),
                    "dto_snake": stringcase.snakecase(dto_in_type_prefix + "_dto"),
                    "request_is_id": custom_command.get("request_is_id", False),
                    "check_if_exists": custom_command.get("validator", {}).get(
                        "check_if_exists", False
                    ),
                    "feature_name_snake": feature_name_snake,
                    "feature_name_pascal": feature_name_pascal,
                    "repositories": repositories,
                }

        # add all files in each generation_dict[feature]["files"]
        generation_dict["features"][feature_name]["files"] = []
        for query_name, query in generation_dict["features"][feature_name][
            "queries"
        ].items():
            generation_dict["features"][feature_name]["files"].append(query["file"])
        for command_name, command in generation_dict["features"][feature_name][
            "commands"
        ].items():
            generation_dict["features"][feature_name]["files"].append(command["file"])
        for validator_name, validator in generation_dict["features"][feature_name][
            "validators"
        ].items():
            generation_dict["features"][feature_name]["files"].append(validator["file"])
        # add the feature cmake file
        generation_dict["features"][feature_name]["files"].append(
            os.path.join(
                generation_dict["features"][feature_name]["cmake_folder_path"],
                "CMakeLists.txt",
            )
        )

    # add all files into the generation dict["files"]
    generation_dict["files"] = []
    for feature_name, feature in generation_dict["features"].items():
        for file in feature["files"]:
            generation_dict["files"].append(file)

    return generation_dict


def generate_query(query: dict, files_to_be_generated: dict[str, bool] = None):
    template_env = Environment(loader=FileSystemLoader("templates/CQRS"))
    template = template_env.get_template("query_template.jinja2")

    file_path = query["file"]

    if not files_to_be_generated.get(file_path, False):
        return file_path

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(file_path), exist_ok=True)

    with open(file_path, "w") as f:
        f.write(template.render(query=query))
        print(f"Successfully wrote file {file_path}")

    return file_path


def generate_command(command: dict, files_to_be_generated: dict[str, bool] = None):
    template_env = Environment(loader=FileSystemLoader("templates/CQRS"))
    template = template_env.get_template("command_template.jinja2")

    file_path = command["file"]

    if not files_to_be_generated.get(file_path, False):
        return file_path

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(file_path), exist_ok=True)

    with open(file_path, "w") as f:
        f.write(template.render(command=command))
        print(f"Successfully wrote file {file_path}")

    return file_path


def generate_validator(validator: dict, files_to_be_generated: dict[str, bool] = None):
    template_env = Environment(loader=FileSystemLoader("templates/CQRS"))
    template = template_env.get_template("validator_template.jinja2")

    file_path = validator["file"]

    if not files_to_be_generated.get(file_path, False):
        return file_path

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(file_path), exist_ok=True)

    with open(file_path, "w") as f:
        f.write(template.render(validator=validator))
        print(f"Successfully wrote file {file_path}")

    return file_path


def generate_feature_cmake(
    application_name: str, feature: dict, files_to_be_generated: dict[str, bool] = None
):
    template_env = Environment(loader=FileSystemLoader("templates/CQRS"))
    template = template_env.get_template("cmakelists_template.jinja2")

    cmake_file_path = os.path.join(feature["cmake_folder_path"], "CMakeLists.txt")
    files = feature["files"]

    # strip the cmakelists file from the files list
    files = [file for file in files if file != cmake_file_path]

    ## Convert the file path to be relative to the directory of the cmakelists
    relative_generated_files = []
    for file_path in files:
        relative_generated_file = os.path.relpath(
            file_path, os.path.dirname(cmake_file_path)
        )
        relative_generated_files.append(relative_generated_file)

    if not files_to_be_generated.get(cmake_file_path, False):
        return cmake_file_path

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(cmake_file_path), exist_ok=True)

    with open(cmake_file_path, "w") as f:
        f.write(
            template.render(
                feature=feature,
                files=relative_generated_files,
                application_name=application_name,
            )
        )
        print(f"Successfully wrote file {cmake_file_path}")

    return cmake_file_path


def generate_cmake(cqrs_dict: dict, files_to_be_generated: dict[str, bool] = None):
    # generate common cmakelists.txt
    common_cmake_folder_path = cqrs_dict["common_cmake_folder_path"]

    common_cmakelists_file = os.path.join(common_cmake_folder_path, "CMakeLists.txt")
    if files_to_be_generated.get(common_cmakelists_file, False):
        ## After the loop, write the list of features folders to the common cmakelists.txt
        with open(common_cmakelists_file, "w") as fh:
            for feature_name, _ in cqrs_dict["features"].items():
                fh.write(
                    f"add_subdirectory({stringcase.snakecase(feature_name)}_cqrs)"
                    + "\n"
                )
            print(f"Successfully wrote file {common_cmakelists_file}")

    return common_cmakelists_file


def generate_cqrs_files(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
):
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

    cqrs_data = manifest_data.get("CQRS", [])
    cqrs_common_cmake_folder_path = cqrs_data.get("common_cmake_folder_path", "")

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    cqrs_dict = get_cqrs_dict(
        cqrs_common_cmake_folder_path, feature_by_name, entities_by_name
    )

    for feature_name, feature in cqrs_dict["features"].items():
        generated_files = []
        for query_name, query in feature["queries"].items():
            generated_files.append(generate_query(query, files_to_be_generated))
        for command_name, command in feature["commands"].items():
            generated_files.append(generate_command(command, files_to_be_generated))
        for validator_name, validator in feature["validators"].items():
            generated_files.append(generate_validator(validator, files_to_be_generated))

        for file in generated_files:
            # if uncrustify_config_file and files_to_be_generated.get(file, False):
            #     uncrustify.run_uncrustify(file, uncrustify_config_file)
            if files_to_be_generated.get(file, False):
                clang_format.run_clang_format(file)

        # generate feature CMakeLists.txt
        generate_feature_cmake(application_name, feature, files_to_be_generated)

    # generate CMakeLists.txt
    generate_cmake(cqrs_dict, files_to_be_generated)


def get_files_to_be_generated(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
) -> list[str]:
    # Read the manifest file
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    cqrs_data = manifest_data.get("CQRS", [])
    cqrs_common_cmake_folder_path = cqrs_data.get("common_cmake_folder_path", "")

    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    cqrs_dict = get_cqrs_dict(
        cqrs_common_cmake_folder_path, feature_by_name, entities_by_name
    )

    files = cqrs_dict["files"]

    # generate common cmakelists.txt
    dto_cmakelists_file = os.path.join(cqrs_common_cmake_folder_path, "CMakeLists.txt")

    files.append(dto_cmakelists_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_cqrs_files(
    manifest_file: str,
    files_to_be_generated: dict[str, bool] = None,
    uncrustify_config_file: str = None,
):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["CQRS"]["common_cmake_folder_path"] = "preview/" + manifest["CQRS"][
        "common_cmake_folder_path"
    ].replace("..", "")
    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    # preprend preview/ to the file names in the dict files_to_be_generated and remove .. from the path
    if files_to_be_generated:
        preview_files_to_be_generated = {}
        for path, value in files_to_be_generated.items():
            preview_files_to_be_generated["preview/" + path.replace("..", "")] = value

        generate_cqrs_files(
            manifest_preview_file, preview_files_to_be_generated, uncrustify_config_file
        )

    else:
        generate_cqrs_files(manifest_preview_file, {}, uncrustify_config_file)


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
        preview_cqrs_files(file)
    else:
        # generate the files
        generate_cqrs_files(file)
