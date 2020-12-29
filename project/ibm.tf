variable "aws_access_key_id" {}
variable "aws_secret_key" {}
variable "aws_session_token" {}
variable "aws_forecast_dataset" {}
variable "aws_forecast_role" {}
variable "ibmcloud_api_key" {}
variable "ibmcloud_washington_namespace" {}
variable "alphavantage_api_key" {}
variable "ibm_object_storage_endpoint_url" {}
variable "ibm_object_storage_api_key" {}
variable "ibm_object_storage_bucket_name" {}
variable "ibm_object_storage_access_key_id" {}
variable "ibm_object_storage_secret_access_key" {}

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.0"
    }

    ibm = {
      source  = "IBM-Cloud/ibm"
      version = "~> 1.18.0"
    }
  }
}

provider "aws" {
  region     = "us-east-1"
  access_key = var.aws_access_key_id
  secret_key = var.aws_secret_key
  token      = var.aws_session_token
}

provider "ibm" {
  ibmcloud_api_key = var.ibmcloud_api_key
  region           = "us-east"
}

resource "aws_s3_bucket" "stock_forecast_data" {
  bucket = "stock-forecast-data"
  acl    = "private"
}

resource "local_file" "dotenv" {
  sensitive_content = templatefile("${path.module}/.env.example", {
    aws_secret_key                       = var.aws_secret_key,
    aws_access_key_id                    = var.aws_access_key_id,
    aws_session_token                    = var.aws_session_token,
    aws_forecast_bucket                  = aws_s3_bucket.stock_forecast_data.bucket,
    aws_forecast_dataset                 = var.aws_forecast_dataset,
    aws_forecast_role                    = var.aws_forecast_role,
    alphavantage_api_key                 = var.alphavantage_api_key,
    ibm_object_storage_endpoint_url      = var.ibm_object_storage_endpoint_url,
    ibm_object_storage_bucket_name       = var.ibm_object_storage_bucket_name,
    ibm_object_storage_api_key           = var.ibm_object_storage_api_key,
    ibm_object_storage_access_key_id     = var.ibm_object_storage_access_key_id,
    ibm_object_storage_secret_access_key = var.ibm_object_storage_secret_access_key,
  })
  filename = "${path.module}/.env"
}

resource "local_file" "ibmcloud_api_key" {
  sensitive_content = var.ibmcloud_api_key
  filename = "${path.module}/ibmcloud_api_key.txt"
}

resource "random_id" "rust_functions" {
  byte_length = 8
}

data "external" "fetch_prices_rs_zip" {
  depends_on = [local_file.dotenv]

  program = ["${path.module}/build_rust_function.sh", "fetch_prices"]
}

resource "null_resource" "fetch_prices_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.fetch_prices_rs_zip.result.id
    executable = filesha256(data.external.fetch_prices_rs_zip.result.filename)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' fetch_prices_rs '${var.ibmcloud_washington_namespace}' '${data.external.fetch_prices_rs_zip.result.filename}'"
  }
}

data "external" "forecast_rs_zip" {
  depends_on = [local_file.dotenv]

  program = ["${path.module}/build_rust_function.sh", "forecast"]
}

resource "null_resource" "forecast_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.forecast_rs_zip.result.id
    executable = filesha256(data.external.forecast_rs_zip.result.filename)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' forecast_rs '${var.ibmcloud_washington_namespace}' '${data.external.forecast_rs_zip.result.filename}'"
  }
}

resource "ibm_function_action" "fetch_prices_js" {
  depends_on = [local_file.dotenv]

  name      = "fetch_prices_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = file("dist/fetch_prices.bundle.js")
  }

  limits {
    timeout = 10000
    memory  = 128
  }
}

