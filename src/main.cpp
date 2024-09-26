#include <opencv2/opencv.hpp>
#include <iostream>
#include <math.h>

#include "util.h"

using std::vector;
typedef vector<cv::Point> Contour;
typedef vector<cv::Point2f> Contour2f;

using std::cout, std::endl;

cv::Mat sourceImage;
int threshold = 159;

// The width and height of your paper target. These can be in any unit, but the units must be the same.
double targetWidth = 11;
double targetHeight = 8.5;

vector<cv::Point2f> findCornersOfTarget(vector<Contour> contours) {
    vector<Contour> quadrilaterals;

    // Find all the contours with four corners
    for (auto contour : contours) {
        Contour simplifiedContour;
        double epsilon = 0.02 * cv::arcLength(contour, true);
        cv::approxPolyDP(contour, simplifiedContour, epsilon, true); // Simplify the countour

        if (simplifiedContour.size() == 4) {
            quadrilaterals.push_back(simplifiedContour);
        }
    }

    // Find the contour with the biggest area
    Contour biggestContour;
    double areaOfBiggestContour = 0;
    for (auto contour : quadrilaterals) {
        double area = cv::contourArea(contour);
        if (area > areaOfBiggestContour) {
            areaOfBiggestContour = area;
            biggestContour = contour;
        }
    }

    Contour2f out;
    for (auto point : biggestContour) {
        out.push_back(point);
    }

    return out;
}

cv::Mat transformImage(cv::Mat image, Contour2f corners) {
    double aspectRatio = targetWidth / targetHeight;
    std::cout << "ASPECT RATIO" << aspectRatio << std::endl;

    vector<cv::Point2f> destinationPoints;
    destinationPoints.push_back(cv::Point2f(0, 0));
    destinationPoints.push_back(cv::Point2f(1500, 0));
    destinationPoints.push_back(cv::Point2f(1500, 1500 / aspectRatio));
    destinationPoints.push_back(cv::Point2f(0, 1500 / aspectRatio));

    // Make sure the corners are sorted in the right spatial order. We do this by
    // computing the "average location" of all the points and seeing which quadrant
    // each point lies in relative to that center.
    Contour2f sortedCorners(4);
    cv::Point2f center;
    float sumx = 0, sumy = 0;
    for (cv::Point2f point : corners) {
        sumx += point.x;
        sumy += point.y;
    }
    center.x = sumx / corners.size();
    center.y = sumy / corners.size();
    for (cv::Point2f point : corners) {
        if (point.x <= center.x && point.y <= center.y) {        // Top left quadrant
            sortedCorners[0] = point;
        } else if (point.x >= center.x && point.y <= center.y) { // Top right quadrant
            sortedCorners[1] = point;
        } else if (point.x >= center.x && point.y >= center.y) { // Bottom right quadrant
            sortedCorners[2] = point;
        } else {                                                 // Bottom left quadrant
            sortedCorners[3] = point;
        }
    }

    cv::Mat transform = cv::getPerspectiveTransform(sortedCorners, destinationPoints, cv::DECOMP_LU);

    cv::warpPerspective(image, image, transform, cv::Size(1500, 1500/aspectRatio));

    return image;
}

struct processImageResults {
    cv::Mat image;
    Contour2f corners;
};

processImageResults processImage(cv::Mat image) {
    // Convert to grayscale
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_BGR2GRAY);

    // Blur
    cv::GaussianBlur(image, image, cv::Size(cv::Point2i(15, 15)), 0.0);

    cv::threshold(image, image, threshold, 255, cv::ThresholdTypes::THRESH_BINARY);
    erodeThenDilate(image, image, 30);

    //image = transformImage(image);

    vector<Contour> contours;
    cv::findContours(image, contours, cv::RETR_CCOMP, cv::CHAIN_APPROX_SIMPLE);

    Contour2f corners = findCornersOfTarget(contours);

    // Covert back to rgb so we can draw the contours in a different color
    cv::cvtColor(image, image, cv::ColorConversionCodes::COLOR_GRAY2BGR);

    // Draw all the contours onto the image
    for (auto contour : contours) {
        for (int i = 0; i < contour.size(); i++) {
            cv::line(image, contour[i], contour[(i+1) % contour.size()], cv::Scalar(0, 255, 0), 5);
        }
    }

    // Draw the borders of the target onto the image
    for (auto contour : corners) {
        for (int i = 0; i < corners.size(); i++) {
            cv::line(image, corners[i], corners[(i+1) % corners.size()], cv::Scalar(255, 255, 0), 50);
        }
    }

    return {image, corners};
}

void updateImages(int, void*) {
    processImageResults results = processImage(sourceImage);
    cv::imshow("Processed image", results.image);

    cv::Mat transformedImage = transformImage(sourceImage, results.corners);
    // Convert to grayscale
    cv::cvtColor(transformedImage, transformedImage, cv::ColorConversionCodes::COLOR_BGR2GRAY);
    //cv::GaussianBlur(transformedImage, transformedImage, cv::Size(15, 15), 0.0);
    //cv::threshold(transformedImage, transformedImage, 165, 255, cv::ThresholdTypes::THRESH_BINARY_INV);
    
    erodeThenDilate(transformedImage, transformedImage, 10);
    //cv::Canny(transformedImage, transformedImage, 50, 150);
    
    cv::SimpleBlobDetector::Params params;
    params.minThreshold = 50;
    params.maxThreshold = 230;
    params.filterByConvexity = false;
    params.filterByCircularity = false;

    cv::Ptr<cv::SimpleBlobDetector> blobDetector = cv::SimpleBlobDetector::create(params);

    std::vector<cv::KeyPoint> keypoints;
    blobDetector->detect(transformedImage, keypoints);
    cv::drawKeypoints(transformedImage, keypoints, transformedImage, cv::Scalar(0,0,255), cv::DrawMatchesFlags::DRAW_RICH_KEYPOINTS );
    
    // vector<Contour> contours;
    // cv::findContours(transformedImage, contours, cv::RETR_CCOMP, cv::CHAIN_APPROX_SIMPLE);
    // cv::drawContours(transformedImage, contours, -1, cv::Scalar(255, 255, 0), 5);

    cv::imshow("Transformed image", transformedImage);
}

int main(int argc, char** argv) {
    // Check if image path is provided as a command line argument
    if (argc != 2) {
        std::cout << "Usage: ./DisplayImage <Image_Path>" << std::endl;
        return -1;
    }

    // Read the image file
    sourceImage = cv::imread(argv[1]);

    // Check for failure
    if (sourceImage.empty()) {
        std::cout << "Could not open or find the image" << std::endl;
        return -1;
    }

    cv::namedWindow("Source image", cv::WINDOW_NORMAL);
    cv::imshow("Source image", sourceImage);
    contrast(sourceImage, sourceImage, 1.7, -75);
    // cv::namedWindow("Contrasted image", cv::WINDOW_NORMAL);
    // cv::imshow("Contrasted image", sourceImage);

    // Create a window
    cv::namedWindow("Processed image", cv::WINDOW_NORMAL);
    cv::createTrackbar("Threshold", "Processed image", &threshold, 255, updateImages);
    
    // Create a window for the transformed image
    cv::namedWindow("Transformed image", cv::WINDOW_NORMAL);

    updateImages(0, nullptr);

    // Wait for any keystroke in the window
    cv::waitKey(0);

    return 0;
}