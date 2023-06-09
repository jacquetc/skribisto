from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
import copy
from pathlib import Path
from collections import OrderedDict


def get_dto_dict_and_feature_ordered_dict(
    feature_by_name: dict, entities_by_name: dict
) -> tuple[dict, OrderedDict]:
    def get_fields_with_foreign_entities(
        fields: list, entities_by_name: dict, entity_mappable_with: str = ""
    ) -> list:
        # make a deep copy of the fields
        fields = copy.deepcopy(fields)

        # get recursive fields from parent

        parent_fields = []
        entity_parent = entities_by_name.get(entity_mappable_with, {}).get("parent", "")

        while entity_parent:
            parent_fields = (
                get_entity_fields(entity_parent, entities_by_name) + parent_fields
            )
            entity_parent = entities_by_name.get(entity_parent, {}).get("parent", "")

        fields = parent_fields + fields

        # add fields with foreign entities
        for field in fields:
            field["pascal_name"] = stringcase.pascalcase(field["name"])

            if isUniqueForeignEntity(
                field["type"], entities_by_name
            ) or isListForeignEntity(field["type"], entities_by_name):
                field["is_foreign"] = True

                # get foreign entity name
                foreign_entity_name = getEntityFromForeignFieldType(
                    field["type"], entities_by_name
                )
                field["foreign_dto_type"] = f"{foreign_entity_name}DTO"
                field["entity_type"] = field["type"]
                field["type"] = (
                    f"{foreign_entity_name}DTO"
                    if field["type"].count(">") == 0
                    else f"QList<{foreign_entity_name}DTO>"
                )

            else:
                field["is_foreign"] = False

        return fields

    def get_fields_without_foreign_entities(
        fields: list, entities_by_name: dict, entity_mappable_with: str = ""
    ) -> list:
        # make a deep copy of the fields
        fields = copy.deepcopy(fields)

        # get recursive fields from parent

        parent_fields = []
        entity_parent = entities_by_name.get(entity_mappable_with, {}).get("parent", "")

        while entity_parent:
            parent_fields = (
                get_entity_fields(entity_parent, entities_by_name) + parent_fields
            )
            entity_parent = entities_by_name.get(entity_parent, {}).get("parent", "")

        fields = parent_fields + fields

        # add fields without foreign entities
        fields_without_foreign = []
        for field in fields:
            field["pascal_name"] = stringcase.pascalcase(field["name"])

            if isUniqueForeignEntity(
                field["type"], entities_by_name
            ) or isListForeignEntity(field["type"], entities_by_name):
                continue

            else:
                field["is_foreign"] = False
                fields_without_foreign.append(field)

        return fields_without_foreign

    def determine_dto_dependencies_from_fields(fields: list) -> list:
        dto_dependencies = []

        for field in fields:
            if field["is_foreign"]:
                dto_dependencies.append(field["foreign_dto_type"])

        return dto_dependencies

    dto_dict = {}

    for feature in feature_by_name.values():
        # fetch basic entity DTOs

        feature_dto_data = feature.get("DTO", {})
        dto_identical_to_entity = feature_dto_data.get("dto_identical_to_entity", {})

        entity_mappable_with = dto_identical_to_entity.get("entity_mappable_with", "")
        generate_dto_identical_to_entity = dto_identical_to_entity.get(
            "enabled", False
        ) and dto_identical_to_entity.get("entity_mappable_with", "")

        # create DTO entry without foreign DTOs

        if generate_dto_identical_to_entity:
            dto_type_name = f"{stringcase.pascalcase(feature['name'])}DTO"

            dto_dict[dto_type_name] = {
                "feature_name": stringcase.pascalcase(feature["name"]),
                "generate": dto_identical_to_entity.get("generate", True),
                "entity_mappable_with": entity_mappable_with,
                "file_name": f"{stringcase.snakecase(feature['name'])}_dto.h",
                "fields": [],
            }

            # add fields without foreign entities

            dto_dict[dto_type_name]["fields"] = get_fields_without_foreign_entities(
                entities_by_name[entity_mappable_with]["fields"],
                entities_by_name,
                entity_mappable_with,
            )

            dto_dict[dto_type_name]["dto_dependencies"] = []

        # create DTO entry with foreign DTOs (details)

        if generate_dto_identical_to_entity and entityHaveForeignEntity(
            entity_mappable_with, entities_by_name
        ):
            dto_type_name = f"{stringcase.pascalcase(feature['name'])}WithDetailsDTO"
            dto_dict[dto_type_name] = {
                "feature_name": stringcase.pascalcase(feature["name"]),
                "generate": dto_identical_to_entity.get("generate", True),
                "entity_mappable_with": entity_mappable_with,
                "file_name": f"{stringcase.snakecase(feature['name'])}_with_details_dto.h",
                "fields": [],
            }

            # add fields with foreign entities
            dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                entities_by_name[entity_mappable_with]["fields"],
                entities_by_name,
                entity_mappable_with,
            )

            dto_dict[dto_type_name][
                "dto_dependencies"
            ] = determine_dto_dependencies_from_fields(
                dto_dict[dto_type_name]["fields"]
            )

        # fetch CRUD DTOs

        crud_data = feature.get("CRUD", {})
        crud_enabled = feature.get("CRUD", {}).get("enabled", {})
        entity_mappable_with = crud_data.get("entity_mappable_with", "")

        if crud_data and crud_enabled and entity_mappable_with:
            generate_create = False
            generate_update = False

            if crud_data["enabled"]:
                generate_create = crud_data["create"].get("enabled", False)
                generate_update = crud_data["update"].get("enabled", False)

            if generate_create:
                dto_type_name = f"Create{stringcase.pascalcase(feature['name'])}DTO"
                generate = crud_data["create"].get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": entity_mappable_with,
                    "file_name": f"create_{stringcase.snakecase(feature['name'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities but without id
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    entities_by_name[entity_mappable_with]["fields"],
                    entities_by_name,
                    entity_mappable_with,
                )

                # remove id field
                dto_dict[dto_type_name]["fields"] = [
                    field
                    for field in dto_dict[dto_type_name]["fields"]
                    if field["name"] != "id"
                ]

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            if generate_update:
                dto_type_name = f"Update{stringcase.pascalcase(feature['name'])}DTO"
                generate = crud_data["update"].get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": entity_mappable_with,
                    "file_name": f"update_{stringcase.snakecase(feature['name'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    entities_by_name[entity_mappable_with]["fields"],
                    entities_by_name,
                    entity_mappable_with,
                )

                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

        # fetch command DTOs
        for command in feature.get("commands", []):
            command_dto_data = command.get("dto", {})
            dto_in = command_dto_data.get("in", {})
            if dto_in:
                dto_type_name = f"{dto_in['type_prefix']}DTO"
                generate = dto_in.get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_in['type_prefix'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    dto_in["fields"], entities_by_name
                )
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            dto_out = command_dto_data.get("out", {})
            if dto_out:
                dto_type_name = f"{dto_out['type_prefix']}DTO"
                generate = dto_out.get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_out['type_prefix'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    dto_out["fields"], entities_by_name
                )
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

        # fetch query DTOs
        for query in feature.get("queries", []):
            query_dto_data = query.get("dto", {})
            dto_in = query_dto_data.get("in", {})
            if dto_in:
                dto_type_name = f"{dto_in['type_prefix']}DTO"
                generate = dto_in.get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_in['type_prefix'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    dto_in["fields"], entities_by_name
                )
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

            dto_out = query_dto_data.get("out", {})
            if dto_out:
                dto_type_name = f"{dto_out['type_prefix']}DTO"
                generate = dto_out.get("generate", True)
                dto_dict[dto_type_name] = {
                    "feature_name": stringcase.pascalcase(feature["name"]),
                    "generate": generate,
                    "entity_mappable_with": "",
                    "file_name": f"{stringcase.snakecase(dto_out['type_prefix'])}_dto.h",
                    "fields": [],
                }

                # add fields with foreign entities
                dto_dict[dto_type_name]["fields"] = get_fields_with_foreign_entities(
                    dto_out["fields"], entities_by_name
                )
                dto_dict[dto_type_name][
                    "dto_dependencies"
                ] = determine_dto_dependencies_from_fields(
                    dto_dict[dto_type_name]["fields"]
                )

    # create header files for DTOs
    for dto_type_name, dto_data in dto_dict.items():
        header_list = []
        for field in dto_data["fields"]:
            if field["type"] in ["QString", "QDateTime", "QUuid", "QUrl"]:
                header_list.append(f"<{field['type']}>")

        dto_dependencies = dto_data.get("dto_dependencies", [])
        for dto_dependency in dto_dependencies:
            if dto_dependency in dto_dict:
                dto_dependency_file_name = dto_dict[dto_dependency]["file_name"]
                header_list.append(f'"{dto_dependency_file_name}"')

        header_list = list(set(header_list))
        header_list.sort()

        dto_data["header_list"] = header_list

    # add "foreign_feature_name" to foreign fields in DTOs
    for dto_type_name, dto_data in dto_dict.items():
        for field in dto_data["fields"]:
            if not field["is_foreign"]:
                continue
            foreign_dto_type = field["foreign_dto_type"]
            field["foreign_feature_name"] = dto_dict[foreign_dto_type]["feature_name"]

            # dto_data["fields"][i] = field

    # create a new dict "feature_dto_dict" with only the DTOs that are needed for the feature. Each key is a feature name and each value is a dict.
    # The dict contains "dtos" with the list of DTOs. The dict also contains a "feature_dependencies" list with the features that the feature depends on.

    feature_dto_dict = {}
    for dto_type_name, dto_data in dto_dict.items():
        feature_name = dto_data["feature_name"]
        if feature_name not in feature_dto_dict:
            feature_dto_dict[feature_name] = {
                "feature_dependencies": [],
                "dtos": [],
            }

        if not feature_dto_dict[feature_name].get("dtos", {}):
            feature_dto_dict[feature_name]["dtos"] = {}
        feature_dto_dict[feature_name]["dtos"][dto_type_name] = dto_data

        if dto_data["dto_dependencies"]:
            feature_dto_dict[feature_name]["feature_dependencies"].extend(
                dto_data["dto_dependencies"]
            )

    # replace the DTOs in "feature_dependencies" with the feature name of each of these DTOs
    for feature_name, feature_data in feature_dto_dict.items():
        for i, feature_dependency in enumerate(feature_data["feature_dependencies"]):
            feature_data["feature_dependencies"][i] = dto_dict[feature_dependency][
                "feature_name"
            ]

    # remove duplicates from feature_dependencies
    for feature_name, feature_data in feature_dto_dict.items():
        feature_data["feature_dependencies"] = list(
            set(feature_data["feature_dependencies"])
        )

    # order the features by their dependencies, so that the features that depend on other features are generated after the features they depend on

    dto_ordered_dict = OrderedDict()
    original_feature_dto_dict = copy.deepcopy(feature_dto_dict)

    while len(feature_dto_dict) > 0:
        for feature_name, feature_data in feature_dto_dict.items():
            if len(feature_data["feature_dependencies"]) == 0:
                dto_ordered_dict[feature_name] = {}
                dto_ordered_dict[feature_name]["dtos"] = feature_data["dtos"]
                dto_ordered_dict[feature_name][
                    "feature_dependencies"
                ] = original_feature_dto_dict[feature_name]["feature_dependencies"]

                del feature_dto_dict[feature_name]
                for feature_name2, feature_data2 in feature_dto_dict.items():
                    if feature_name in feature_data2["feature_dependencies"]:
                        feature_data2["feature_dependencies"].remove(feature_name)
                break

    return dto_dict, dto_ordered_dict


