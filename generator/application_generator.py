from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
from pathlib import Path


def get_generation_dict(
    common_cmake_folder_path: str,
    export: str,
    export_header_file: str,
    feature_by_name: dict,
) -> dict:
    generation_dict = {}
    for feature_name, feature in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_spinal_name = stringcase.spinalcase(feature_name)
        generation_dict[feature_pascal_name] = {
            "feature_snake_name": feature_snake_name,
            "feature_pascal_name": feature_pascal_name,
            "feature_spinal_name": feature_spinal_name,
        }

    for feature_name, feature in generation_dict.items():
        feature_snake_name = feature["feature_snake_name"]
        feature["crud_handlers"] = {}
        crud_handlers = feature["crud_handlers"]
        crud_data = feature_by_name[feature_name].get("CRUD", {})
        if crud_data.get("enabled", False):
            create_data = crud_data.get("create", {})
            if create_data.get("enabled", False) and create_data.get("generate", True):
                crud_handlers["create"] = {
                    "generate": True,
                    "templates": [
                        "create_handler.cpp.jinja2",
                        "create_handler.h.jinja2",
                    ],
                    "files": [
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "commands",
                            f"create_{feature_snake_name}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "commands",
                            f"create_{feature_snake_name}_command_handler.h",
                        ),
                    ],
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
                            feature_snake_name,
                            "commands",
                            f"remove_{feature_snake_name}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "commands",
                            f"remove_{feature_snake_name}_command_handler.h",
                        ),
                    ],
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
                            feature_snake_name,
                            "commands",
                            f"update_{feature_snake_name}_command_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "commands",
                            f"update_{feature_snake_name}_command_handler.h",
                        ),
                    ],
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
                            feature_snake_name,
                            "queries",
                            f"get_{feature_snake_name}_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "queries",
                            f"get_{feature_snake_name}_query_handler.h",
                        ),
                    ],
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
                            feature_snake_name,
                            "queries",
                            f"get_all_{feature_snake_name}_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "queries",
                            f"get_all_{feature_snake_name}_query_handler.h",
                        ),
                    ],
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
                            feature_snake_name,
                            "queries",
                            f"get_with_details_{feature_snake_name}_query_handler.cpp",
                        ),
                        os.path.join(
                            common_cmake_folder_path,
                            feature_snake_name,
                            "queries",
                            f"get_with_details_{feature_snake_name}_query_handler.h",
                        ),
                    ],
                }
        for handler_name, handler_data in crud_handlers.items():
            handler_data["export"] = export
            handler_data["export_header_file"] = export_header_file

    # add commands and queries to the feature:
    for feature_name, feature in generation_dict.items():
        feature["custom_handlers"] = {}
        if feature_by_name[feature_name].get("commands", None):
            feature["custom_handlers"]["commands"] = feature_by_name[feature_name][
                "commands"
            ]

            for command in feature["custom_handlers"]["commands"]:
                files = [
                    os.path.join(
                        common_cmake_folder_path,
                        feature_snake_name,
                        "commands",
                        f"{stringcase.snakecase(command['name'])}_command_handler.h",
                    ),
                    os.path.join(
                        common_cmake_folder_path,
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
                command["export"] = export
                command["export_header_file"] = export_header_file
                command["camel_name"] = stringcase.camelcase(command["name"])
                command["snake_name"] = stringcase.snakecase(command["name"])
                command["pascal_name"] = stringcase.pascalcase(command["name"])
                # DTO out
                if command.get("dto", {}).get("out", {}):
                    if command.get("dto", {}).get("out", {}).get("is_void", False):
                        command["dto_out_pascal_name"] = "void"
                        command["dto_out_camel_name"] = "void"
                    else:
                        command["dto_out_pascal_name"] = stringcase.pascalcase(
                            command["dto"]["out"]["type_prefix"] + "DTO"
                        )
                        command["dto_out_camel_name"] = stringcase.camelcase(
                            command["dto"]["out"]["type_prefix"] + "DTO"
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
                        feature_snake_name,
                        "queries",
                        f"{stringcase.snakecase(query['name'])}_query_handler.h",
                    ),
                    os.path.join(
                        common_cmake_folder_path,
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
                query["export"] = export
                query["export_header_file"] = export_header_file
                query["camel_name"] = stringcase.camelcase(query["name"])
                query["snake_name"] = stringcase.snakecase(query["name"])
                query["pascal_name"] = stringcase.pascalcase(query["name"])

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
            common_cmake_folder_path, feature_snake_name, "CMakeLists.txt"
        )

    return generation_dict


def generate_handler_cmakelists(
    feature: dict, application_name: str, files_to_be_generated: dict[str, bool]
):
    # generate these DTO's cmakelists.txt

    template_env = Environment(loader=FileSystemLoader("templates/application/CRUD"))
    dto_cmakelists_template = template_env.get_template("cmakelists_template.jinja2")

    dto_cmakelists_file = feature["cmakelists_file"]

    if files_to_be_generated.get(dto_cmakelists_file, False):
        return

    ## Convert the file path to be relative to the directory of the cmakelists
    relative_generated_files = []
    for file_path in feature["handler_files"]:
        relative_generated_file = os.path.relpath(
            file_path, os.path.dirname(dto_cmakelists_file)
        )
        relative_generated_files.append(relative_generated_file)

    rendered_template = dto_cmakelists_template.render(
        feature_snake_name=feature["feature_snake_name"],
        files=relative_generated_files,
        application_name=application_name,
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(dto_cmakelists_file), exist_ok=True)

    with open(dto_cmakelists_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {dto_cmakelists_file}")


def generate_crud_handler(
    handler: dict, feature: dict, files_to_be_generated: dict[str, bool]
):
    for file_path in handler["files"]:
        if files_to_be_generated.get(file_path, False):
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
                    export_header_file=handler["export_header_file"],
                )
            )
            print(f"Successfully wrote file {file_path}")


def generate_custom_command_handler(
    handler: dict, feature: dict, files_to_be_generated: dict[str, bool]
):
    for file_path in handler["files"]:
        if files_to_be_generated.get(file_path, False):
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
                    export_header_file=handler["export_header_file"],
                    validator_enabled=handler.get("validator", {}).get(
                        "enabled", False
                    ),
                )
            )
            print(f"Successfully wrote file {file_path}")


