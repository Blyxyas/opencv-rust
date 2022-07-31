extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* cv_PtrOfDetail_BundleAdjusterAffinePartial_new(cv::detail::BundleAdjusterAffinePartial* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterAffinePartial>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterAffinePartial_delete(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterAffinePartial* cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterAffinePartial* cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

