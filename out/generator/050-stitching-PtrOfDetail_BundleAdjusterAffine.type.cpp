extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterAffine>* cv_PtrOfDetail_BundleAdjusterAffine_new(cv::detail::BundleAdjusterAffine* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterAffine>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterAffine_delete(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterAffine* cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterAffine* cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

