variable "aws_access_key_id" {}
variable "aws_secret_access_key" {}
variable "aws_session_token" {}
variable "aws_forecast_role" {}
variable "ibmcloud_api_key" {}
variable "ibmcloud_washington_namespace" {}
variable "ibmcloud_washington_region" {}
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
  secret_key = var.aws_secret_access_key
  token      = var.aws_session_token
}

provider "ibm" {
  ibmcloud_api_key = var.ibmcloud_api_key
  region           = var.ibmcloud_washington_region
}

resource "aws_s3_bucket" "stock_forecast_data" {
  bucket = "stock-forecast-data"
  acl    = "private"
}

resource "local_file" "dotenv" {
  sensitive_content = templatefile("${path.module}/.env.example", {
    aws_access_key_id                    = var.aws_access_key_id,
    aws_secret_access_key                = var.aws_secret_access_key,
    aws_session_token                    = var.aws_session_token,
    aws_forecast_bucket                  = aws_s3_bucket.stock_forecast_data.bucket,
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


// Rust Functions

data "external" "rust_functions" {
  depends_on = [local_file.dotenv]

  program = ["${path.module}/build_rust_functions.sh"]
}

resource "null_resource" "fetch_prices_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.rust_functions.result.id
    executable = filesha256(data.external.rust_functions.result.fetch_prices)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' fetch_prices_rs '${var.ibmcloud_washington_region}' '${var.ibmcloud_washington_namespace}' '${data.external.rust_functions.result.fetch_prices}'"
  }
}

resource "null_resource" "forecast_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.rust_functions.result.id
    executable = filesha256(data.external.rust_functions.result.forecast)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' forecast_rs '${var.ibmcloud_washington_region}' '${var.ibmcloud_washington_namespace}' '${data.external.rust_functions.result.forecast}'"
  }
}

resource "null_resource" "process_result_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.rust_functions.result.id
    executable = filesha256(data.external.rust_functions.result.process_result)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' process_result_rs '${var.ibmcloud_washington_region}' '${var.ibmcloud_washington_namespace}' '${data.external.rust_functions.result.process_result}'"
  }
}

resource "null_resource" "create_chart_rs" {
  depends_on = [local_file.ibmcloud_api_key]

  triggers = {
    id = data.external.rust_functions.result.id
    executable = filesha256(data.external.rust_functions.result.create_chart)
  }

  provisioner "local-exec" {
    command = "'${path.module}/deploy_rust_function.sh' create_chart_rs '${var.ibmcloud_washington_region}' '${var.ibmcloud_washington_namespace}' '${data.external.rust_functions.result.create_chart}'"
  }
}


// JS Functions

data "external" "js_functions" {
  depends_on = [local_file.dotenv]

  program = ["${path.module}/build_js_functions.sh"]
}

resource "ibm_function_action" "fetch_prices_js" {
  name      = "fetch_prices_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = filebase64(data.external.js_functions.result.fetch_prices)
  }

  limits {
    timeout = 600000
    memory  = 128
  }

  user_defined_annotations = "[{\"key\":\"web-export\",\"value\":true}]"
}

resource "ibm_function_action" "forecast_js" {
  name      = "forecast_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = filebase64(data.external.js_functions.result.forecast)
  }

  limits {
    timeout = 600000
    memory  = 128
  }

  user_defined_annotations = "[{\"key\":\"web-export\",\"value\":true}]"
}

resource "ibm_function_action" "process_result_js" {
  name      = "process_result_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = filebase64(data.external.js_functions.result.process_result)
  }

  limits {
    timeout = 600000
    memory  = 128
  }

  user_defined_annotations = "[{\"key\":\"web-export\",\"value\":true}]"
}

resource "ibm_function_action" "create_chart_js" {
  name      = "create_chart_js"
  namespace = var.ibmcloud_washington_namespace

  exec {
    kind = "nodejs:12"
    code = filebase64(data.external.js_functions.result.create_chart)
  }

  limits {
    timeout = 600000
    memory  = 128
  }

  # publish = true

  user_defined_annotations = "[{\"key\":\"web-export\",\"value\":true}]"
}

