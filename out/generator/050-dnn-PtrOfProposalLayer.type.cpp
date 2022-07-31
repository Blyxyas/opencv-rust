extern "C" {
	cv::Ptr<cv::dnn::ProposalLayer>* cv_PtrOfProposalLayer_new(cv::dnn::ProposalLayer* val) {
		return new cv::Ptr<cv::dnn::ProposalLayer>(val);
	}
	
	void cv_PtrOfProposalLayer_delete(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ProposalLayer* cv_PtrOfProposalLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ProposalLayer* cv_PtrOfProposalLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
		return instance->get();
	}
}

