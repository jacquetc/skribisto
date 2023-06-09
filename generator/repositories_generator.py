from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
from pathlib import Path


def generate_repository_files(manifest_file):
    with open(manifest_file, "r") as stream:
        try:
            manifest_data = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)
            return

    repositories_data = manifest_data.get("repositories", [])
    repositories_list = repositories_data.get("list", [])
    export = repositories_data.get("export", "")
    export_header_file = repositories_data.get("export_header_file", "")
    path = repositories_data.get("folder_path", ".")
    interface_path = repositories_data.get("interface_path", ".")
    repositories_list_file = repositories_data.get("list_file", "generated_files.txt")
    interface_list_file = repositories_data.get(
        "interface_list_file", "generated_interface_files.txt"
    )

    template_env = Environment(loader=FileSystemLoader("templates/repositories"))
    repo_template = template_env.get_template("repository_template.h.jinja2")
    repo_cpp_template = template_env.get_template("repository_template.cpp.jinja2")
    iface_template = template_env.get_template("interface_repository_template.h.jinja2")

    contracts_data = manifest_data.get("contracts", [])
    contracts_export = contracts_data.get("export", "")
    contracts_export_header_file = contracts_data.get("export_header_file", "")
    inverted_app_domain = contracts_data.get("inverted_app_domain", "com.example")

    entities_data = manifest_data.get("entities", [])
    entities_list = entities_data.get("list", [])

    # Organize entities by name for easier lookup
    entities_by_name = {entity["name"]: entity for entity in entities_list}

    generated_files = []
    interface_generated_files = []

    def is_unique_foreign_entity(field_type: str) -> bool:
        for entity in entities_list:
            name = entity["name"]
            if name == field_type:
                return True

        return False

    def is_list_foreign_entity(field_type: str) -> bool:
        if "<" not in field_type:
            return False

        type = field_type.split("<")[1].split(">")[0].strip()

        for entity in entities_list:
            name = entity["name"]
            if name == type:
                return True

        return False

    for repository in repositories_list:
        name = repository["entity_name"]
        pascal_name = stringcase.pascalcase(name)
        snake_name = stringcase.snakecase(name)
        camel_name = stringcase.camelcase(name)
        generate = repository.get("generate", True)
        generate_lazy_loaders = repository.get("lazy_loaders", True)

        if not generate:
            continue

        # create a dict of foreign entities for this repository
        foreign_entities = {}
        for field in entities_by_name[name]["fields"]:
            field_type = field["type"]
            if is_unique_foreign_entity(field_type):
                foreign_entities[field_type] = entities_by_name[field_type]
            if is_list_foreign_entity(field_type):
                foreign_entities[field_type] = entities_by_name[
                    field_type.split("<")[1].split(">")[0].strip()
                ]

        # add helpful keys to foreign entities
        for key, value in foreign_entities.items():
            value["is_list"] = key.split("<")[0] == "QList" and is_list_foreign_entity(
                key
            )
            if value["is_list"]:
                value["type_camel_name"] = stringcase.camelcase(
                    key.split("<")[1].split(">")[0].strip()
                )
            else:
                value["type_camel_name"] = stringcase.camelcase(key)

            if value["is_list"]:
                value["type_name_only"] = key.split("<")[1].split(">")[0].strip()
            else:
                value["type_name_only"] = stringcase.camelcase(key)

            for field in entities_by_name[name]["fields"]:
                field_type = field["type"]
                if field_type == key:
                    value["related_field_name"] = field["name"]
                    value["related_field_pascal_name"] = stringcase.pascalcase(
                        field["name"]
                    )

        foreign_database_table_constructor_arguments = []
        if generate_lazy_loaders:
            for key, value in foreign_entities.items():
                foreign_database_table_constructor_arguments.append(
                    f"InterfaceDatabaseTable<Domain::{value['type_name_only']}> *{value['type_camel_name']}Database"
                )
        foreign_database_table_constructor_arguments_string = ", ".join(
            foreign_database_table_constructor_arguments
        )
        foreign_database_table_constructor_arguments_string = (
            ", " + foreign_database_table_constructor_arguments_string
            if foreign_database_table_constructor_arguments
            else foreign_database_table_constructor_arguments_string
        )

        loader_private_member_list = []
        if generate_lazy_loaders:
            for key, value in foreign_entities.items():
                loader_private_member_list.append(
                    f"InterfaceDatabaseTable<Domain::{value['type_name_only']}> *m_{value['type_camel_name']}Database;"
                )
        # loader functions like     Domain::Book::ChaptersLoader fetchChaptersLoader();

        loader_function_list = []
        if generate_lazy_loaders:
            for key, value in foreign_entities.items():
                loader_function_list.append(
                    f"Domain::{name}::{value['related_field_pascal_name']}Loader fetch{value['related_field_pascal_name']}Loader();"
                )

        # Create .h file
        rendered_template = repo_template.render(
            name=name,
            pascal_name=pascal_name,
            snake_name=snake_name,
            camel_name=camel_name,
            foreign_database_table_constructor_arguments_string=foreign_database_table_constructor_arguments_string,
            loader_private_member_list=loader_private_member_list,
            loader_function_list=loader_function_list,
            export=export,
            export_header_file=export_header_file,
        )
        output_file = os.path.join(path, f"{snake_name}_repository.h")
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")
        generated_files.append(output_file)

        # prepare the fields init values

        fields_init_values = ", ".join(
            [
                f"m_{value['type_camel_name']}Database({value['type_camel_name']}Database)"
                for key, value in foreign_entities.items()
            ]
        )
        fields_init_values = (
            ", " + fields_init_values if fields_init_values else fields_init_values
        )

        # Create .cpp file
        rendered_template = repo_cpp_template.render(
            name=name,
            pascal_name=pascal_name,
            snake_name=snake_name,
            camel_name=camel_name,
            foreign_entities=foreign_entities,
            foreign_database_table_constructor_arguments_string=foreign_database_table_constructor_arguments_string,
            fields_init_values=fields_init_values,
        )
        output_file = os.path.join(path, f"{snake_name}_repository.cpp")
        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")
        generated_files.append(output_file)

        # Create interface .h file
        rendered_template = iface_template.render(
            name=name,
            snake_name=snake_name,
            inverted_app_domain=inverted_app_domain,
            contracts_export=contracts_export,
            contracts_export_header_file=contracts_export_header_file,
        )
        output_file = os.path.join(
            interface_path, f"interface_{snake_name}_repository.h"
        )
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")
        interface_generated_files.append(output_file)

    # write the repository list file

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(repositories_list_file), exist_ok=True)

    # After the loop, write the list of generated files to a file
    with open(repositories_list_file, "w") as fh:
        for file_path in generated_files:
            # Convert the file path to be relative to the directory of the repositories_list_file
            relative_path = os.path.relpath(
                file_path, os.path.dirname(repositories_list_file)
            )
            fh.write(relative_path + "\n")

    # write the interface list file

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(interface_list_file), exist_ok=True)

    # After the loop, write the list of generated files to a file
    with open(interface_list_file, "w") as fh:
        for file_path in interface_generated_files:
            # Convert the file path to be relative to the directory of the repositories_list_file
            relative_path = os.path.relpath(
                file_path, os.path.dirname(interface_list_file)
            )
            fh.write(relative_path + "\n")


