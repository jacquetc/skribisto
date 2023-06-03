from jinja2 import Environment, FileSystemLoader
import yaml
import os
import stringcase


def generate_cpp_files(manifest_file):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])
    export = entities_data.get("export", "")
    header_file = entities_data.get("export_header_file", "")
    path = entities_data.get("forder_path", ".")
    entities_list_file = entities_data.get("list_file", "generated_files.txt")

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    template_env = Environment(loader=FileSystemLoader("templates"))
    template = template_env.get_template("entity_template.h.jinja2")

    # Default initialization values
    default_init_values = {
        "int": "-1",
        "float": "0.0f",
        "double": "0.0",
        "bool": "false",
        "QString": "QString()",
        "QDateTime": "QDateTime::currentDateTime()",
        "QUuid": "QUuid::createUuid()",
        "QObject": "nullptr",
        "QList": "QList<>()",
    }

    generated_files = []

    for entity in entities_list:
        name = entity["name"]
        fields = entity["fields"]
        parent = entity.get("parent", "QObject")
        generate = entity.get("generate", True)

        # add name_pascal to fields
        for field in fields:
            field["name_pascal"] = stringcase.pascalcase(field["name"])

        if not generate:
            continue

        # Get the headers to include based on the field types
        headers = []
        for field in fields:
            field_type = field["type"]
            if field_type.startswith("QList"):
                include_header_file = (
                    field_type.split("<")[1].split(">")[0].strip().lower()
                )
                headers.append(f'"{include_header_file}.h"')
            elif field_type in ["QString", "QDateTime", "QUuid"]:
                headers.append(f"<{field_type}>")
        # remove duplicates
        headers = list(set(headers))

        # If the parent is a custom entity defined in the manifest, include its fields as well
        parent_fields = (
            entities_by_name.get(parent, {}).get("fields", [])
            if parent in entities_by_name
            else []
        )

        # only keep the fields from the current entity for initialization
        fields_init_values = ", ".join(
            [
                f"{field['name']}({default_init_values.get(field['type'].split('<')[0], '{}')})"
                for field in fields
            ]
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
            export=export,
            header_file=header_file,
            fields_init_values=fields_init_values,
            parent_init_values=parent_init_values,
        )

        output_file = os.path.join(path, f"{name.lower()}.h")

        # Create the directory if it does not exist
        os.makedirs(os.path.dirname(output_file), exist_ok=True)

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
generate_cpp_files("manifest.yaml")
