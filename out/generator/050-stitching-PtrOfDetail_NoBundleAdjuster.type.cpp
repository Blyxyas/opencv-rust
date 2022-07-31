extern "C" {
	cv::Ptr<cv::detail::NoBundleAdjuster>* cv_PtrOfDetail_NoBundleAdjuster_new(cv::detail::NoBundleAdjuster* val) {
		return new cv::Ptr<cv::detail::NoBundleAdjuster>(val);
	}
	
	void cv_PtrOfDetail_NoBundleAdjuster_delete(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		delete instance;
	}

	const cv::detail::NoBundleAdjuster* cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr(const cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return instance->get();
	}

	cv::detail::NoBundleAdjuster* cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

