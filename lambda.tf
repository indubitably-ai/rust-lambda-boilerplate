# resource "aws_lambda_function" "lambda_function" {
#   function_name = "rust-aws-lambda"

#   # Ensure that the filename attribute points to the correct path of the zip file
#   # The null_resource.lambda_build must run before this to create the zip file
#   filename      = "${path.module}/target/lambda/release/rust-aws-lambda.zip"
#   role          = aws_iam_role.lambda_role.arn
#   handler       = "bootstrap"
#   runtime       = "provided.al2"

#   tags = {
#     Name = "rust-lambda-function"
#   }

#   # Add a dependency on null_resource.lambda_build to ensure the zip file is created first
#   depends_on = [null_resource.lambda_build]
# }

# resource "aws_iam_role" "lambda_role" {
#   name = "lambda-role"

#   assume_role_policy = jsonencode({
#     Version = "2012-10-17",
#     Statement = [
#       {
#         Action = "sts:AssumeRole",
#         Principal = {
#           Service = "lambda.amazonaws.com"
#         },
#         Effect = "Allow",
#         Sid    = ""
#       }
#     ]
#   })
# }

# resource "aws_lambda_permission" "permission" {
#   action        = "lambda:InvokeFunction"
#   function_name = aws_lambda_function.lambda_function.function_name
#   principal     = "apigateway.amazonaws.com"

#   source_arn = "${aws_api_gateway_rest_api.api.execution_arn}/*/*"
# }

# # Archive lambda function using cargo lambda build
# resource "null_resource" "lambda_build" {
#   triggers = {
#     updated_at = timestamp()
#   }

#   provisioner "local-exec" {
#     command = "cargo lambda build --release --arm64 --output-format zip"
#     working_dir = "${path.module}/src"
#   }

#   # Ensure the zip file is removed when the resource is destroyed
#   provisioner "local-exec" {
#     when    = "destroy"
#     command = "rm -f ${path.module}/target/lambda/release/rust-aws-lambda.zip"
#   }

#   depends_on = [
#     aws_iam_role.lambda_role
#   ]
# }


# # The archive_file data block is no longer needed since cargo lambda build creates the zip file
