import os
import sys

import tomli
from packaging import version


if __name__ == '__main__':
    toml_file_path = sys.argv[1]

    with open(toml_file_path, 'rb') as f:
        project = tomli.load(f)

    print(project)

    name = project.get('package', {}).get('name', None)
    project_version = version.parse(project.get('package', {}).get('version', None))
    description = project.get('package', {}).get('description', None)
    
    with open(os.environ['GITHUB_OUTPUT'], 'at') as f:
        f.write(f'local-version={str(project_version)}\n')
        f.write(f'package-name={name}\n')
        f.write(f'package-description={description}\n')
