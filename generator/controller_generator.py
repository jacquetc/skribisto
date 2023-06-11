from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
from pathlib import Path


def generate_controller_files(
    manifest_file: str, files_to_be_generated: dict[str, bool] = None
):
    pass


def get_files_to_be_generated(
    manifest_file: str, files_to_be_generated: dict[str, bool] = None
) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    folder_path = manifest["controller"]["folder_path"]

    # Get the list of files to be generated
    files = []
    for controller in manifest["controller"]["list"]:
        controller_name = controller["name"]
        if controller.get("generate", True):
            files.append(
                os.path.join(
                    folder_path,
                    f"{stringcase.snakecase(controller_name)}_controller.h",
                )
            )
            files.append(
                os.path.join(
                    folder_path,
                    f"{stringcase.snakecase(controller_name)}_controller.cpp",
                )
            )

    # add list_file:
    list_file = manifest["controller"]["list_file"]
    files.append(list_file)

    # strip from files if the value in files_to_be_generated is False
    if files_to_be_generated:
        for path, generate in files_to_be_generated.items():
            if not generate:
                files.remove(path)

    return files


# generate the files into the preview folder
def preview_controller_files(
    manifest_file: str, files_to_be_generated: dict[str, bool] = None
):
    manifest_preview_file = "temp/manifest_preview.yaml"

    # make a copy of the manifest file into temp/manifest_preview.yaml
    shutil.copy(manifest_file, manifest_preview_file)

    # modify the manifest file to generate the files into the preview folder
    with open(manifest_preview_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    # remove .. from the path and add preview before the folder name
    manifest["controller"]["folder_path"] = "preview/" + manifest["controller"][
        "folder_path"
    ].replace("..", "")
    manifest["controller"]["list_file"] = "preview/" + manifest["controller"][
        "list_file"
    ].replace("..", "")

    # write the modified manifest file
    with open(manifest_preview_file, "w") as fh:
        yaml.dump(manifest, fh)

    if files_to_be_generated:
        # preprend preview/ to the file names in the dict files_to_be_generated and remove .. from the path

        for path, _ in files_to_be_generated.items():
            files_to_be_generated[path] = "preview/" + path.replace("..", "")

        generate_controller_files(manifest_preview_file, files_to_be_generated)

    else:
        generate_controller_files(manifest_preview_file)


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
        preview_controller_files(file)
    else:
        # generate the files
        generate_controller_files(file)
