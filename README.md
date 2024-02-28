# Rust AWS Lambda Function for S3 Object Count
By Daniel Medina

## Overview

This service consists of an AWS Lambda function implemented in Rust that counts the number of objects in a specified Amazon S3 bucket. The primary use case of this function is to monitor and report the number of photo image files stored within the bucket, providing insights into data accumulation over time.

## Rust Lambda Functionality

The Lambda function, written in Rust, leverages the `aws-sdk-s3` client to interact with Amazon S3 services. The core logic of the function follows these steps:

1. The function is triggered manually or by another AWS service.
2. It creates an S3 client instance using default configurations.
3. The function calls `list_objects_v2` on the specified bucket ('photo-rag') to retrieve all objects.
4. It counts the number of items returned by the S3 service.
5. The function constructs a JSON response containing the total count of objects in the bucket.

The Lambda function requires the following permissions:
- `s3:ListBucket` on the specific bucket to count objects.
- CloudWatch Logs permissions for logging purposes (`logs:CreateLogGroup`, `logs:CreateLogStream`, `logs:PutLogEvents`).

## Database Integration

Although this specific Lambda function does not directly interact with a database, it counts objects within an S3 bucket typically used to store photo image files. This data can be utilized to monitor storage needs, inform user quotas, or track photo uploads over time. If needed, the function can be extended to record count results in a database for historical tracking and analysis.

## Service Implementation

To implement and deploy this service:

1. Ensure you have AWS CLI installed and configured with the necessary permissions.
2. Compile the Rust project into a binary compatible with the Lambda execution environment.
3. Create a new Lambda function in the AWS Console or update an existing one, setting the handler to the compiled binary.
4. Assign the execution role to the Lambda function with the necessary S3 and CloudWatch Logs permissions.
5. (Optional) Set up a trigger for the Lambda function, such as an S3 event or a scheduled event via Amazon EventBridge.
6. Test the function manually through the AWS Console or programmatically via AWS SDKs to ensure proper execution and permissions.

For manual testing, invoke the function without any specific input, as the function does not require event data to count the objects in the predefined S3 bucket.

