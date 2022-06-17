
## How the pipeline works

* A 'server in from this model' will actively look for incoming frames from vision.rs. From there on, the frames will pass through the feature extractor
* Feature extractor will try to filter out People and Everyday Objects. This features will later be utilized to check for similarity b/w the object
found in frame and the objects stored in the repository.