def isUniqueForeignEntity(field_type: str, entities_by_name: dict) -> bool:
    for entity_name in entities_by_name:
        if entity_name == field_type:
            return True

    return False


def isListForeignEntity(field_type: str, entities_by_name: dict) -> bool:
    if "<" not in field_type:
        return False

    type = field_type.split("<")[1].split(">")[0].strip()

    for entity_name in entities_by_name:
        if entity_name == type:
            return True

    return False


def getEntityFromForeignFieldType(field_type: str, entities_by_name: dict) -> str:
    if "<" not in field_type:
        return ""

    type = field_type.split("<")[1].split(">")[0].strip()

    for entity_name in entities_by_name:
        if entity_name == type:
            return entity_name

    return ""


def get_entity_fields(entity_name: str, entities_by_name: dict) -> list:
    if entity_name == "EntityBase":
        return [{"name": "id", "type": "int", "pascal_name": "Id", "is_foreign": False}]

    entity_data = entities_by_name[entity_name]
    fields = entity_data["fields"]
    return fields


def entityHaveForeignEntity(entity_name: str, entities_by_name: dict) -> bool:
    entity = entities_by_name.get(entity_name, None)
    if entity is None:
        return False

    fields = entity["fields"]
    for field in fields:
        field_type = field["type"]
        if isUniqueForeignEntity(field_type, entities_by_name) or isListForeignEntity(
            field_type, entities_by_name
        ):
            return True

    return False


