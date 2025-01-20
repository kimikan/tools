#include <Eigen/Core>
#include <Eigen/Dense>
#include <chrono>
#include <iostream>

enum LanefitType {
    Poly = 0,
    Arc = 1,
};

class LaneLineFitCurve {
   public:
    bool fit_success = false;
    double err = 0.0;
    LanefitType type;
    // @brief min value of x
    double x_min;
    // @brief max value of x
    double x_max;
    double c0;
    double c1;
    double c2;
    double c3;
};
auto now() {
    auto start = std::chrono::steady_clock::now();
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
               start.time_since_epoch())
        .count();
}

bool polyfit3(const Eigen::VectorXd &x, const Eigen::VectorXd &y, double *err,
              Eigen::Vector4d *coef, [[maybe_unused]] double eps, bool is_xz,
              bool limited_c3, const std::vector<int> &high_weight_idx,
              const double ref_x = HUGE_VAL,
              const LaneLineFitCurve &ref_line = LaneLineFitCurve()) {
    Eigen::VectorXd w = Eigen::VectorXd::Ones(x.size());
    Eigen::ArrayXd ax = x.array(), ay = y.array(), aw = w.array();
    double mx = x.cwiseAbs().maxCoeff(), my = y.cwiseAbs().maxCoeff();
    int point_num = x.rows();
    if (mx == 0 || my == 0 || point_num < 3) {
        return false;
    }

    Eigen::Matrix4d lambda;
    Eigen::Vector4d beta;
    lambda.setZero();
    beta.setZero();

    double s0 = my;
    double s1 = s0 / mx;
    double s2 = s1 / mx;
    double s3 = s2 / mx;
    double prior_weight = 10000;

    if (ref_x < HUGE_VAL) {
        double x1_ = ref_x;
        double x2_ = x1_ * x1_;
        double x3_ = x1_ * x2_;

        double r0 = ref_line.c3 * x3_ + ref_line.c2 * x2_ + ref_line.c1 * x1_ +
                    ref_line.c0;
        double r1 = 3 * ref_line.c3 * x2_ + 2 * ref_line.c2 * x1_ + ref_line.c1;
        // double r2 = 6 * ref_line.c3 * x1_ + 2 * ref_line.c2;

        Eigen::Matrix<double, 1, 4> J;
        J << s3 * x3_, s2 * x2_, s1 * x1_, s0 * 1.0;
        lambda += prior_weight * J.transpose() * J;
        beta += prior_weight * J.transpose() * r0;

        J << s3 * 3 * x2_, s2 * 2 * x1_, s1 * 1.0, 0.0;
        lambda += prior_weight * J.transpose() * J;
        beta += prior_weight * J.transpose() * r1;

        // J << s3 * 6 * x1_, s2 * 2.0, 0.0, 0.0;
        // lambda += prior_weight * J.transpose() * J;
        // beta += prior_weight * J.transpose() * r2;
    }

    if (limited_c3) {
        lambda(0, 0) += 10;
        if (is_xz) {
            lambda(1, 1) += 10;
        }
    }

    Eigen::Matrix4d &A = lambda;
    Eigen::Vector4d &b = beta;
    size_t k = 0;
    for (int i = 0; i < point_num; ++i) {
        double w = 1.0;
        if (k < high_weight_idx.size() && i == high_weight_idx[k]) {
            if (k == 0 || k == high_weight_idx.size() - 1 ||
                std::fabs(x[high_weight_idx[k]]) <= 15.0) {
                w = 3.0;
            }
            ++k;
        }

        double _x = ax[i] / mx;
        double x0 = w * w;
        double x1 = x0 * _x;
        double x2 = x1 * _x;
        double x3 = x2 * _x;
        double x4 = x3 * _x;
        double x5 = x4 * _x;
        double x6 = x5 * _x;
        A(0, 0) += x6;
        A(0, 1) += x5;
        A(0, 2) += x4;
        A(0, 3) += x3;
        A(1, 1) += x4;
        A(1, 2) += x3;
        A(1, 3) += x2;
        A(2, 2) += x2;
        A(2, 3) += x1;
        A(3, 3) += x0;
        double _y = ay[i] / my;
        b(0) += x3 * _y;
        b(1) += x2 * _y;
        b(2) += x1 * _y;
        b(3) += x0 * _y;
    }

    A(1, 0) = A(0, 1);
    A(2, 0) = A(0, 2);
    A(2, 1) = A(1, 2);
    A(3, 0) = A(0, 3);
    A(3, 1) = A(1, 3);
    A(3, 2) = A(2, 3);

    if (A.determinant() == 0) {
        return false;
    }
    Eigen::Vector4d t_res = A.inverse() * b * my;
    (*coef)(3) = t_res(0) / pow(mx, 3);
    (*coef)(2) = t_res(1) / pow(mx, 2);
    (*coef)(1) = t_res(2) / mx;
    (*coef)(0) = t_res(3);
    *err = (((((*coef)(3) * ax + (*coef)(2)) * ax + (*coef)(1)) * ax +
             (*coef)(0) - ay)
                .pow(2) *
            aw)
               .sum();

    // std::cout<<" "<<start2 - start<<" "<<start3-start2<<" "<<start4-start3<<"
    // "<<start5-start4<<" "<<start6-start5<<" "<<start7-start6<<" "<<std::endl;
    // std::cout<<"xxxxxxxxxxxx:"<<start7-start<<" "<<std::endl;

    return true;
}

