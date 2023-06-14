import subprocess

import subprocess


def run_clang_format(file_path: str):
    try:
        # Run uncrustify on the file
        result = subprocess.run(
            [
                "clang-format",
                "-i",
                "--style=file",
                file_path,
            ],
            check=True,
        )

        # If the return code is 0, then clang-format ran successfully
        if result.returncode == 0:
            return
            print(f"clang-format ran successfully on {file_path}.")
        else:
            print(f"clang-format failed on {file_path}.")

    except Exception as e:
        print(f"Error running clang-format: {e}")
