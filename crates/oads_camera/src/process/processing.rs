
// TODO: Move this function to oads_models-Hawk
///
/// Detect any facial presence in the frame using Hawk as backend
///
/// Hawk is primary algorithm responsible for detecting objects. Instead of using a NN, Hawk relies
/// on set of image analysis techniques. When the system is set on `Learning` mode, Hawk captures the background
/// data and saves that in its db. During execution, it pulls this cleaned background image and masks
/// itself with the current frame. TODO: Validate this thesis. If true, make the necessary
/// grammatical changes. The masked image should be a difference frame containing foreign objects.
/// This images will further be passed to next step where the algorithm will return bounded
/// coordinates for detected objects
///
/// Hawk is going to return a vector of coordinates instances every detected instances
/// (unsure) After obtaining the coordinates, the function will rank them based on how close
/// these objects are to each other, generating a combination list of these objects.
///
/// Assume these are coordinates of this objects
///   Obj ID    (Coordinates {x1;y1, x2:y2})
/// - Object 1: (1:10, 4:30)
/// - Object 2: (2:14, 4:12)
/// - Object 3: (1: 53, 7:53)
///

#[inline]
pub fn scan_facial_presence() {
}
