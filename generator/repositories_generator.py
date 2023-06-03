from jinja2 import Environment, FileSystemLoader
import yaml
import os


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
    path = repositories_data.get("forder_path", ".")
    interface_path = repositories_data.get("interface_path", ".")
    repositories_list_file = repositories_data.get("list_file", "generated_files.txt")
    interface_list_file = repositories_data.get(
        "interface_list_file", "generated_interface_files.txt"
    )

    template_env = Environment(loader=FileSystemLoader("templates"))
    repo_template = template_env.get_template("repository_template.h.jinja2")
    repo_cpp_template = template_env.get_template("repository_template.cpp.jinja2")
    iface_template = template_env.get_template("interface_repository_template.h.jinja2")

    contracts_data = manifest_data.get("contracts", [])
    contracts_export = contracts_data.get("export", "")
    contracts_export_header_file = contracts_data.get("export_header_file", "")

    generated_files = []
    interface_generated_files = []

    for repository in repositories_list:
        name = repository["entity_name"]
        generate = repository.get("generate", True)

        if not generate:
            continue

        # Create .h file
        rendered_template = repo_template.render(
            name=name, export=export, export_header_file=export_header_file
        )
        output_file = os.path.join(path, f"{name.lower()}_repository.h")
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")
        generated_files.append(output_file)

        # Create .cpp file
        rendered_template = repo_cpp_template.render(name=name)
        output_file = os.path.join(path, f"{name.lower()}_repository.cpp")
        with open(output_file, "w") as fh:
            fh.write(rendered_template)
            print(f"Successfully wrote file {output_file}")
        generated_files.append(output_file)

        # Create interface .h file
        rendered_template = iface_template.render(
            name=name,
            contracts_export=contracts_export,
            contracts_export_header_file=contracts_export_header_file,
        )
        output_file = os.path.join(
            interface_path, f"interface_{name.lower()}_repository.h"
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


# Main execution
generate_repository_files("manifest.yaml")
