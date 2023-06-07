from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
from pathlib import Path


def generate_cpp_files(manifest_file):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    # remove entities that are not to be generated
    entities_list = [entity for entity in entities_list if entity.get("generate", True)]

    register_entity_base = entities_data.get("register_entity_base", False)
    export = entities_data.get("export", "")
    header_file = entities_data.get("export_header_file", "")
    path = entities_data.get("forder_path", ".")
    entities_list_file = entities_data.get("list_file", "generated_files.txt")

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    template_env = Environment(loader=FileSystemLoader("templates/entities"))
    template = template_env.get_template("entity_template.h.jinja2")

    # Default initialization values
    default_init_values = {
        "int": "0",
        "float": "0.0f",
        "double": "0.0",
        "bool": "false",
        "QString": "QString()",
        "QDateTime": "QDateTime()",
        "QUuid": "QUuid()",
        "QObject": "nullptr",
        "QList": "QList<>()",
    }

    def isUniqueForeignEntity(field_type: str) -> bool:
        for entity in entities_list:
            name = entity["name"]
            if name == field_type:
                return True

        return False

    def isListOrSetForeignEntity(field_type: str) -> bool:
        if "<" not in field_type:
            return False

        type = field_type.split("<")[1].split(">")[0].strip()

        for entity in entities_list:
            name = entity["name"]
            if name == type:
                return True

        return False

    generated_files = []

    for entity in entities_list:
        name = entity["name"]
        fields = entity["fields"]
        parent = entity.get("parent", "EntityBase")
        generate = entity.get("generate", True)

        # add name_pascal to fields
        for field in fields:
            field["name_pascal"] = stringcase.pascalcase(field["name"])

        # add need_lazy_loader to fields
        for field in fields:
            if isListOrSetForeignEntity(field["type"]) or isUniqueForeignEntity(
                field["type"]
            ):
                field["need_lazy_loader"] = True
            else:
                field["need_lazy_loader"] = False

        if not generate:
            continue

        # Get the headers to include based on the field types
        headers = []
        for field in fields:
            field_type = field["type"]
            if isListOrSetForeignEntity(field_type):
                include_header_file = field_type.split("<")[1].split(">")[0].strip()
                headers.append(f'"{stringcase.snakecase(include_header_file)}.h"')
            elif isUniqueForeignEntity(field_type):
                headers.append(f'"{stringcase.snakecase(field_type)}.h"')
            elif field_type in ["QString", "QDateTime", "QUuid"]:
                headers.append(f"<{field_type}>")

        # remove duplicates
        headers = list(set(headers))

        parent_header = f'"{stringcase.snakecase(parent)}.h"'

        # If the parent is a custom entity defined in the manifest, include its fields as well. Also, it will add the fields parent of the parent recursively, butonly if the parent is defined in the
        # manifest. If the parent is not defined in the manifest, it will only add the fields of the current entity. But it adds the field "id" of type "int" from EntityBase.
        # This is done to avoid having to add the field "id" in every entity defined in the manifest.

        # Initialize an empty list for parent fields
        parent_fields = []

        # Check if the parent of the current entity is a custom entity defined in the manifest
        if parent in entities_by_name:
            parent_fields.append(
                {
                    "name": "id",
                    "type": "int",
                    "need_lazy_loader": False,
                    "name_pascal": "Id",
                }
            )
            # If it is, then get the fields of the parent entity
            parent_fields += entities_by_name[parent]["fields"]

            # If parent has its own parent, recursively add its fields
            while (
                "parent" in entities_by_name[parent]
                and entities_by_name[parent]["parent"] in entities_by_name
            ):
                parent = entities_by_name[parent]["parent"]

                parent_fields += entities_by_name[parent]["fields"]
        else:
            parent_fields.append(
                {
                    "name": "id",
                    "type": "int",
                    "need_lazy_loader": False,
                    "name_pascal": "Id",
                }
            )

        # only keep the fields from the current entity for initialization
        fields_init_values = ", ".join(
            [
                f"m_{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
                for field in fields
                if not field["need_lazy_loader"]
            ]
        )
        fields_init_values = (
            ", " + fields_init_values if fields_init_values else fields_init_values
        )

        # use parent fields to initialize parent class in constructor
        parent_init_values = ", ".join(
            [
                f"{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
                for field in parent_fields
            ]
        )

        rendered_template = template.render(
            name=name,
            parent=parent,
            fields=fields,
            parent_fields=parent_fields,
            headers=headers,
            parent_header=parent_header,
            export=export,
            header_file=header_file,
            fields_init_values=fields_init_values,
            parent_init_values=parent_init_values,
        )

        output_file = os.path.join(path, f"{stringcase.snakecase(name)}.h")

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

        # Add the generated file to the list
        generated_files.append(output_file)

        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")

    # add entity_base
    if register_entity_base:
        name = "EntityBase"
        entities_list.append({"name": name})
        output_file = os.path.join(path, f"{stringcase.snakecase(name)}.h")
        generated_files.append(output_file)

    # generate domain registration file

    registration_template = template_env.get_template(
        "domain_registration_template.h.jinja2"
    )

    headers = []
    for entity in entities_list:
        name = entity["name"]
        headers.append(f'"{stringcase.snakecase(name)}.h"')

    rendered_template = registration_template.render(
        export=export,
        header_file=header_file,
        headers=headers,
        entities_list=entities_list,
    )

    output_file = os.path.join(path, "domain_registration.h")

    # Add the generated file to the list
    generated_files.append(output_file)

    with open(output_file, "w") as fh:
        fh.write(rendered_template)
        print(f"Successfully wrote file {output_file}")

    # After the loop, write the list of generated files to a file
    with open(entities_list_file, "w") as fh:
        for file_path in generated_files:
            # Convert the file path to be relative to the directory of the entities_list_file
            relative_path = os.path.relpath(
                file_path, os.path.dirname(entities_list_file)
            )
            fh.write(relative_path + "\n")


# Main execution
full_path = Path(__file__).resolve().parent

# add the current directory to the path so that we can import the generated files
sys.path.append(full_path)


# set the current directory to the generator directory
os.chdir(full_path)


file = "manifest.yaml"

# generate the files
generate_cpp_files(file)
