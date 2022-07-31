#include "ocvrs_common.hpp"
#include <opencv2/videostab.hpp>

/* Dummy namespace that contains types that will be used in the bindings generator to resolve clang::Type from
 * string. Due to limitations of libclang it's not possible to resolve arbitrary strings. That's why we pre-add
 * typedefs that will be analyzed in the initial parsing step and added to the global cache that can resolve
 * e.g. "void" to a specific clang::Type.
 *
 * Only typedef's are analyzed in this namespace and each entry should alias distinct type, the name of the
 * typedef is ignored by the parser.
 */
namespace ocvrs_resolve_types {
	typedef void ephem0;
	typedef bool ephem1;
	typedef int ephem2;
	typedef unsigned int ephem3;
	typedef double ephem4;
	typedef const char* ephem5;
	typedef void* ephem6;
	typedef std::vector<cv::String> ephem7;
	typedef std::vector<std::string> ephem8;
	typedef cv::_InputArray ephem9;
	typedef cv::_OutputArray ephem10;
	typedef cv::_InputOutputArray ephem11;
}

/* This is used to generate types otherwise not referenced in the OpenCV headers. It's useful for example to
 * generate Ptr<Child> when OpenCV function only accepts Ptr<Parent>.
 */
namespace cv {
	OCVRS_ONLY_DEPENDENT_TYPES void dependent_types_dummy(
		cv::Ptr<cv::videostab::NullDeblurer>,
		cv::Ptr<cv::videostab::WeightingDeblurer>,
		cv::Ptr<cv::videostab::MaskFrameSource>,
		cv::Ptr<cv::videostab::NullFrameSource>,
		cv::Ptr<cv::videostab::OnePassStabilizer>,
		cv::Ptr<cv::videostab::TwoPassStabilizer>,
		cv::Ptr<cv::videostab::VideoFileSource>,
		cv::Ptr<cv::videostab::LogToStdout>,
		cv::Ptr<cv::videostab::NullLog>,
		cv::Ptr<cv::videostab::LpMotionStabilizer>,
		cv::Ptr<cv::videostab::MotionStabilizationPipeline>,
		cv::Ptr<cv::videostab::NullOutlierRejector>,
		cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>,
		cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>,
		cv::Ptr<cv::videostab::FromFileMotionReader>,
		cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>,
		cv::Ptr<cv::videostab::ToFileMotionWriter>,
		cv::Ptr<cv::videostab::ColorAverageInpainter>,
		cv::Ptr<cv::videostab::ColorInpainter>,
		cv::Ptr<cv::videostab::ConsistentMosaicInpainter>,
		cv::Ptr<cv::videostab::InpaintingPipeline>,
		cv::Ptr<cv::videostab::MotionInpainter>,
		cv::Ptr<cv::videostab::NullInpainter>,
		cv::Ptr<cv::videostab::MotionEstimatorL1>,
		cv::Ptr<cv::videostab::MotionEstimatorRansacL2>,
		cv::Ptr<cv::videostab::GaussianMotionFilter>,
		cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>,
		cv::Ptr<cv::videostab::NullWobbleSuppressor>
	) {}
}