bool polyfit3_op(Eigen::VectorXd &w, double mx, const Eigen::VectorXd &x,
                 const Eigen::VectorXd &y, double *err, Eigen::Vector4d *coef,
                 [[maybe_unused]] double eps, bool is_xz, bool limited_c3,
                 double *sums, Eigen::ArrayXd *xxs,
                 const double ref_x = HUGE_VAL,
                 const LaneLineFitCurve &ref_line = LaneLineFitCurve()) {
    Eigen::ArrayXd ax = x.array(), ay = y.array(), aw = w.array();
    double my = y.cwiseAbs().maxCoeff();
    int point_num = x.rows();
    if (mx == 0 || my == 0 || point_num < 3) {
        return false;
    }

    Eigen::Matrix4d lambda = Eigen::Matrix4d::Zero();
    Eigen::Vector4d beta = Eigen::Vector4d::Zero();

    double s0 = my;
    double s1 = s0 / mx;
    double s2 = s1 / mx;
    double s3 = s2 / mx;
    double prior_weight = 10000;

    if (ref_x < HUGE_VAL) {
        double x1_ = ref_x;
        double x2_ = x1_ * x1_;
        double x3_ = x1_ * x2_;

        double r0 = ref_line.c3 * x3_ + ref_line.c2 * x2_ + ref_line.c1 * x1_ +
                    ref_line.c0;
        double r1 = 3 * ref_line.c3 * x2_ + 2 * ref_line.c2 * x1_ + ref_line.c1;

        Eigen::Matrix<double, 1, 4> J(s3 * x3_, s2 * x2_, s1 * x1_, s0 * 1.0);
        auto tmp = prior_weight * J.transpose();
        lambda += tmp * J;
        beta += tmp * r0;

        J << s3 * 3 * x2_, s2 * 2 * x1_, s1 * 1.0, 0.0;
        auto tmp2 = prior_weight * J.transpose();
        lambda += tmp2 * J;
        beta += tmp2 * r1;
    }

    if (limited_c3) {
        lambda(0, 0) += 10;
        if (is_xz) {
            lambda(1, 1) += 10;
        }
    }
    Eigen::Matrix4d &A = lambda;
    Eigen::Vector4d &b = beta;
    size_t k = 0;

    // 构造矩阵批量更新
    A(0, 0) += sums[6];
    A(0, 1) += sums[5];
    A(0, 2) += sums[4];
    A(0, 3) += sums[3];
    A(1, 1) += sums[4];
    A(1, 2) += sums[3];
    A(1, 3) += sums[2];
    A(2, 2) += sums[2];
    A(2, 3) += sums[1];
    A(3, 3) += sums[0];
    Eigen::ArrayXd norm_y = ay / my;
    b(0) += (xxs[3] * norm_y).sum();
    b(1) += (xxs[2] * norm_y).sum();
    b(2) += (xxs[1] * norm_y).sum();
    b(3) += (xxs[0] * norm_y).sum();

    A(1, 0) = A(0, 1);
    A(2, 0) = A(0, 2);
    A(2, 1) = A(1, 2);
    A(3, 0) = A(0, 3);
    A(3, 1) = A(1, 3);
    A(3, 2) = A(2, 3);

    if (A.determinant() == 0) {
        return false;
    }
    Eigen::Vector4d t_res = A.inverse() * b * my;
    (*coef)(3) = t_res(0) / pow(mx, 3);
    (*coef)(2) = t_res(1) / pow(mx, 2);
    (*coef)(1) = t_res(2) / mx;
    (*coef)(0) = t_res(3);
    *err = (((((*coef)(3) * ax + (*coef)(2)) * ax + (*coef)(1)) * ax +
             (*coef)(0) - ay)
                .pow(2) *
            aw)
               .sum();

    // std::cout<<" "<<start2 - start<<" "<<start3-start2<<" "<<start4-start3<<"
    // "<<start5-start4<<" "<<start6-start5<<" "<<start7-start6<<" "<<std::endl;
    // std::cout<<"xxxxxxxxxxxx:"<<start7-start<<" "<<std::endl;

    return true;
}