def generate_custom_query_handler(
    handler: dict, feature: dict, files_to_be_generated: dict[str, bool]
):
    for file_path in handler["files"]:
        if files_to_be_generated.get(file_path, False):
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
                    query=handler,
                    export=handler["export"],
                    export_header_file=handler["export_header_file"],
                    validator_enabled=handler.get("validator", {}).get(
                        "enabled", False
                    ),
                )
            )
            print(f"Successfully wrote file {file_path}")


def generate_application_files(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
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
    export = application_data.get("export", "")
    export_header_file = application_data.get("export_header_file", "")
    feature_list = application_data.get("features", [])
    common_cmake_folder_path = application_data.get("common_cmake_folder_path", "")

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    for _, feature in get_generation_dict(
        common_cmake_folder_path, export, export_header_file, feature_by_name
    ).items():
        # generate crud handlers
        for handler in feature["crud_handlers"].values():
            generate_crud_handler(handler, feature, files_to_be_generated)

        # generate custom handlers
        if feature.get("custom_handlers", None):
            if feature["custom_handlers"].get("commands", None):
                for command_handler in feature["custom_handlers"]["commands"]:
                    generate_custom_command_handler(
                        command_handler, feature, files_to_be_generated
                    )
            if feature["custom_handlers"].get("queries", None):
                for query_handler in feature["custom_handlers"]["queries"]:
                    generate_custom_query_handler(
                        query_handler, feature, files_to_be_generated
                    )

        # generate handler cmakelists.txt
        generate_handler_cmakelists(feature, application_name, files_to_be_generated)


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
    export = application_data.get("export", "")
    export_header_file = application_data.get("export_header_file", "")
    feature_list = application_data.get("features", [])
    common_cmake_folder_path = application_data.get("common_cmake_folder_path", "")

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    files = []
    for _, feature in get_generation_dict(
        common_cmake_folder_path, export, export_header_file, feature_by_name
    ).items():
        files += feature["handler_files"]

        common_cmake_file = feature["cmakelists_file"]
        files.append(common_cmake_file)

    # # add CMakelists.txt:
    common_cmake_file = Path.joinpath(common_cmake_folder_path, "CMakeLists.txt")
    files.append(common_cmake_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_application_files(
    manifest_file: str, files_to_be_generated: dict[str, bool] = {}
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
        for path, _ in files_to_be_generated.items():
            files_to_be_generated[path] = "preview/" + path.replace("..", "")

        generate_application_files(manifest_preview_file, files_to_be_generated)

    else:
        generate_application_files(manifest_preview_file)


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
