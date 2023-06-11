from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
from pathlib import Path


def get_generation_dict(
    real_imports_folder_path: str,
    mock_imports_folder_path: str,
    feature_by_name: dict,
    application_name: str,
) -> dict:
    generation_dict = {}
    generation_dict["controllers"] = {}
    for feature_name, feature_data in feature_by_name.items():
        feature_snake_name = stringcase.snakecase(feature_name)
        feature_pascal_name = stringcase.pascalcase(feature_name)
        feature_camel_name = stringcase.camelcase(feature_name)
        generation_dict["controllers"][feature_pascal_name] = {
            "mock_presenter_file": os.path.join(
                mock_imports_folder_path,
                "Presenter",
                f"{feature_pascal_name}Controller.qml",
            ),
            "mock_template_path": "QML/mock_imports/presenter/",
            "mock_template": "controller.qml.jinja2",
            "real_presenter_file": os.path.join(
                real_imports_folder_path,
                "presenter",
                f"foreign_{feature_snake_name}_controller.h",
            ),
            "real_template_path": "QML/real_imports/presenter/",
            "real_template": "foreign_controller.h.jinja2",
            "feature_pascal_name": feature_pascal_name,
            "feature_camel_name": feature_camel_name,
            "feature_snake_name": feature_snake_name,
            "CRUD": feature_data.get("CRUD", {}),
        }

    # add mock_custom_functions to the generation_dict["controllers"][feature_pascal_name] dict
    for feature_name, feature_data in feature_by_name.items():
        feature_pascal_name = stringcase.pascalcase(feature_name)
        generation_dict["controllers"][feature_pascal_name][
            "mock_custom_functions"
        ] = []
        commands = feature_data.get("commands", [])
        if commands:
            for command in commands:
                generation_dict["controllers"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(command["name"])]

        queries = feature_data.get("queries", [])
        if queries:
            for query in queries:
                generation_dict["controllers"][feature_pascal_name][
                    "mock_custom_functions"
                ] += [stringcase.camelcase(query["name"])]

    # add qmldir:
    qmldir_file = os.path.join(mock_imports_folder_path, "Presenter", "qmldir")

    generation_dict["qmldir_file"] = qmldir_file

    # add CMakelists.txt:
    common_cmake_file = os.path.join(
        real_imports_folder_path, "presenter", "CMakeLists.txt"
    )
    generation_dict["common_cmake_file"] = common_cmake_file

    # add "mock_presenter_file" and "real_presenter_file" to the generation_dict["real_presenter_files"] list
    generation_dict["real_presenter_files"] = []
    generation_dict["mock_presenter_files"] = []
    for _, controller in generation_dict["controllers"].items():
        generation_dict["real_presenter_files"].append(
            controller["real_presenter_file"]
        )
        generation_dict["mock_presenter_files"].append(
            controller["mock_presenter_file"]
        )

    # add application_name
    generation_dict["application_name"] = application_name

    return generation_dict


