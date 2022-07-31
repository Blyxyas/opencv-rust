#include "ocvrs_common.hpp"
#include <opencv2/stitching.hpp>

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
		cv::Ptr<cv::AffineWarper>,
		cv::Ptr<cv::CompressedRectilinearPortraitWarper>,
		cv::Ptr<cv::CompressedRectilinearWarper>,
		cv::Ptr<cv::CylindricalWarper>,
		cv::Ptr<cv::FisheyeWarper>,
		cv::Ptr<cv::MercatorWarper>,
		cv::Ptr<cv::PaniniPortraitWarper>,
		cv::Ptr<cv::PaniniWarper>,
		cv::Ptr<cv::PlaneWarper>,
		cv::Ptr<cv::SphericalWarper>,
		cv::Ptr<cv::StereographicWarper>,
		cv::Ptr<cv::TransverseMercatorWarper>,
		cv::Ptr<cv::detail::FeatherBlender>,
		cv::Ptr<cv::detail::MultiBandBlender>,
		cv::Ptr<cv::detail::BundleAdjusterAffine>,
		cv::Ptr<cv::detail::BundleAdjusterAffinePartial>,
		cv::Ptr<cv::detail::BundleAdjusterRay>,
		cv::Ptr<cv::detail::BundleAdjusterReproj>,
		cv::Ptr<cv::detail::NoBundleAdjuster>,
		cv::Ptr<cv::detail::AffineBasedEstimator>,
		cv::Ptr<cv::detail::HomographyBasedEstimator>,
		cv::Ptr<cv::detail::BlocksCompensator>,
		cv::Ptr<cv::detail::ChannelsCompensator>,
		cv::Ptr<cv::detail::GainCompensator>,
		cv::Ptr<cv::detail::NoExposureCompensator>,
		cv::Ptr<cv::detail::BestOf2NearestMatcher>,
		cv::Ptr<cv::detail::DpSeamFinder>,
		cv::Ptr<cv::detail::GraphCutSeamFinder>,
		cv::Ptr<cv::detail::NoSeamFinder>,
		cv::Ptr<cv::detail::PairwiseSeamFinder>
	) {}
}