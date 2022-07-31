extern "C" {
	void cv_PtrOfVariationalRefinement_delete(cv::Ptr<cv::VariationalRefinement>* instance) {
		delete instance;
	}

	const cv::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr(const cv::Ptr<cv::VariationalRefinement>* instance) {
		return instance->get();
	}

	cv::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr_mut(cv::Ptr<cv::VariationalRefinement>* instance) {
		return instance->get();
	}
}

