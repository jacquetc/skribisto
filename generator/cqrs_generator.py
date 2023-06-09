from jinja2 import Environment, FileSystemLoader
import yaml
import os
import sys
import stringcase
import shutil
from pathlib import Path


def generate_cqrs_files(manifest_file: str):
    pass


def get_files_to_be_generated(manifest_file: str) -> list[str]:
    """
    Get the list of files that need to be generated based on the manifest file
    """
    # Read the manifest file
    with open(manifest_file, "r") as fh:
        manifest = yaml.safe_load(fh)

    folder_path = manifest["entities"]["folder_path"]

    # Get the list of files to be generated
    files_to_be_generated = []
    for entity in manifest["entities"]["list"]:
        entity_name = entity["name"]
        if entity.get("generate", True):
            files_to_be_generated.append(
                os.path.join(folder_path, f"{stringcase.snakecase(entity_name)}.h")
            )

    # add list_file:
    list_file = manifest["entities"]["list_file"]
    files_to_be_generated.append(list_file)

    return files_to_be_generated


# generate the files into the preview folder
def preview_cqrs_files(manifest_file: str):
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

    generate_cqrs_files(manifest_preview_file)


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