def isUniqueForeignDTO(dto_list: list, field_type: str) -> bool:
    for dto_type in dto_list:
        if dto_type == field_type:
            return True

    return False


def isListForeignDTO(dto_list: list, field_type: str) -> bool:
    if "<" not in field_type:
        return False

    type = field_type.split("<")[1].split(">")[0].strip()

    for dto_type in dto_list:
        if dto_type == type:
            return True

    return False


""" 

def generate_dto(
    template,
    feature_snake_name,
    dto_pascal_type,
    dto_file_name,
    dto_common_cmake_folder_path,
    fields,
    default_init_values,
    dto_list,
) -> str:
    dto_file_path = os.path.join(
        dto_common_cmake_folder_path, feature_snake_name, dto_file_name
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(dto_file_path), exist_ok=True)

    # transform foreigner entities to DTOs
    # bad hack to infer DTOs from entities
    for field in fields:
        field_type = field["type"]
        if "<" in field_type:
            # insert 'DTO' before the last '>'
            field_type = field_type.replace(">", "DTO>", 1)
            if isListForeignDTO(dto_list, field_type):
                entity_name = field_type.split("<")[1].split(">")[0].strip()
                field["type"] = field_type
        elif isUniqueForeignDTO(dto_list, f"{field_type}DTO"):
            field["type"] = f"{entity_name}DTO"
        elif field_type in ["QString", "QDateTime", "QUuid"]:
            continue

    # only keep the fields from the current entity for initialization
    fields_init_values = ", ".join(
        [
            f"m_{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
            for field in fields
            if field["type"].count("<") == 0
        ]
    )

    # Get the headers to include based on the field types
    headers = []
    for field in fields:
        field_type = field["type"]
        if isListForeignDTO(dto_list, field_type):
            include_header_file = field_type.split("<")[1].split(">")[0].strip()
            headers.append(
                f'"{stringcase.snakecase(include_header_file)}.h"'.replace(
                    "_d_t_o", "_dto"
                )
            )
        elif isUniqueForeignDTO(dto_list, field_type):
            headers.append(f'"{stringcase.snakecase(field_type)}.h"')
        elif field_type in ["QString", "QDateTime", "QUuid"]:
            headers.append(f"<{field_type}>")

    # remove duplicates
    headers = list(set(headers))

    with open(dto_file_path, "w") as f:
        f.write(
            template.render(
                feature_pascal_name=stringcase.pascalcase(feature_snake_name),
                dto_pascal_type=dto_pascal_type,
                fields=fields,
                headers=headers,
                fields_init_values=fields_init_values,
            )
        )
        print(f"Successfully wrote file {dto_file_path}")

    return dto_file_path

"""


