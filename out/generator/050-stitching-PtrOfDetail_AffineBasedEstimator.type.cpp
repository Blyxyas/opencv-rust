extern "C" {
	cv::Ptr<cv::detail::AffineBasedEstimator>* cv_PtrOfDetail_AffineBasedEstimator_new(cv::detail::AffineBasedEstimator* val) {
		return new cv::Ptr<cv::detail::AffineBasedEstimator>(val);
	}
	
	void cv_PtrOfDetail_AffineBasedEstimator_delete(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
		delete instance;
	}

	const cv::detail::AffineBasedEstimator* cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr(const cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
		return instance->get();
	}

	cv::detail::AffineBasedEstimator* cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr_mut(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_AffineBasedEstimator_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

