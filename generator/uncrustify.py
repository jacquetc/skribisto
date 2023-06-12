import subprocess

import subprocess


def run_uncrustify(file_path: str, config_path: str):
    try:
        # Run uncrustify on the file
        result = subprocess.run(
            [
                "uncrustify",
                "-c",
                config_path,
                "-f",
                file_path,
                "-o",
                file_path,
                "--no-backup",
            ],
            check=True,
        )

        # If the return code is 0, then uncrustify ran successfully
        if result.returncode == 0:
            return
            # print(f"Uncrustify ran successfully on {file_path}.")
        else:
            print(f"Uncrustify failed on {file_path}.")

    except Exception as e:
        print(f"Error running uncrustify: {e}")