def generate_dto(
    template,
    dto_type,
    dto_data,
    dto_common_cmake_folder_path,
    default_init_values,
):
    fields = dto_data.get("fields", [])

    # only keep the fields from the current entity for initialization
    fields_init_values = ", ".join(
        [
            f"m_{field['name']}({default_init_values.get(field['type'], '{}')})"
            for field in fields
            if not field["is_foreign"]
        ]
    )

    # Get the headers to include based on the field types
    headers = dto_data["header_list"]

    dto_file_path = os.path.join(
        dto_common_cmake_folder_path,
        stringcase.snakecase(dto_data["feature_name"]),
        dto_data["file_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(dto_file_path), exist_ok=True)

    with open(dto_file_path, "w") as f:
        f.write(
            template.render(
                feature_pascal_name=dto_data["feature_name"],
                dto_pascal_type=dto_type,
                fields=fields,
                headers=headers,
                fields_init_values=fields_init_values,
            )
        )
        print(f"Successfully wrote file {dto_file_path}")


def generate_dto_files(manifest_file):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    dto_data = manifest_data.get("DTOs", [])
    application_name = manifest_data.get("global", {}).get(
        "application_name", "example"
    )
    application_name = stringcase.spinalcase(application_name)
    dto_common_cmake_folder_path = dto_data.get("common_cmake_folder_path", "")

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    template_env = Environment(loader=FileSystemLoader("templates/DTOs"))
    template = template_env.get_template("dto_template.h.jinja2")

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    # Default initialization values
    default_init_values = {
        "int": "0",
        "float": "0.0f",
        "double": "0.0",
        "bool": "false",
        "QString": "QString()",
        "QDateTime": "QDateTime()",
        "QUuid": "QUuid()",
        "QUrl": "QUrl()",
        "QObject": "nullptr",
        "QList": "QList<>()",
    }

    dto_dict, feature_ordered_dict = get_dto_dict_and_feature_ordered_dict(
        feature_by_name, entities_by_name
    )

    for feature_name, feature_data in feature_ordered_dict.items():
        generated_files = []
        feature_snake_name = stringcase.snakecase(feature_name)

        for dto_type, dto_data in feature_data["dtos"].items():
            generate_dto(
                template,
                dto_type,
                dto_data,
                dto_common_cmake_folder_path,
                default_init_values,
            )
            generated_files.append(
                os.path.join(
                    dto_common_cmake_folder_path,
                    feature_snake_name,
                    dto_data["file_name"],
                )
            )

        # generate these DTO's cmakelists.txt
        dto_cmakelists_template = template_env.get_template(
            "cmakelists_template.jinja2"
        )

        dto_cmakelists_file = os.path.join(
            dto_common_cmake_folder_path, feature_snake_name, "CMakeLists.txt"
        )

        ## Convert the file path to be relative to the directory of the cmakelists
        relative_generated_files = []
        for file_path in generated_files:
            relative_generated_file = os.path.relpath(
                file_path, os.path.dirname(dto_cmakelists_file)
            )
            relative_generated_files.append(relative_generated_file)

        # Get the feature dependencies and make them spinal case
        spinal_case_feature_dependencies = []
        for feature_dependency in feature_data["feature_dependencies"]:
            spinal_case_feature_dependencies.append(
                stringcase.spinalcase(feature_dependency)
            )

        rendered_template = dto_cmakelists_template.render(
            feature_snake_name=stringcase.spinalcase(feature_snake_name),
            files=relative_generated_files,
            application_name=application_name,
            feature_dependencies=spinal_case_feature_dependencies,
        )

        with open(dto_cmakelists_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {dto_cmakelists_file}")

    # generate common cmakelists.txt

    dto_common_cmakelists_file = os.path.join(
        dto_common_cmake_folder_path, "CMakeLists.txt"
    )

    ## After the loop, write the list of features folders to the common cmakelists.txt
    with open(dto_common_cmakelists_file, "w") as fh:
        for feature_name, _ in feature_ordered_dict.items():
            fh.write(f"add_subdirectory({stringcase.snakecase(feature_name)})" + "\n")
        print(f"Successfully wrote file {dto_common_cmakelists_file}")


def get_files_to_be_generated(manifest_file: str) -> list[str]:
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

    dto_data = manifest_data.get("DTOs", [])
    dto_common_cmake_folder_path = dto_data.get("common_cmake_folder_path", "")

    application_data = manifest_data.get("application", [])
    feature_list = application_data.get("features", [])

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    dto_dict, feature_ordered_dict = get_dto_dict_and_feature_ordered_dict(
        feature_by_name, entities_by_name
    )

    files_to_be_generated = []
    for feature_name, feature_data in feature_ordered_dict.items():
        feature_snake_name = stringcase.snakecase(feature_name)

        for dto_type, dto_data in feature_data["dtos"].items():
            files_to_be_generated.append(
                os.path.join(
                    dto_common_cmake_folder_path,
                    feature_snake_name,
                    dto_data["file_name"],
                )
            )

        dto_cmakelists_file = os.path.join(
            dto_common_cmake_folder_path, feature_snake_name, "CMakeLists.txt"
        )

        files_to_be_generated.append(dto_cmakelists_file)

    # generate common cmakelists.txt
    dto_cmakelists_file = os.path.join(dto_common_cmake_folder_path, "CMakeLists.txt")

    files_to_be_generated.append(dto_cmakelists_file)

    return files_to_be_generated


# generate the files into the preview folder
def preview_dto_files(manifest_file: str):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["DTOs"]["common_cmake_folder_path"] = "preview/" + manifest["DTOs"][
        "common_cmake_folder_path"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    generate_dto_files(manifest_preview_file)


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
        preview_dto_files(file)
    else:
        # generate the files
        generate_dto_files(file)
