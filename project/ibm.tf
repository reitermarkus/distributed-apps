variable "ibmcloud_api_key" {}
variable "ibmcloud_washington_namespace" {}
variable "alphavantage_api_key" {}
variable "ibm_object_storage_endpoint_url" {}
variable "ibm_object_storage_api_key" {}
variable "ibm_object_storage_bucket_name" {}

terraform {
  required_providers {
    ibm = {
      source  = "IBM-Cloud/ibm"
      version = "~> 1.18.0"
    }
  }
}

provider "ibm" {
  ibmcloud_api_key = var.ibmcloud_api_key
  region           = "us-east"
}

resource "local_file" "dotenv" {
  sensitive_content = templatefile("${path.module}/.env.example", {
    alphavantage_api_key            = var.alphavantage_api_key,
    ibm_object_storage_endpoint_url = var.ibm_object_storage_endpoint_url,
    ibm_object_storage_api_key      = var.ibm_object_storage_api_key,
    ibm_object_storage_bucket_name  = var.ibm_object_storage_bucket_name,
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