const int rounds = 500;
void test_op() {
    // 输入数据点
    std::vector<double> x_data = {0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5,
                                  0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5,
                                  0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5};
    std::vector<double> y_data = {0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25,
                                  0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25,
                                  0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25};
    std::vector<double> z_data = {0, 1, 4, 9, 16,  25, 0, 1, 4, 90, 16, 25,
                                  0, 1, 4, 9, 116, 25, 0, 1, 4, 9,  16, 25,
                                  0, 1, 4, 9, 16,  25, 0, 1, 4, 9,  16, 215};

    // 转换为 Eigen::VectorXd
    Eigen::VectorXd x =
        Eigen::Map<Eigen::VectorXd>(x_data.data(), x_data.size());
    Eigen::VectorXd y =
        Eigen::Map<Eigen::VectorXd>(y_data.data(), y_data.size());
    Eigen::VectorXd z =
        Eigen::Map<Eigen::VectorXd>(z_data.data(), y_data.size());

    // 存储结果
    Eigen::Vector4d coef;
    double error;

    // 参考线与高权重索引
    LaneLineFitCurve ref_line;
    ref_line.c3 = 0.1;
    ref_line.c2 = 0.2;
    ref_line.c1 = 0.3;
    ref_line.c0 = 0.4;

    LaneLineFitCurve ref_line2;
    ref_line2.c3 = 0.1;
    ref_line2.c2 = 0.2;
    ref_line2.c1 = 0.3;
    ref_line2.c0 = 0.4;

    double ref_x = 2.5;
    std::vector<int> high_weight_idx = {2, 4};  // 假设第 2 和第 4 个点权重较高
    // 调用 polyfit3
    bool success;
    auto start1 = now();
    for (int i = 0; i < rounds; ++i) {
        Eigen::VectorXd w = Eigen::VectorXd::Ones(x.size());
        double mx = x.cwiseAbs().maxCoeff();
        auto point_num = x.rows();
        Eigen::VectorXd weights = Eigen::VectorXd::Ones(point_num);
        for (int i = 0; i < high_weight_idx.size(); ++i) {
            if (high_weight_idx[i] < point_num &&
                (i == 0 || i == high_weight_idx.size() - 1 ||
                 std::fabs(x[high_weight_idx[i]]) <= 15.0)) {
                weights[high_weight_idx[i]] = 3.0;
            }
        }

        Eigen::ArrayXd norm_x = x.array() / mx;
        Eigen::ArrayXd x0 = weights.array().square();
        Eigen::ArrayXd x1 = x0 * norm_x;
        Eigen::ArrayXd x2 = x1 * norm_x;
        Eigen::ArrayXd x3 = x2 * norm_x;
        Eigen::ArrayXd x4 = x3 * norm_x;
        Eigen::ArrayXd x5 = x4 * norm_x;
        Eigen::ArrayXd x6 = x5 * norm_x;

        Eigen::ArrayXd xxs[4] = {x0, x1, x2, x3};
        double sums[7];
        sums[6] = x6.sum();
        sums[5] = x5.sum();
        sums[4] = x4.sum();
        sums[3] = x3.sum();
        sums[2] = x2.sum();
        sums[1] = x1.sum();
        sums[0] = x0.sum();

        success = polyfit3_op(w, mx, x, y, &error, &coef, 0, false, false, sums,
                              xxs, 2.5, ref_line);
        success = polyfit3_op(w, mx, x, z, &error, &coef, 0, true, true, sums,
                              xxs, 2.5, ref_line2);
    }
    auto end1 = now() - start1;
    std::cout << "优化后基于相同输入30个点测试500次消耗时间为: "
              << end1 / rounds << std::endl;

    // 输出结果
    if (success) {
        std::cout << "Fit successful!" << std::endl;
        std::cout << "Coefficients: " << coef.transpose() << std::endl;
        std::cout << "Error: " << error << std::endl;
    } else {
        std::cout << "Fit failed." << std::endl;
    }
}