def generate_mock_controller_file(
    controller: dict, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock controller file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(controller["mock_presenter_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", controller["mock_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(controller["mock_template"])

    crud = controller["CRUD"]
    crud_enabled = crud.get("enabled", False)
    get_enabled = crud.get("get", {}).get("enabled", False)
    get_all_enabled = crud.get("get_all", {}).get("enabled", False)
    create_enabled = crud.get("create", {}).get("enabled", False)
    update_enabled = crud.get("update", {}).get("enabled", False)
    remove_enabled = crud.get("remove", {}).get("enabled", False)

    # Render the template
    output = template.render(
        controller=controller,
        crud=crud,
        crud_enabled=crud_enabled,
        get_enabled=get_enabled,
        get_all_enabled=get_all_enabled,
        create_enabled=create_enabled,
        update_enabled=update_enabled,
        remove_enabled=remove_enabled,
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(controller["mock_presenter_file"]), exist_ok=True)

    # Write the output to the file
    with open(controller["mock_presenter_file"], "w") as fh:
        fh.write(output)


def generate_mock_qmldir_file(
    generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the mock qmldir file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(generation_dict["qmldir_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/mock_imports/presenter/"))
    # Load the template
    template = env.get_template("qmldir_template.jinja2")

    singleton_list = []
    for _, controller in generation_dict["controllers"].items():
        name = controller["feature_pascal_name"]
        singleton_list.append(f"singleton {name}Controller 1.0 {name}Controller.qml")

    # Render the template
    output = template.render(singleton_list=singleton_list)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(generation_dict["qmldir_file"]), exist_ok=True)

    # Write the output to the file
    with open(generation_dict["qmldir_file"], "w") as fh:
        fh.write(output)


def generate_real_controller_file(
    controller: dict, generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the real controller file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(controller["real_presenter_file"], False):
        return

    # Create the jinja2 environment
    template_path = os.path.join("templates", controller["real_template_path"])
    env = Environment(loader=FileSystemLoader(template_path))
    # Load the template
    template = env.get_template(controller["real_template"])

    # Render the template
    output = template.render(controller=controller)

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(controller["real_presenter_file"]), exist_ok=True)

    # Write the output to the file
    with open(controller["real_presenter_file"], "w") as fh:
        fh.write(output)


def generate_real_cmakelists_file(
    generation_dict: dict, files_to_be_generated: dict[str, bool]
):
    # generate the real cmakelists file if in the files_to_be_generated dict the value is True
    if files_to_be_generated.get(generation_dict["common_cmake_file"], False):
        return

    # Create the jinja2 environment
    env = Environment(loader=FileSystemLoader("templates/QML/real_imports/presenter/"))
    # Load the template
    template = env.get_template("cmakelists_template.jinja2")

    files = generation_dict["real_presenter_files"]
    relative_files = []
    for file in files:
        relative_files.append(
            os.path.relpath(file, generation_dict["common_cmake_file"])
        )

    # Render the template
    output = template.render(
        files=relative_files,
        application_name=generation_dict["application_name"],
    )

    # Create the directory if it does not exist
    os.makedirs(os.path.dirname(generation_dict["common_cmake_file"]), exist_ok=True)

    # Write the output to the file
    with open(generation_dict["common_cmake_file"], "w") as fh:
        fh.write(output)


def generate_qml_files(manifest_file: str, files_to_be_generated: dict[str, bool] = {}):
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

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    qml_data = manifest_data.get("qml", [])

    generation_dict = get_generation_dict(
        qml_data["real_imports_folder_path"],
        qml_data["mock_imports_folder_path"],
        feature_by_name,
        application_name,
    )

    # generate mock files
    for _, controller in generation_dict["controllers"].items():
        generate_mock_controller_file(
            controller, generation_dict, files_to_be_generated
        )

    # generate mock qmldir file
    generate_mock_qmldir_file(generation_dict, files_to_be_generated)

    # generate real files
    for _, controller in generation_dict["controllers"].items():
        generate_real_controller_file(
            controller, generation_dict, files_to_be_generated
        )

    # generate real CMakeLists.txt file
    generate_real_cmakelists_file(generation_dict, files_to_be_generated)


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

    # Organize feature_list by name for easier lookup
    feature_by_name = {feature["name"]: feature for feature in feature_list}

    qml_data = manifest_data.get("qml", [])

    files = []
    generation_dict = get_generation_dict(
        qml_data["real_imports_folder_path"],
        qml_data["mock_imports_folder_path"],
        feature_by_name,
        application_name,
    )
    files += generation_dict["real_presenter_files"]
    files += generation_dict["mock_presenter_files"]

    # for _, controller in generation_dict["controllers"].items():
    #     files += feature["real_presenter_files"]

    # # add CMakelists.txt:
    common_cmake_file = generation_dict["common_cmake_file"]
    files.append(common_cmake_file)

    # # add qmldir:
    qmldir_file = generation_dict["qmldir_file"]
    files.append(qmldir_file)

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
    manifest["qml"]["mock_imports_folder_path"] = "preview/" + manifest["qml"][
        "mock_imports_folder_path"
    ].replace("..", "")
    manifest["qml"]["real_imports_folder_path"] = "preview/" + manifest["qml"][
        "real_imports_folder_path"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    # preprend preview/ to the file names in the dict files_to_be_generated and remove .. from the path
    if files_to_be_generated:
        for path, _ in files_to_be_generated.items():
            files_to_be_generated[path] = "preview/" + path.replace("..", "")

        generate_qml_files(manifest_preview_file, files_to_be_generated)

    else:
        generate_qml_files(manifest_preview_file)


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
        generate_qml_files(file)
