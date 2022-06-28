# Changelog
All notable changes to this project will be documented in this file.

[0.1.0.dev] - 2022-06-14
------------------------
- Fixed runtime issue caused by libtorch. Temporary solution for this issue is to change ownership of all the subdirectory
and files contained within `/var/system/openads`

- The future trajectory of this project has been changed to using single-device cameras for surveillance. This has been done in order to minimize
the CPU utilization and repurpose the threads into processing models much efficiently.

- Now ships with configuration file which lets the system know about mode-of-execution. Any model running in `Learning` mode will switch to
`Execution` mode after 7 days of its initial change. This setting can manually be overwritten by setting the defined string value in file located
in `/var/system/openads/config/execution/exec`

- [install.py](https://github.com/dpyte/rs-openads/blob/master/scripts/installation/install.py) can be used to copy required configuration files.
NOTE: Creation of `/system/openads` directory in `/var/` requires to be done manually. Additionally, user must recursively change the ownership to
$USER for `/var/system/openads`. Future release of this file will include this functionality.

#### Internal Changes
* Substituted Tokio channels with Crossbeams channels

#### Planned features and their progress
* Face detection - This pipeline is designed to utilize bits from phase-1 of the process. As of right now,
it is still in design process, making it difficult to give estimated development time. For additional information on Phase-1 please refer to (TODO:Add Reference).