void test_normal() {
    // 输入数据点
    std::vector<double> x_data = {0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5,
                                  0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5,
                                  0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5};
    std::vector<double> y_data = {0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25,
                                  0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25,
                                  0, 1, 4, 9, 16, 25, 0, 1, 4, 9, 16, 25};
    std::vector<double> z_data = {0, 1, 4, 9, 16,  25, 0, 1, 4, 90, 16, 25,
                                  0, 1, 4, 9, 116, 25, 0, 1, 4, 9,  16, 25,
                                  0, 1, 4, 9, 16,  25, 0, 1, 4, 9,  16, 215};

    // 转换为 Eigen::VectorXd
    Eigen::VectorXd x =
        Eigen::Map<Eigen::VectorXd>(x_data.data(), x_data.size());
    Eigen::VectorXd y =
        Eigen::Map<Eigen::VectorXd>(y_data.data(), y_data.size());
    Eigen::VectorXd z =
        Eigen::Map<Eigen::VectorXd>(z_data.data(), y_data.size());

    // 存储结果
    Eigen::Vector4d coef;
    double error;

    // 参考线与高权重索引
    LaneLineFitCurve ref_line;
    ref_line.c3 = 0.1;
    ref_line.c2 = 0.2;
    ref_line.c1 = 0.3;
    ref_line.c0 = 0.4;

    LaneLineFitCurve ref_line2;
    ref_line2.c3 = 0.1;
    ref_line2.c2 = 0.2;
    ref_line2.c1 = 0.3;
    ref_line2.c0 = 0.4;

    std::vector<int> high_weight_idx = {2, 4};  // 假设第 2 和第 4 个点权重较高
    // 调用 polyfit3
    bool success;
    auto start1 = now();
    for (int i = 0; i < rounds; ++i) {
        success = polyfit3(x, y, &error, &coef, 0, false, false,
                           high_weight_idx, 2.5, ref_line);
        success = polyfit3(x, z, &error, &coef, 0, true, true, high_weight_idx,
                           2.5, ref_line2);
    }
    auto end1 = now() - start1;
    std::cout << "未优化前基于30个点测试500次消耗时间为: " << end1 / rounds
              << std::endl;

    // 输出结果
    if (success) {
        std::cout << "Fit successful!" << std::endl;
        std::cout << "Coefficients: " << coef.transpose() << std::endl;
        std::cout << "Error: " << error << std::endl;
    } else {
        std::cout << "Fit failed." << std::endl;
    }
}

int main() {
    test_normal();
    test_op();
}
