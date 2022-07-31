#include "ocvrs_common.hpp"
#include <opencv2/surface_matching.hpp>
#include "surface_matching_types.hpp"

extern "C" {
	void cv_ICP_delete(cv::ppf_match_3d::ICP* instance) {
		delete instance;
	}
	// ICP() /usr/include/opencv2/surface_matching/icp.hpp:90
	void cv_ppf_match_3d_ICP_ICP(Result<cv::ppf_match_3d::ICP*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::ICP*>))
	}
	
	// ICP(const int, const float, const float, const int, const int, const int) /usr/include/opencv2/surface_matching/icp.hpp:117
	void cv_ppf_match_3d_ICP_ICP_const_int_const_float_const_float_const_int_const_int_const_int(const int iterations, const float tolerence, const float rejectionScale, const int numLevels, const int sampleType, const int numMaxCorr, Result<cv::ppf_match_3d::ICP*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP(iterations, tolerence, rejectionScale, numLevels, sampleType, numMaxCorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::ICP*>))
	}
	
	// registerModelToScene(const cv::Mat &, const cv::Mat &, double &, cv::Matx44d &) /usr/include/opencv2/surface_matching/icp.hpp:139
	void cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_doubleR_Matx44dR(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, double* residual, cv::Matx44d* pose, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *residual, *pose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// registerModelToScene(const cv::Mat &, const cv::Mat &, std::vector<Pose3DPtr> &) /usr/include/opencv2/surface_matching/icp.hpp:152
	void cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vector_Pose3DPtr_R(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, std::vector<cv::ppf_match_3d::Pose3DPtr>* poses, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *poses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_PPF3DDetector_delete(cv::ppf_match_3d::PPF3DDetector* instance) {
		delete instance;
	}
	// PPF3DDetector() /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:104
	void cv_ppf_match_3d_PPF3DDetector_PPF3DDetector(Result<cv::ppf_match_3d::PPF3DDetector*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PPF3DDetector*>))
	}
	
	// PPF3DDetector(const double, const double, const double) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:112
	void cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double_const_double_const_double(const double relativeSamplingStep, const double relativeDistanceStep, const double numAngles, Result<cv::ppf_match_3d::PPF3DDetector*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector(relativeSamplingStep, relativeDistanceStep, numAngles);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PPF3DDetector*>))
	}
	
	// setSearchParams(const double, const double, const bool) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:122
	void cv_ppf_match_3d_PPF3DDetector_setSearchParams_const_double_const_double_const_bool(cv::ppf_match_3d::PPF3DDetector* instance, const double positionThreshold, const double rotationThreshold, const bool useWeightedClustering, Result_void* ocvrs_return) {
		try {
			instance->setSearchParams(positionThreshold, rotationThreshold, useWeightedClustering);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// trainModel(const cv::Mat &) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:131
	void cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatR(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* Model, Result_void* ocvrs_return) {
		try {
			instance->trainModel(*Model);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// match(const cv::Mat &, std::vector<Pose3DPtr> &, const double, const double) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:141
	void cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vector_Pose3DPtr_R_const_double_const_double(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* scene, std::vector<cv::ppf_match_3d::Pose3DPtr>* results, const double relativeSceneSampleStep, const double relativeSceneDistance, Result_void* ocvrs_return) {
		try {
			instance->match(*scene, *results, relativeSceneSampleStep, relativeSceneDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// alpha /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	double cv_ppf_match_3d_Pose3D_getPropAlpha_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->alpha;
			return ret;
	}
	
	// alpha /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	void cv_ppf_match_3d_Pose3D_setPropAlpha_double(cv::ppf_match_3d::Pose3D* instance, double val) {
			instance->alpha = val;
	}
	
	// residual /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	double cv_ppf_match_3d_Pose3D_getPropResidual_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->residual;
			return ret;
	}
	
	// residual /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	void cv_ppf_match_3d_Pose3D_setPropResidual_double(cv::ppf_match_3d::Pose3D* instance, double val) {
			instance->residual = val;
	}
	
	// modelIndex /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	size_t cv_ppf_match_3d_Pose3D_getPropModelIndex_const(const cv::ppf_match_3d::Pose3D* instance) {
			size_t ret = instance->modelIndex;
			return ret;
	}
	
	// modelIndex /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	void cv_ppf_match_3d_Pose3D_setPropModelIndex_size_t(cv::ppf_match_3d::Pose3D* instance, size_t val) {
			instance->modelIndex = val;
	}
	
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	size_t cv_ppf_match_3d_Pose3D_getPropNumVotes_const(const cv::ppf_match_3d::Pose3D* instance) {
			size_t ret = instance->numVotes;
			return ret;
	}
	
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	void cv_ppf_match_3d_Pose3D_setPropNumVotes_size_t(cv::ppf_match_3d::Pose3D* instance, size_t val) {
			instance->numVotes = val;
	}
	
	// pose /usr/include/opencv2/surface_matching/pose_3d.hpp:127
	void cv_ppf_match_3d_Pose3D_getPropPose_const(const cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* ocvrs_return) {
			cv::Matx44d ret = instance->pose;
			*ocvrs_return = ret;
	}
	
	// pose /usr/include/opencv2/surface_matching/pose_3d.hpp:127
	void cv_ppf_match_3d_Pose3D_setPropPose_Matx44d(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* val) {
			instance->pose = *val;
	}
	
	// angle /usr/include/opencv2/surface_matching/pose_3d.hpp:128
	double cv_ppf_match_3d_Pose3D_getPropAngle_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->angle;
			return ret;
	}
	
	// angle /usr/include/opencv2/surface_matching/pose_3d.hpp:128
	void cv_ppf_match_3d_Pose3D_setPropAngle_double(cv::ppf_match_3d::Pose3D* instance, double val) {
			instance->angle = val;
	}
	
	// t /usr/include/opencv2/surface_matching/pose_3d.hpp:129
	void cv_ppf_match_3d_Pose3D_getPropT_const(const cv::ppf_match_3d::Pose3D* instance, cv::Vec3d* ocvrs_return) {
			cv::Vec3d ret = instance->t;
			*ocvrs_return = ret;
	}
	
	// t /usr/include/opencv2/surface_matching/pose_3d.hpp:129
	void cv_ppf_match_3d_Pose3D_setPropT_Vec3d(cv::ppf_match_3d::Pose3D* instance, cv::Vec3d* val) {
			instance->t = *val;
	}
	
	// q /usr/include/opencv2/surface_matching/pose_3d.hpp:130
	void cv_ppf_match_3d_Pose3D_getPropQ_const(const cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* ocvrs_return) {
			cv::Vec4d ret = instance->q;
			*ocvrs_return = ret;
	}
	
	// q /usr/include/opencv2/surface_matching/pose_3d.hpp:130
	void cv_ppf_match_3d_Pose3D_setPropQ_Vec4d(cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* val) {
			instance->q = *val;
	}
	
	void cv_Pose3D_delete(cv::ppf_match_3d::Pose3D* instance) {
		delete instance;
	}
	// Pose3D() /usr/include/opencv2/surface_matching/pose_3d.hpp:73
	void cv_ppf_match_3d_Pose3D_Pose3D(Result<cv::ppf_match_3d::Pose3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3D*>))
	}
	
	// Pose3D(double, size_t, size_t) /usr/include/opencv2/surface_matching/pose_3d.hpp:83
	void cv_ppf_match_3d_Pose3D_Pose3D_double_size_t_size_t(double Alpha, size_t ModelIndex, size_t NumVotes, Result<cv::ppf_match_3d::Pose3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D(Alpha, ModelIndex, NumVotes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3D*>))
	}
	
	// updatePose(cv::Matx44d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:97
	void cv_ppf_match_3d_Pose3D_updatePose_Matx44dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* NewPose, Result_void* ocvrs_return) {
		try {
			instance->updatePose(*NewPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updatePose(cv::Matx33d &, cv::Vec3d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:102
	void cv_ppf_match_3d_Pose3D_updatePose_Matx33dR_Vec3dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx33d* NewR, cv::Vec3d* NewT, Result_void* ocvrs_return) {
		try {
			instance->updatePose(*NewR, *NewT);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updatePoseQuat(cv::Vec4d &, cv::Vec3d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:107
	void cv_ppf_match_3d_Pose3D_updatePoseQuat_Vec4dR_Vec3dR(cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* Q, cv::Vec3d* NewT, Result_void* ocvrs_return) {
		try {
			instance->updatePoseQuat(*Q, *NewT);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// appendPose(cv::Matx44d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:113
	void cv_ppf_match_3d_Pose3D_appendPose_Matx44dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* IncrementalPose, Result_void* ocvrs_return) {
		try {
			instance->appendPose(*IncrementalPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// printPose() /usr/include/opencv2/surface_matching/pose_3d.hpp:114
	void cv_ppf_match_3d_Pose3D_printPose(cv::ppf_match_3d::Pose3D* instance, Result_void* ocvrs_return) {
		try {
			instance->printPose();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clone() /usr/include/opencv2/surface_matching/pose_3d.hpp:116
	void cv_ppf_match_3d_Pose3D_clone(cv::ppf_match_3d::Pose3D* instance, Result<cv::ppf_match_3d::Pose3DPtr*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3DPtr ret = instance->clone();
			Ok(new cv::ppf_match_3d::Pose3DPtr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3DPtr*>))
	}
	
	// writePose(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:120
	void cv_ppf_match_3d_Pose3D_writePose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->writePose(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// readPose(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:121
	void cv_ppf_match_3d_Pose3D_readPose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->readPose(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// poseList /usr/include/opencv2/surface_matching/pose_3d.hpp:177
	std::vector<cv::ppf_match_3d::Pose3DPtr>* cv_ppf_match_3d_PoseCluster3D_getPropPoseList_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			std::vector<cv::ppf_match_3d::Pose3DPtr> ret = instance->poseList;
			return new std::vector<cv::ppf_match_3d::Pose3DPtr>(ret);
	}
	
	// poseList /usr/include/opencv2/surface_matching/pose_3d.hpp:177
	void cv_ppf_match_3d_PoseCluster3D_setPropPoseList_vector_Pose3DPtr_(cv::ppf_match_3d::PoseCluster3D* instance, std::vector<cv::ppf_match_3d::Pose3DPtr>* val) {
			instance->poseList = *val;
	}
	
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:178
	size_t cv_ppf_match_3d_PoseCluster3D_getPropNumVotes_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			size_t ret = instance->numVotes;
			return ret;
	}
	
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:178
	void cv_ppf_match_3d_PoseCluster3D_setPropNumVotes_size_t(cv::ppf_match_3d::PoseCluster3D* instance, size_t val) {
			instance->numVotes = val;
	}
	
	// id /usr/include/opencv2/surface_matching/pose_3d.hpp:179
	int cv_ppf_match_3d_PoseCluster3D_getPropId_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			int ret = instance->id;
			return ret;
	}
	
	// id /usr/include/opencv2/surface_matching/pose_3d.hpp:179
	void cv_ppf_match_3d_PoseCluster3D_setPropId_int(cv::ppf_match_3d::PoseCluster3D* instance, int val) {
			instance->id = val;
	}
	
	void cv_PoseCluster3D_delete(cv::ppf_match_3d::PoseCluster3D* instance) {
		delete instance;
	}
	// PoseCluster3D() /usr/include/opencv2/surface_matching/pose_3d.hpp:141
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D(Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	// PoseCluster3D(cv::ppf_match_3d::Pose3DPtr) /usr/include/opencv2/surface_matching/pose_3d.hpp:147
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(cv::ppf_match_3d::Pose3DPtr* newPose, Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	// PoseCluster3D(cv::ppf_match_3d::Pose3DPtr, int) /usr/include/opencv2/surface_matching/pose_3d.hpp:155
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(cv::ppf_match_3d::Pose3DPtr* newPose, int newId, Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose, newId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	// addPose(cv::ppf_match_3d::Pose3DPtr) /usr/include/opencv2/surface_matching/pose_3d.hpp:170
	void cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(cv::ppf_match_3d::PoseCluster3D* instance, cv::ppf_match_3d::Pose3DPtr* newPose, Result_void* ocvrs_return) {
		try {
			instance->addPose(*newPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writePoseCluster(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:174
	void cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->writePoseCluster(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// readPoseCluster(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:175
	void cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->readPoseCluster(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
}
