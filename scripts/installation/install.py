#!bin/env python3

import os
import shutil

from pathlib import Path

path_to_info_xml = '../../configuration/camera/info.xml'
path_to_log_config = '../../crates/oads_log/config/log_config.yaml'


# Configure and crate directory for informational data
def check_info(config_dir):
    cpath = Path(config_dir)
    cpath.parent.mkdir(parents=True, exist_ok=True)

    config_src = os.path.abspath(path_to_info_xml)
    shutil.copyfile(config_src, os.path.join(config_dir, 'info.xml'))


def check_log(log_dir):
    lpath = Path(log_dir)
    lpath.parent.mkdir(parents=True, exist_ok=True)

    log_src = os.path.abspath(path_to_log_config)
    shutil.copyfile(log_src, os.path.join(log_dir, 'log_config.yaml'))


def check_for_openads():
    root_dir = '/var/system/openads'
    config_dir = os.path.join(root_dir, 'config', 'camera')
    log_dir = os.path.join(root_dir, 'config', 'log')
    storage_dir = os.path.join(root_dir, 'storage')

    rpath = Path(root_dir)
    rpath.parent.mkdir(parents=True, exist_ok=True)

    check_info(config_dir)
    check_log(log_dir)

    spath = Path(storage_dir)
    spath.parent.mkdir(parents=True, exist_ok=True)


def main():
    check_for_openads()
    pass


if __name__ == '__main__':
    main()