def get_files_to_be_generated(manifest_file: str) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    folder_path = manifest["repositories"]["folder_path"]
    interface_folder_path = manifest["repositories"]["interface_path"]

    # Get the list of files to be generated
    files_to_be_generated = []
    for entity in manifest["repositories"]["list"]:
        entity_name = entity["entity_name"]
        if entity.get("generate", True):
            files_to_be_generated.append(
                os.path.join(
                    folder_path, f"{stringcase.snakecase(entity_name)}_repository.h"
                )
            )
            files_to_be_generated.append(
                os.path.join(
                    folder_path, f"{stringcase.snakecase(entity_name)}_repository.cpp"
                )
            )
            files_to_be_generated.append(
                os.path.join(
                    interface_folder_path,
                    f"interface_{stringcase.snakecase(entity_name)}_repository.h",
                )
            )

    # add list_file:
    list_file = manifest["repositories"]["list_file"]
    files_to_be_generated.append(list_file)
    # add list_file:
    interface_list_file = manifest["repositories"]["interface_list_file"]
    files_to_be_generated.append(interface_list_file)

    return files_to_be_generated


# generate the files into the preview folder
def preview_repository_files(manifest_file: str):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["repositories"]["folder_path"] = "preview/" + manifest["repositories"][
        "folder_path"
    ].replace("..", "")
    manifest["repositories"]["list_file"] = "preview/" + manifest["repositories"][
        "list_file"
    ].replace("..", "")
    manifest["repositories"]["interface_path"] = "preview/" + manifest["repositories"][
        "interface_path"
    ].replace("..", "")
    manifest["repositories"]["interface_list_file"] = "preview/" + manifest[
        "repositories"
    ]["interface_list_file"].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    generate_repository_files(manifest_preview_file)


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
        preview_repository_files(file)
    else:
        # generate the files
        generate_repository_files(file)
